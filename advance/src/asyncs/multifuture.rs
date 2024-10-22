use std::time::Duration;

use async_std::task::{block_on, sleep};
use futures::{
    future::{Fuse, FusedFuture, FutureExt},
    join, pin_mut, select, try_join, TryFutureExt,
};

#[allow(dead_code)]
pub fn hello() {
    // 同时运行多个 Future
    // 俗话说: 如果只运行一个 Future, 还不如写同步代码!
    // 在异步编程下我们一般都是需要 "同时运行多个 Future"

    // 1. join!
    // one();

    // 2. try_join!
    // two();

    // 3. select! (注: 个人感觉类似于golang的 select )
    three();
}

#[allow(dead_code)]
fn one() {
    // join!宏: 它允许我们同时等待多个不同 Future 的完成, 且可以并发地运行这些 Future
    //
    // 如果希望同时运行 "一个数组" 里的多个异步任务, 可以使用 futures::future::join_all 方法
    async fn foo() {
        sleep(Duration::from_millis(1000)).await;
        println!("i'm foo!");
    }

    async fn bar() {
        sleep(Duration::from_millis(1500)).await;
        println!("i'm bar!");
    }

    async fn exec() {
        let foo = foo();
        let bar = bar();

        // 这种写法是顺序输出的
        // Rust 中的 Future 是惰性的, 直到调用 .await 时, 才会开始 poll 运行
        // 而以下两个 await 由于在代码中有先后顺序, 因此它们是顺序运行的
        // (foo.await, bar.await);

        // 正确方式应该是: join!宏
        // 尽管 join!(a, b) 类似于 (a.await, b.await), 但join!是 "同时轮询两个 future", 因此效率更高
        join!(foo, bar);
    }

    block_on(exec());
}

#[allow(dead_code)]
fn two() {
    // 由于 join! 必须等待它管理的所有 Future 完成后才能完成
    // 如果你希望 '在某一个 Future 报错后就立即停止所有 Future 的执行', 可以使用 try_join!
    // 特别是当 Future 返回 Result 时
    //
    // 有一点需要注意: 传给 try_join! 的所有 Future "都必须拥有相同的错误类型"(那我们可以使用 'anyhow' 将错误归一化)
    // 如果错误类型不同, 可以考虑使用来自 futures::future::TryFutureExt 模块的 map_err 和 err_info 方法将错误进行转换

    {
        async fn foo() -> Result<(), String> {
            println!("foo?");
            sleep(Duration::from_millis(1000)).await;
            println!("i'm foo!");
            Err(String::from("foo 提前返回错误"))
        }

        async fn bar() -> Result<(), String> {
            println!("bar?");
            sleep(Duration::from_millis(1500)).await;
            println!("i'm bar!");
            Ok(())
        }

        async fn zar() -> Result<(), ()> {
            println!("zar?");
            sleep(Duration::from_millis(2000)).await;
            println!("i'm zar!");
            Err(())
        }

        async fn exec() {
            let foo = foo();
            let bar = bar();
            // 使用 TryFutureExt 的 map_err() 将错误转换为所有 Future 同一个类型
            let zar = zar().map_err(|()| "zar error".to_string());

            if let Err(msg) = try_join!(foo, bar, zar) {
                println!("谁返回了错误? 是我: {}", msg);
            }
        }

        block_on(exec());
    }
}

#[allow(dead_code)]
fn three() {
    // join! 只有等所有 Future 结束后, 才能 '集中' 处理结果 (注意这个集中的意思)
    // 如果你想同时等待多个 Future, 且任何一个 Future 结束后都可以立即被处理, 可以考虑使用 futures::select!
    //
    // 注意: 实际上我们使用 select!宏时, select 实际上会按照声明的顺序进行 poll()
    // 如果我们希望某个 future 可以优先 poll, 那么可以调整一下分支的位置

    // 1. select! 入门使用
    {
        async fn foo() {
            println!("foo");
            sleep(Duration::from_millis(150)).await;
            println!("i'm foo!");
        }

        async fn bar() {
            println!("bar");
            sleep(Duration::from_millis(100)).await;
            println!("i'm bar!");
        }

        /// fuse() 是 Future 的一个方法, 用于将一个 Future 转换为一个“熔断”（fuse）的 Future
        /// 最简单说: fuse Future 只会执行一次(因为再次执行将会返回 Poll::Pending, 让其余 Future 执行)
        async fn exec() {
            let foo = foo().fuse();
            let bar = bar().fuse();
            pin_mut!(foo, bar);

            // select! 宏的工作原理是轮询多个 Future, 并在其中一个 Future 完成时返回
            // 如果一个 Future 在完成一次后'再次被轮询', 并且返回 Poll::Pending, 这可能会导致 select! 宏 '陷入死循环或产生未定义行为'
            select! {
                () = foo => println!("foo 先完成"),
                () = bar => println!("bar 先完成"),
            }
        }

        block_on(exec());
    }

    // 2. 使用 default + complete
    // select!宏 还支持 default 和 complete 分支:
    // + complete 分支: 当所有的 Future 和 Stream 完成后才会被执行, 它往往配合 loop 使用, loop 用于循环完成所有的 Future
    // + default 分支:  若没有任何 Future 或 Stream 处于 Ready 状态, 则该分支 '会被立即执行'
    {
        async fn foo() {
            println!("foo");
            sleep(Duration::from_millis(150)).await;
            println!("i'm foo!");
        }

        async fn bar() {
            println!("bar");
            sleep(Duration::from_millis(100)).await;
            println!("i'm bar!");
        }

        async fn exec() {
            let foo = foo().fuse();
            let bar = bar().fuse();
            pin_mut!(foo, bar);

            loop {
                select! {
                    () = foo => println!("foo 完成啦"),
                    () = bar => println!("bar 完成啦"),
                    complete => {
                        // 当所有的 future 都处于 Ready 状态时, 就会执行 complete
                        println!("全部都完成啦");
                        break;
                    },
                    // 如果我们希望 default 先执行, 那么把 default 分支放在第一位~~~
                    default => {
                        // 如果其他 future 还在执行, 那么就会执行 default 分支
                        println!("我是默认执行的!");
                    },
                }
            }
        }
        block_on(exec());
    }

    // 3. 跟 Unpin 和 FusedFuture 进行交互
    // .fuse() 方法可以让 Future 实现 FusedFuture 特征, 而 pin_mut! 宏会为 Future 实现 Unpin 特征, 这两个特征恰恰是使用 select 所必须的:
    // + Unpin: 由于 select 不会通过拿走所有权的方式使用 Future, 而是"通过可变引用的方式去使用", 这样当 select 结束后, 该 Future 若没有被完成, "它的所有权还可以继续被其它代码使用"
    // + FusedFuture 的原因跟上面类似, 当 Future 一旦完成后，那 select 就不能再对其进行轮询使用. (通过返回 Poll::Pending 让 Future 让出执行权)
    // 只有实现了 FusedFuture, select 才能配合 loop 一起使用. 假如没有实现, 就算一个 Future 已经完成了, 它依然会被 select 不停的轮询 poll() 执行

    // 4. 使用 Fuse::terminated() 在 select! 中填充任务
    {
        async fn foo() {
            println!("foo");
            sleep(Duration::from_millis(150)).await;
            println!("i'm foo!");
        }

        async fn exec() {
            let foo = foo().fuse();
            pin_mut!(foo);

            let bar_ft = Fuse::terminated();
            pin_mut!(bar_ft);

            loop {
                select! {
                    () = bar_ft => println!("bar 完成啦"),
                    () = foo => {
                        println!("foo 完成啦, 然后填充 bar_ft");
                        if bar_ft.is_terminated(){
                            // 通过 set 设置一个新的 async Future
                            bar_ft.set(async{
                                println!("bar_rt");
                                sleep(Duration::from_millis(150)).await;
                                println!("i'm bar_rt!");
                            }.fuse());
                        }
                    },
                    complete => {
                        // 当所有的 future 都处于 Ready 状态时, 就会执行 complete
                        println!("全部都完成啦");
                        break;
                    },
                }
            }
        }
        block_on(exec());
    }
}
