use std::time::Duration;

use tokio::{
    sync::mpsc,
    time::{self},
};

pub mod futures;
pub mod streams;

#[allow(dead_code)]
pub async fn sync_and_async() {
    // 同步与异步共存...

    // 1. [tokio::main]
    // 在 Rust 中， main 函数不能是异步的
    // #[tokio::main] 该宏仅仅是提供语法糖, 目的是让大家可以更简单、更一致的去写异步代码
    // 它会将你写下的 async fn main 函数替换为：
    // tokio::runtime::Builder::new_multi_thread().build().unwrap().block_on(async{
    //     /* main code */
    // })
    // 所以它实际上是将整个 main 函数转变为一个入口的 Future , 后续我们编写的代码都是基于这个 Future 派生出的子 future
    // 并且我们需要注意： new_multi_thread 这将启动多个线程, 我们会将任务分发到多个线程上面执行
    // 宏观上看类似于 golang 的协程, N:M = 异步任务:线程 的方式映射

    // 2. new_multi_thread 和 new_current_thread 的知识...
    // 我们还使用了 current_thread 模式的运行时. 这个可不常见, 原因是异步程序往往要利用多线程的威力来实现更高的吞吐性能,
    // 相对应的模式就是 'multi_thread', 该模式会生成多个运行在后台的线程, 它们可以高效的实现多个任务的同时并行处理
    // 但是对于我们的使用场景来说, 在同一时间点只需要做 '一件事', 无需并行处理，多个线程并不能帮助到任何事情
    // 因此 current_thread 此时成为了最佳的选择
    // 总结:
    // new_multi_thread: 多线程对应多个任务, tokio 后台会帮助我们启动多个线程来调度
    // new_current_thread: 单线程对应多个任务, tokio 会直接在当前的 main 线程上处理任务的调度
    /*
       由于 current_thread 运行时并不生成新的线程, 只是运行在已有的主线程上. 因此只有当 block_on 被调用后, 该'运行时才能执行相应的操作'
       一旦 block_on 返回, 那运行时上所有生成的任务将'再次冻结', 直到 'block_on 的再次调用'
    */

    // 3. 其他方法
    // runtime.spawn
    // 可以通过 Runtime 的 spawn 方法来创建一个基于该运行时的后台任务:
    // 类比为 thread::spawn(||{}) 一样, 只不过一个是用于创建线程, 一个是用于创建 Future
    {
        async fn foo(secs: usize) {
            time::sleep(Duration::from_secs(secs as u64)).await;
            println!("current task sleep {}s", secs);
        }

        async fn exec() {
            let mut handlers = vec![];
            for secs in 1..5 {
                let handler = tokio::spawn(async move {
                    foo(secs).await;
                });
                handlers.push(handler);
            }
        }

        exec().await;
        time::sleep(Duration::from_secs(5)).await;
    }
}

#[allow(dead_code)]
pub async fn graceful_shutdown() {
    // 优雅停机
    // 要让一个异步应用优雅的关闭往往需要做到 3 点：
    // 1.找出合适的关闭时机
    // 2.通知程序的每一个子部分开始关闭
    // 3.在主线程等待各个部分的关闭结果

    // 实际上, 我们可以有以下几个技术来解决:
    // 1. 通过广播来发送信号, 让子任务都进行优雅关闭
    // 2. 通过 mpsc 的特性: 发送端drop之后, 消息通道会自动关闭, 此时继续接收消息就会报错

    // (1) 接受 'ctrl + c' 关闭信号, 并使用 mpsc 的 drop 进行通知
    let (tx, mut rx) = mpsc::channel(128);

    tokio::spawn(async move {
        println!("开始监听信息...");

        while let Some(v) = rx.recv().await {
            println!("接收到信息: {v}");
        }

        println!("接收到关闭的信号了...")
    });

    tx.send(1).await.unwrap();
    tx.send(2).await.unwrap();
    tx.send(3).await.unwrap();
    match tokio::signal::ctrl_c().await {
        Ok(()) => {
            drop(tx);
            println!("程序优雅关闭了, 我们关闭 tx 端...");
        }
        Err(e) => eprintln!("程序关闭发生错误: {e}"),
    }
}
