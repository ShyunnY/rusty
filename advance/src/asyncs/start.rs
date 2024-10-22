use std::{
    future::Future,
    sync::Mutex,
    time::{self, Duration, Instant},
};

use async_std::task::sleep;
use futures::{channel::mpsc, executor::block_on, SinkExt};

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
            println!("2 ret got {}", ret);
            let ret = bar().await;
            println!("2 ret got {}", ret);
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
    }

    // 2. async的所有权移动
    // async 允许我们使用 "move" 关键字来将环境中变量的所有权转移到语句块内
    // 就像闭包那样! 好处是你不再发愁该如何解决借用 `生命周期的问题 `, 坏处就是 `无法跟其它代码实现对变量的共享`
    {
        // 此时变量属于共享阶段
        async fn share() {
            let msg = "this is msg!".to_string();

            let future_1 = async {
                println!("2.1 share context1: {}", msg);
            };
            let future_2 = async {
                println!("2.1 share context2: {}", msg);
            };

            futures::join!(future_1, future_2);
        }
        block_on(share());

        // 通过 move 关键字将所有权转移到闭包内的 Future 中
        // 由于 `async move` 会捕获环境中的变量，因此只有一个 `async move` 语句块可以访问该变量，
        // 但是它也有非常明显的好处: "变量可以转移到返回的 Future 中, 不再受借用生命周期的限制"
        async fn unshare() {
            let msg = "this is msg!".to_string();

            let future_1 = async move {
                println!("2.1 unshare context1: {}", msg);
                // 所有权还可以进行返回
                msg
            };

            // 此时不能用下列代码, 因为所有权已经被转移进 future_1
            // let future_2 = async {
            //     println!("2.1 share context2: {}", msg);
            // };

            futures::join!(future_1);
        }
        block_on(unshare());
    }

    // 3.当 ".await" 遇见多线程执行器
    /*
    需要注意的是: 当使用多线程 Future 执行器( executor )时, Future 可能"会在线程间被移动"(顾名思义, 多线程执行器)
    因此 async 语句块中的变量必须要"能在线程间传递"(如果不能在线程中移动, 可能导致 future 切换线程执行时报错)

    至于 Future 会在线程间移动的原因是:它内部的任何 ".await" 都可能导致它 "被切换到一个新线程上去执行"
    由于需要在多线程环境使用, 意味着 Rc、 RefCell 、没有实现 Send 的所有权类型、没有实现 Sync 的引用类型,它们都是不安全的
    因此无法被使用

    类似的原因: 在 ".await" 时使用普通的锁也不安全, 例如 Mutex
    原因是: 它可能会导致线程池被锁!
    当一个任务获取锁A后没有进行释放(可能内部调用了 .await 阻塞), 若它将线程的控制权还给执行器,
    然后执行器又调度运行另一个任务,该任务也去尝试获取了锁A, 结果当前线程会直接卡死, 最终陷入死锁中

    因此为了避免这种情况的发生, 我们需要使用 futures 包下的锁 futures::lock 来替代 Mutex 完成任务
    */
    {
        // example
        static LOCK: Mutex<bool> = Mutex::new(false);
        async fn foo() {
            // 获取锁
            println!("3. foo 尝试获取锁");
            let mut v = LOCK.lock().unwrap();
            *v = true;
            println!("3. foo 获取锁");
            // 此时尝试阻塞
            async {
                println!("3. foo 阻塞");
                sleep(Duration::from_secs(1)).await;
            }
            .await;
        }

        async fn bar() {
            // 获取锁
            println!("3. bar 尝试获取锁");
            let mut v = LOCK.lock().unwrap();
            *v = true;
            println!("3. bar 获取锁");
            // 此时尝试阻塞
            async {
                println!("3. bar 阻塞");
                sleep(Duration::from_secs(1)).await;
            }
            .await;
        }

        async fn exec() {
            // futures::join! 宏只能在异步函数、闭包和async块内部使用, 本质上类似于 => futures::join!(a.awit,b.await)
            // 所以 futures::join! 联合其async函数也是需要通过执行器来执行的~
            futures::join!(foo());

            // 以下语句会导致阻塞
            // futures::join!(foo(), bar());
        }

        block_on(exec());
    }

    // 4. Stream流处理
    // Stream 特征类似于 Future 特征, 但是前者在完成前可以生成多个值.
    // 这种行为跟标准库中的 Iterator 特征倒是颇为相似
    /*
        存在以下三种情况:
        1.`Ok(Some(t))`: 通道中存在消息对应了 => Poll::Ready(Some(T))
        2.`Ok(None)`: 当通道关闭且队列中没有消息时对应了 => Poll::Ready(None)
        3.`Err(e)`: 当没有可用消息，但通道尚未关闭时对应了 => Poll::Pending
    */
    {
        // 关于 Stream 的一个常见例子是消息通道（futures 包中的）的消费者 Receiver.
        // 每次有消息从 Send 端发送后, 它都可以接收到一个 Some(val) 值
        // 一旦 Send 端关闭(drop), 且消息通道中没有消息后, 它会接收到一个 None 值

        async fn demo() {
            let (mut tx, mut rx) = mpsc::channel(5);
            tx.send(1).await.unwrap();
            tx.send(2).await.unwrap();
            tx.send(3).await.unwrap();
            drop(tx);

            assert_eq!(Some(1), rx.try_next().unwrap());
            assert_eq!(Some(2), rx.try_next().unwrap());
            assert_eq!(Some(3), rx.try_next().unwrap());
            assert_eq!(None, rx.try_next().unwrap());
        }

        block_on(demo());
    }
}
