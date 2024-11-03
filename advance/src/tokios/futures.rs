use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Poll, Waker},
    thread,
    time::{Duration, Instant},
};

use futures::future::poll_fn;
use tokio::{
    net::TcpListener,
    sync::{
        mpsc::{Receiver, Sender},
        oneshot,
    },
};
use tokio::{sync::mpsc, time::sleep};

#[allow(dead_code)]
pub async fn hello() {
    // 和其它语言不同, Rust 中的 Future 不代表一个发生在后台的计算, 而是 Future 就代表了计算本身
    // 因此 Future 的所有者 '有责任去推进该计算过程的执行'。例如通过 Future::poll 函数。听上去好像还挺复杂？但是大家不必担心，因为这些都在 Tokio 中帮你自动完成了 :)

    // select().await;
    select_advance().await;
}

#[allow(dead_code)]
async fn select_advance() {
    // select 中的借用
    // 当在 Tokio 中生成( spawn )任务时, 其 async 语句块必须拥有其中数据的 "所有权"
    // 而 select! 并没有这个限制, 它的每个分支表达式可以直接借用数据, 然后进行并发操作。
    // 只要遵循 Rust 的借用规则, 多个分支表达式可以 "不可变的借用同一个数据", 或者 "在一个表达式可变的借用某个数据"
    //
    /*
       这里其实有一个很有趣的题外话，由于 TCP 连接过程是在模式中发生的
       因此当某一个连接过程失败后，它通过 ? 返回的 Err 类型并无法匹配 Ok，因此另一个分支会继续被执行，继续连接。
       如果你把连接过程放在了结果处理中，那连接失败会直接从 race 函数中返回，而不是继续执行另一个分支中的连接！
    */
    {
        // 借用规则在分支表达式和结果处理中存在很大的不同.
        // 例如上面代码中: 我们在两个分支表达式中分别对 data 做了不可变借用, 这当然 ok
        // 但是若是两次可变借用, 那编译器会立即进行报错
        // 原因就在于：select!会保证只有一个分支的结果处理会被运行, 然后在运行结束后, 另一个分支会被直接丢弃
        // 所以我们在结果中使用, 是能够保证只有一个分支的结果被执行!!!
        // 如果我们在表达式中使用, 是有可能出现多个分支表达式被执行(因为分支的模式不匹配, 会往下执行...)
        async fn foo(data: &str) {
            // 我们可以在多个分支的表达式中进行: 不可变借用
            // 但是我们无法在多个分支的表达式中进行: 可变借用
            tokio::select! {
                _ = async{
                    // data = ""; // 这将会报错
                    println!("branchA: data got {}",data);
                } => {}
                _ = async{
                    println!("branchB: data got {}",data);
                } => {}
            };
        }

        foo("borrow").await;
    }

    // 2. 循环
    // select! 经常与 loop 循环配合使用
    /*
       在循环中使用 select! 最大的不同就是: 当某一个分支执行完成后, select! 会继续循环等待并执行下一个分支
       直到所有分支最终都完成, 最终匹配到 else 分支, 然后通过 break 跳出循环

       老生常谈的一句话: select! 中哪个分支先被执行是无法确定的, 因此不要依赖于分支执行的顺序！
    */
    {
        async fn foo() {
            // 我们通过 loop 重复执行 select 中的分支
            // 当 select! 中所有的分支都模式不匹配了 -- 执行完毕了
            // 此时我们执行 else 跳出循环
            loop {
                tokio::select! {
                    Some(_) = async{
                        Option::<i32>::None
                    } => {}
                    Some(_) = async{
                        Option::<i32>::None
                    } => {}
                    else => {
                        break;
                    }
                }
            }
        }
    }

    // TODO: 3. 恢复异步执行

    // TODO: 4. 修改一个分支

    // tokio::spawn 和 tokio::select! 的区别
    // + tokio::spawn 函数会启动新的任务来运行一个异步操作,
    //   每个任务都是一个独立的对象可以单独被 Tokio 调度运行, 因此两个不同的任务的调度都是 '独立进行的'
    //   甚至于它们可能会运行在'两个不同的操作系统线程'上。鉴于此，生成的任务和生成的线程有一个相同的限制: 不允许对外部环境中的值进行借用。
    // + tokio::select! 宏就不一样了, 它在同一个任务中并发运行所有的分支。
    //   正是因为这样, 在同一个任务中, 这些分支无法被同时运行。 select! 宏在单个任务中实现了'多路复用'的功能
}

#[allow(dead_code)]
async fn select() {
    // 1. select! 入门
    let (mut tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();
    tokio::spawn(async {
        tokio::select! {
            _ = sleep(Duration::from_millis(10)) => {
                println!("ok~");
                let _ = tx1.send("one");

            }
            _ = tx1.closed() => {
                // 收到了发送端发来的关闭信号, 此时第一个分支任务会被取消, 任务不再执行
                println!("do_stuff_async() completed first");
            }
        }
    });

    tokio::spawn(async {
        let _ = tx2.send("two");
    });

    // select! 在从两个通道阻塞等待接收消息时, rx1 和 rx2 都有可能被先打印出来
    // 需要注意: 任何一个 select 分支完成后, 都会继续执行后面的代码, 没被执行的分支会被丢弃 (dropped)
    //
    // select! 一开始会随机选择一个分支进行 poll
    tokio::select! {
        // 只会执行一个, 没有执行的分支将会被 drop
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
    }
    // 任何一个 select 分支结束后，都会继续执行接下来的代码

    // 2. select! 语法
    // 语法:  <模式> = <async 表达式> => <结果处理>
    // 当 select 宏开始执行后, 所有的分支会开始并发的执行(注意: 是并发的执行!).
    // 当任何一个表达式完成时, 会将结果跟模式进行匹配. 若匹配成功, 则剩下的表达式会被释放(Drop, 任务停止)
    // 最常用的模式就是用变量名去匹配表达式返回的值, 然后该变量就可以在结果处理环节使用
    //
    // 如果 '当前的模式不能匹配, 剩余的 async 表达式将继续 **并发** 的执行', 直到下一个完成
    // 由于 select! 使用的是一个 async 表达式，因此我们可以定义一些更复杂的计算
    {
        // 例如我们可以在 select 中接受 Tcp 链接

        let (tx, rx) = oneshot::channel();
        tokio::spawn(async {
            tx.send(()).unwrap();
        });
        let tcp_listener = TcpListener::bind("127.0.0.1:3308").await.unwrap();

        // 分支中接收连接的循环会一直运行, 直到遇到错误才停止
        // 或者当 rx 中有值时, 也会停止
        // _ 表示我们并不关心这个值
        tokio::select! {
            _ = async{
                loop {
                    let (_,_ ) = tcp_listener.accept().await.unwrap();
                }
            } => {}

            _ = rx => {
                println!("terminating accept loop")
            }
        }
    }

    // 3. select! 的返回值
    // select! 的所有分支 "必须返回一样的类型" , 否则编译器会报错！
    {
        async fn foo() -> String {
            "foo".to_string()
        }

        async fn bar() -> String {
            "bar".to_string()
        }

        // 所有的分支必须返回一样的类型返回值, 这其实就类似于 match 分支
        let ret = tokio::select! {
            foo_ret = foo() => foo_ret,
            bar_ret = bar() => bar_ret,
        };
        println!("ret got: {}", ret);
    }

    // 4. select! 的错误处理
    // 在 Rust 中使用 ? 可以对错误进行传播, 但是在 select! 中
    // ? 如何工作取决于它是在分支中的 async 表达式使用还是在结果处理的代码中使用:
    // + 在分支中 async 表达式使用会将该表达式的结果变成一个 Result
    // + 在结果处理中使用, 会将错误 "直接传播到 select! 之外"
    {
        async fn foo() -> Result<(), String> {
            Err("()".to_string())
        }

        async fn bar() -> Result<(), String> {
            tokio::select! {
                res = foo() => {
                    // **错误直接传递到 select! 外部的函数上**
                    let r = res?;
                    Ok(r)
                }
            }
        }
    }

    // 5. 模式匹配
    // 既然是模式匹配, 我们需要再来回忆下 select! 的分支语法形式:
    // " <模式> = <async 表达式> => <结果处理> ""
    // 迄今为止, 我们只用了变量绑定的模式. 事实上, 任何 Rust 模式都可以在此处使用
    {
        let (_tx, mut rx): (Sender<i32>, Receiver<i32>) = mpsc::channel(1);

        // 我们可以看到: 我们还可以进行 Some 的模式匹配
        // 同时我们使用 else 作为最后的 "匹配守卫"
        // else 代表: 当上面的所有分支都不匹配, 则最后走到 else 分支上
        tokio::select! {
            Some(v) = rx.recv() => {
                println!("receive some value: {v}");
            }
            else => {
                println!("no value!")
            }
        };
    }
}

#[allow(dead_code)]
async fn my_futures() {
    let when = Instant::now() + Duration::from_secs(2);
    let mut delay = Some(Delay {
        when: when,
        waker: None,
    });

    poll_fn(move |cx| {
        let mut delay = delay.take().unwrap();

        // 在这里 poll 了一次
        let ret = Pin::new(&mut delay).poll(cx);

        assert!(ret.is_pending());
        tokio::spawn(async move {
            // 在这里又 poll 了一次, 所以我们应该保存新的 Waker
            // 并且检查是否启动了线程, 避免重复启动线程
            delay.await;
        });

        Poll::Ready(())
    })
    .await;

    thread::sleep(Duration::from_secs(5));
}

/// 当实现一个 Future 时, 很关键的一点就是要 **假设每次 `poll()` 调用都会应用到一个不同的 [Waker] 实例上**
///
/// 因此 poll 函数必须要使用一个新的 waker 去更新替代之前的 waker
struct Delay {
    when: Instant,
    waker: Option<Arc<Mutex<Waker>>>,
}

/// 做一个小小的总结:
/// * 在 Rust 中，async 是**惰性**的，直到执行器 `poll()` 它们时, 才会开始执行
/// * Waker 是 Future 被执行的关键, 它可以链接起 **Future任务** 和 **执行器**
/// * 当资源没有准备时, 会返回一个 Poll::Pending. 当资源准备好时, 会返回一个 Poll::Ready(T)
/// * 当资源准备好时, 会通过 waker.wake 发出通知告诉执行器: i'm OK!
/// * 执行器会收到通知, 然后调度该任务继续执行, 此时由于资源已经准备好, 因此任务可以顺利往前推进了
impl Future for Delay {
    type Output = &'static str;

    /// 为什么需要判断 Waker 是否与当前 Future 中保存的是同一个?
    ///
    /// 1. 避免不必要的唤醒: 如果 Future 在多次 poll 调用之间保存了 Waker, 那么在每次 poll 调用时, 都需要**检查当前 Context 中的 Waker 是否与之前保存的 Waker 相同**。
    /// 如果不同，说明执行器可能已经发生了变化（例如，任务被迁移到另一个线程或执行器），此时需要更新保存的 Waker。
    /// 2. 确保正确的唤醒: 如果 Future 在多个不同的执行器之间迁移, Waker 可能会发生变化。为了确保在 Future 准备好时能够正确地通知当前的执行器, 需要更新保存的 Waker。
    /// 因为每个执行器都有自己的 Waker 实现
    ///
    /// 因为一个 Future 在 `.await` 时可能会切换到另外一个任务中, 此时再次 poll 的 Waker 信息可能不一样, 旧的 Waker 不再与当前的执行器关联。 所以我们需要更新
    ///
    /// **最主要还是: Rust 的异步模型允许一个 Future 在执行过程中可以 `跨任务迁移`, 每个执行器都有自己的 Waker 实现, 如果 Future 移动到其他任务, 调用 poll 的 Waker 就会更新**
    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if let Some(waker) = &self.waker {
            let mut waker = waker.lock().unwrap();

            // 由于 Future poll() 也许会应用到在两个不同的 Waker 上(其实就是 Future 被转移到另外一个任务中)
            // 然后存储的 waker 被该任务进行了更新(新的执行器使用了新的 Context, 新的 Context 的 Waker 和原来的不一样!)
            //
            // 所以我们需要检测当前 poll() 的 Waker 是否与已经保存的 Waker 是同一个, 如果不是就更新
            // 因为我们需要唤醒最新的 Waker
            if !waker.will_wake(cx.waker()) {
                println!("update...");
                waker.clone_from(cx.waker());
            }
        } else {
            let when = self.when;
            let waker = Arc::new(Mutex::new(cx.waker().clone()));
            self.as_mut().get_mut().waker = Some(waker.clone());

            // 第一次调用 'poll', 生成新线程
            thread::spawn(move || {
                let now = Instant::now();
                if now < when {
                    thread::sleep(when - now);
                }

                // 通知执行器再次 poll
                let waker = waker.lock().unwrap();
                waker.wake_by_ref(); // wake_by_ref: 唤醒与当前任务相关的 waker, 并且不消耗 waker(因为是引用)
            });
        }

        if Instant::now() >= self.when {
            // 时间到了, 执行完毕
            println!("Hello, world!");
            Poll::Ready("done")
        } else {
            // **在返回 Poll::Pending 时, 我们一般需要保证 wake 是能够正常调用的. 如果没有 wake 将会发生不为人知的 bug...**
            // wake 用于通知调度器在未来某个时间段内再次执行当前 Future 的 poll()
            // 流程: '执行 -> 通知再调度 -> 执行'
            //
            // 如果忘记调用 waker, 那等待我们的将是深渊: 该任务将被永远的挂起, 无法再执行

            Poll::Pending
        }
    }
}
