use std::{
    future::Future,
    time::{self, Instant},
};

use async_std::task::sleep;
use futures::executor::block_on;

#[allow(dead_code)]
pub fn hello() {
    // async/await入门
    //one();

    // async/await以及Stream
    two();
}

#[allow(dead_code)]
fn one() {
    // async/.await 是 Rust 内置的语言特性，可以让我们"用同步的方式去编写异步的代码"
    // 通过 async 标记的语法块会被转换成实现了Future特征的状态机
    // 与同步调用阻塞当前线程不同, 当Future执行并遇到阻塞时, "它会让出当前线程的控制权"
    // 这样其它的Future就可以在该线程中运行, 这种方式完全不会导致当前线程的阻塞(让其他 future 能够执行)
    /*
    // async标注的函数返回的是一个 future 而已, 并不会进行执行(惰性执行)
    // 只有上了状态机才能真正的进行执行(也就是说需要一个执行器才能进行执行)
    // 想要在 async 函数中调用其他 async 函数
    // 两种解决方法: 使用.await语法或者对Future进行轮询(poll)
     */

    // 1. 使用 async
    // 异步函数的返回值是一个 Future, 若直接调用该函数不会输出任何结果, 因为 Future 还未被执行
    // 我们需要使用一个 executor 执行器来进行执行 future
    {
        async fn go() {
            println!("1. go,go,go!");
        }

        // `block_on`会"阻塞当前线程直到指定的`Future`执行完成", 这种阻塞当前线程以等待任务完成的方式较为简单、粗暴
        // 好在其它运行时的执行器(executor)会提供更加复杂的行为, 例如将多个`future`调度到同一个线程上执行
        block_on(go());
    }

    // 2. 使用 await
    // 如果你要在一个async fn函数中去调用另一个async fn并等待其完成后再执行后续的代码, 该如何做？
    // 我们需要使用 "await" 关键字等待一个异步函数执行完成
    // 但是与block_on不同, .await并不会阻塞当前的线程而是异步的等待Future A的完成
    // 在等待的过程中, 该线程还可以继续执行其它的Future B, 最终实现了并发处理的效果
    // 换句话说: 我们使用 await 时只是等待异步函数的执行, 此时线程是不会被阻塞的. 可以让其执行其他 future 任务(仅仅阻塞当前 future)
    {
        // 使用同步的代码顺序实现了异步的执行效果，非常简单、高效，而且很好理解，未来也绝对不会有回调地狱的发生
        async fn golang() {
            rust().await;
            println!("2. hello,golang!");
        }

        async fn rust() {
            println!("2. hello,rust!")
        }

        block_on(golang());
    }

    // 3. 一个栗子~
    {
        async fn learn_song() {
            println!("正在学习唱歌");
            // 此时会异步等待, 并让线程执行其他futures
            sleep(time::Duration::from_millis(1000)).await;
            println!("学会唱 Find you~")
        }

        async fn learn_dance() {
            println!("正在学习跳舞");
            sleep(time::Duration::from_millis(2000)).await;
            println!("学会跳名族舞")
        }

        async fn exec() {
            futures::join!(learn_song(), learn_dance());
        }

        // 此时就是用异步方式进行并发
        // 以上代码同步情况下需要执行3s, 异步情况下只需要执行2s
        let now = Instant::now();
        block_on(exec());
        println!(
            "3. 异步学习唱歌跳舞的耗时: {:?}ms",
            now.elapsed().as_millis()
        );
    }

    /*
       换个思路理解
       这里的异步是在单线程（Main线程）上做异步, 如果用 Thread.sleep 是整个 Main 线程都阻塞挂起了
       异步是指 Main 线程上有2个IO操作耗时, async异步函数会创建2个Futuer, 如果一个 Futuer 被阻塞了那么会执行另外一个 Future

       实际上在 main 线程中运行, 自始至终都只存在一个线程. 这过程只涉及 future 切换而不涉及 线程切换

       总结:
       因此 .await 对于实现异步编程至关重要, 它允许我们"在同一个线程内并发的运行多个任务, 而不是一个一个先后完成"
       大胆猜测一下: 这是不是类似于 Goroutine 呢?
       async await是一个标记作用, 编译器会将对应函数和函数调用封装成Future, 然后交给 async runtime 去调度执行, 这才是真正开始了并发
    */
}

#[allow(dead_code)]
fn two() {
    /*
       async/.await 是Rust语法的一部分, 它在遇到阻塞操作时(例如 IO)会"让出当前线程的所有权而不是阻塞当前线程"
       这样就"允许当前线程继续去执行其它代码, 最终实现并发"

       有两种方式可以使用 async:
       1.async fn 用于声明函数
       2.async { ... } 用于声明语句块，它们会返回一个实现 Future 特征的值

       原理:
       async 是懒惰的, 直到"被执行器 poll "或者 ".await" 后才会开始运行, 其中后者是最常用的运行 Future 的方法!
       当 ".await" 被调用时, 它会 "尝试运行 Future 直到完成", 但是若该 Future 进入阻塞, 那就会"让出当前线程的控制权"
       当 Future 后面准备再一次被运行时(例如从 socket 中读取到了数据),
       执行器"会得到通知(应该是通过 wake 函数进行通知吧)"并再次运行该 Future, 如此循环直到完成
       (所以我们经常能看见 ".await" 来显式尝试执行一个 Future)
    */
    {
        async fn foo() -> i32 {
            10
        }

        fn bar() -> impl Future<Output = i32> {
            async { 20 }
        }

        async fn exec() {
            let ret = foo().await;
            println!("2.1 ret got {}", ret);
            let ret = bar().await;
            println!("2.1 ret got {}", ret);
        }

        block_on(exec());
    }

    // 1. async的生命周期
    /*
        "async fn"函数如果拥有 `引用类型的参数`, 那它返回的 Future 的生命周期就会"被这些参数的生命周期所限制"

        async fn foo(x: &u8) -> u8 { *x }

        // 上面的函数跟下面的函数是等价的:
        // 这代表 Future 与传递的参数具有相同的生命周期(如果是多个引用类型参数可能会更复杂)
        fn foo_expanded<'a>(x: &'a u8) -> impl Future<Output = u8> + 'a {
            async move { *x }
        }

        // 意味着async fn函数返回的 Future 必须满足以下条件:
        // 当 x 依然有效时,该 Future 就必须继续等待( .await )
        // "也就是说引用参数x必须比 Future 活得更久"

    */
    {
        // 在一般情况下在函数调用后就立即 .await 不会存在任何问题, 例如foo(&x).await
        // 若 Future 被先存起来或发送到另一个任务或者线程, 就可能存在问题了
        async fn foo(x: &i32) -> i32 {
            *x
        }

        fn bar() -> impl Future<Output = i32> {
            // let x = 10;
            // foo(&x)  // x 超过这里就会被销毁, 生命周期不够

            // 通过 "将参数移动到 async 语句块内, 我们将它的生命周期扩展到 'static"
            // 并跟返回的 Future 保持了一致
            async {
                let x = 10;
                foo(&x).await
            }
        }

        async fn good<'a, 'b>(_x: &'a i32, y: &'b i32) -> &'b i32 {
            y
        }
    }
}
