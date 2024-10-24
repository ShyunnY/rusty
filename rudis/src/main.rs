use std::{collections::HashMap, time::Duration};

use mini_redis::{Command, Connection, Frame};
use tokio::{
    net::{TcpListener, TcpStream},
    time::sleep,
};

/// 我们将 `.await` 理解为就是: **一步走两步判读**
/// * 一步走: 推动执行一个 Future 的 poll()
/// * 两个判断: 如果 Future 返回了 Poll:Ready(T), 就执行完毕. 如果 Future 返回了 Poll::Pending, 就不阻塞并向下执行(类似同步模式)
///
/// 在 tokio 中, main函数是作为一个大的 Future, 内部包含了多个小 Future
/// 如果我们不使用类似于 join! 的方式, 组合多个 future, 最终也是类似于 **同步的方式去执行**
/// 所以我们应该需要类似于 `tokio::spawn()` 的方式创建一个可调度的任务
#[tokio::main]
async fn main() {
    let tcp_listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (tcp_stream, _) = tcp_listener.accept().await.unwrap();

        // 一个 Tokio 任务是一个异步的绿色线程, 它们通过 `tokio::spawn` 进行创建
        // 该函数会返回一个 `JoinHandle` 类型的句柄, 调用者可以使用该句柄跟创建的任务进行交互
        // 任务是调度器管理的执行单元. spawn生成的任务会首先提交给'调度器', 然后由它负责调度执行.
        // 需要注意的是, 执行任务的线程未必是创建任务的线程, 任务'完全有可能运行在另一个不同的线程'上, 而且任务在生成后, 它还可能会在线程间被移动.
        // 类似于 Golang 的协程 :)
        tokio::spawn(async {
            process(tcp_stream).await;
        });
    }
}

async fn process(socket: TcpStream) {
    // Connection 对 redis 的读写进行了封装
    // Frame(数据帧 = redis命令 + 数据)
    let mut connection = Connection::new(socket);
    let mut db: HashMap<String, Vec<u8>> = HashMap::new();

    // 我们需要使用循环 (while) 的方式在同一个客户端连接中处理多次连续的请求
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response: Frame = match Command::from_frame(frame).unwrap() {
            Command::Set(set) => {
                db.insert(set.key().to_string(), set.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Command::Get(get) => {
                if let Some(val) = db.get(get.key()) {
                    Frame::Bulk(val.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimmplementd command: {:?}", cmd),
        };

        // reply
        connection.write_frame(&response).await.unwrap();
    }
}
