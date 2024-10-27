use bytes::Bytes;
use log::{debug, info};
use mini_redis::{Command, Connection, Frame};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::net::{TcpListener, TcpStream};

/// Tokio 提供的异步锁只应该在跨多个 `.await` 调用时使用
/// 在 `.await` 执行期间, 任务可能会在线程间转移. 理解了这个很多时候就明白错误了
///
/// Example:
///
/// 如果我们在 `async` 中跨 `.await` 使用了 Mutex, 此时会可能导致死锁的问题
/// 因为 `.await` 期间如果调度了另外一个 Future, 并且该 Future 也需要获取锁, 此时就导致死锁啦
///
/// 或者我们可以使用 Tokio 提供的锁.
/// 最大的优点就是：它可以在 `.await` 执行期间被持有，而且不会有任何问题。但是代价就是，这种异步锁的性能开销会更高
type Database = Arc<Vec<Mutex<HashMap<String, Bytes>>>>;

/// 我们将 `.await` 理解为就是: **一步走两步判读**
/// * 一步走: 推动执行一个 Future 的 poll()
/// * 两个判断: 如果 Future 返回了 Poll:Ready(T), 就执行完毕. 如果 Future 返回了 Poll::Pending, 就调度其他 Future
///           **如果我们希望在原 async 中继续向下执行, 就需要等待 `.await` 返回一个 Poll::Ready(T)**
///
/// `.await` 就是执行一次 poll 方法, 如果完成就是ready, 没完成就pending. 当 pending 时会调度其他的任务执行
///
/// 在 tokio 中, main函数是作为一个大的 Future, 内部包含了多个小 Future
/// 如果我们不使用类似于 join! 的方式, 组合多个 future, 最终也是类似于 **同步的方式去执行**
/// 所以我们应该需要类似于 `tokio::spawn()` 的方式创建一个可调度的任务
#[tokio::main]
async fn main() {
    // init logger
    env_logger::init();
    info!("rudis is starting");

    let tcp_listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let db = new_shared_db(3);

    loop {
        let (tcp_stream, _) = tcp_listener.accept().await.unwrap();

        // 一个 Tokio 任务是一个异步的绿色线程, 它们通过 `tokio::spawn` 进行创建
        // 该函数会返回一个 `JoinHandle` 类型的句柄, 调用者可以使用该句柄跟创建的任务进行交互
        // 任务是调度器管理的执行单元. spawn生成的任务会首先提交给'调度器', 然后由它负责调度执行.
        // 需要注意的是, 执行任务的线程未必是创建任务的线程, 任务'完全有可能运行在另一个不同的线程'上, 而且任务在生成后, 它还可能会在线程间被移动.
        // 类似于启动一个 "Golang的协程" :)

        let db = db.clone();
        tokio::spawn(async {
            process(tcp_stream, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Database) {
    // Connection 对 redis 的读写进行了封装
    // Frame(数据帧 = redis命令 + 数据)
    let mut connection = Connection::new(socket);

    // 我们需要使用循环 (while) 的方式在同一个客户端连接中处理多次连续的请求
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response: Frame = match Command::from_frame(frame).unwrap() {
            Command::Set(set) => {
                let key = set.key().to_string();
                let val = set.value().clone();

                debug!("Set command: key={}, val={:?}", &key, &val);

                let mut db = db[key.len() % db.len()].lock().unwrap();
                db.insert(key, val);
                Frame::Simple("OK".to_string())
            }
            Command::Get(get) => {
                let key = get.key();
                let db = db[key.len() % db.len()].lock().unwrap();

                if let Some(val) = db.get(key) {
                    debug!("Get command: key={},val={:?}", key, val);

                    Frame::Bulk(val.clone().into())
                } else {
                    info!("Get command not found val by key={}", key);

                    Frame::Null
                }
            }
            cmd => panic!("unimmplementd command: {:?}", cmd),
        };

        // reply
        connection.write_frame(&response).await.unwrap();
    }
}

fn new_shared_db(mut shards_num: usize) -> Database {
    if shards_num == 0 {
        shards_num = 1;
    }

    let mut db = Vec::with_capacity(3);
    for _ in 0..shards_num {
        db.push(Mutex::new(HashMap::new()));
    }

    Arc::new(db)
}
