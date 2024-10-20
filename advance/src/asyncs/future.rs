use {
    async_std::task::block_on,
    core::time,
    futures::{
        future::{BoxFuture, FutureExt},
        task::{waker_ref, ArcWake},
    },
    std::{
        future::Future,
        sync::{
            mpsc::{sync_channel, Receiver, SyncSender},
            Arc, Mutex,
        },
        task::{Context, Poll, Waker},
        thread,
    },
};

/// 总结一下几个要点:
/// 1.async作为一个future, "每次poll遇到await停止,进入await方法里的poll"(也就是执行await函数)
/// 2.await方法内 "awake 唤醒的是上层调度逻辑", 而不是await方法本身
/// 3.唤醒调度逻辑后会"继续在上层调度逻辑进行poll", 然后 "再次进入await方法里的poll" 直到该await方法ready
/// 4.当检测到await方法ready, 那么"该await方法就相当于poll内的同步代码了，直接往下走"
/// 再次总结一下async内碰到await的逻辑：
/// 1. async碰到await，会进入到await方法内的poll
/// 2. await poll pending: 那么记录当前future，等待await中调用 wake 唤醒执行器
/// 3. await poll ready: 则await执行完毕, 继续同步代码直到再次await
///
/// Rust 的 Future 是惰性的: 只有屁股上拍一拍, 它才会努力动一动.
/// 其中一个推动它的方式就是"在 async 函数中使用 .await 来调用另一个 async 函数"
/// 但是这个只能解决 async 内部的问题, 那么这些最外层的 async 函数谁来推动它们运行呢？
/// 答案就是我们之前多次提到的执行器 executor
///
/// 执行器会管理一批 Future (最外层的 async 函数), 然后通过"不停地 poll 推动它们直到完成"
/// 最开始，执行器会先 poll 一次 Future, 后面就不会主动去 poll 了, 而是等待 Future 通过调用 wake 函数来通知它可以继续
/// 它才会继续去 poll. 这种 wake 通知然后 poll 的方式会不断重复, 直到 Future 完成
///
/// 总结一下以下几点:
/// 1. Poll::Ready(T)代表当前的 future 执行完成啦, Poll::Pending 代表当前的 future 还没执行完, 可以让出当前线程所有权
/// 2. Executor 仅会在一开始对所有外层的 Future 调用一次 poll(), 后续就等待 future 调用 wake() 通知执行器执行 poll()
/// 3. 在 async 函数中使用 .await 来调用另一个 async 函数相当于让调度器执行该 await future的 poll()
#[allow(dead_code)]
pub fn hello() {
    // 实现一个简单的 future
    // simple_future()

    // 实现一个简单的 executor
    simple_executor();
}

#[allow(dead_code)]
fn simple_future() {
    {
        #[derive(Debug)]
        struct IOFuture {
            shared_state: Arc<Mutex<SharedState>>,
        }

        impl IOFuture {
            fn new() -> Self {
                IOFuture {
                    shared_state: Arc::new(Mutex::new(SharedState {
                        is_done: false,
                        waker: None,
                    })),
                }
            }

            fn modify(&self) {
                let shared_state_clone = self.shared_state.clone();
                thread::spawn(move || {
                    println!("future开始阻塞等待事件处理");
                    thread::sleep(time::Duration::from_millis(1500));
                    let mut ret = shared_state_clone.lock().unwrap();
                    ret.is_done = true;
                    println!("future处理完毕, 可以通知执行器了");

                    // 这里需要使用 take 而不是 unwrap, 因为unwrap会导致所有权移动Option不可用
                    // unwrap() 会转移整个 Option 的所有权, 而 take() 会保留下一个None在原始位置

                    if let Some(waker) = ret.waker.take() {
                        waker.wake(); // wake to Future Executor
                    }
                });
            }
        }

        /// 实现 Future trait, 构建一个自己的 future
        impl Future for IOFuture {
            type Output = ();

            fn poll(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<Self::Output> {
                let mut shared_state = self.shared_state.lock().unwrap();

                if !shared_state.is_done {
                    println!("future还没准备好, 调用modify");
                    shared_state.waker = Some(cx.waker().clone());
                    self.modify();
                    return Poll::Pending;
                }

                println!("future已经 Ready!");
                Poll::Ready(())
            }
        }

        #[derive(Debug)]
        struct SharedState {
            is_done: bool,
            waker: Option<Waker>,
        }

        async fn exec() {
            let io = IOFuture::new();
            let f = async { println!("io future在忙碌, 先看看我!") };
            futures::join!(io, f);
        }
        block_on(exec());
    }
}

#[allow(dead_code)]
fn simple_executor() {
    {
        /// Executor 用于执行 future 的执行器
        struct Executor {
            ready_queue: Receiver<Arc<Task>>,
        }

        impl Executor {
            fn run(&self) {
                while let Ok(task) = self.ready_queue.recv() {
                    // 获取 task 中的 future
                    let mut future = task.future.lock().unwrap();

                    if let Some(mut f) = future.take() {
                        // 创建一个 LocalWaker
                        let waker = waker_ref(&task);
                        let context = &mut Context::from_waker(&*waker);

                        // 这里调用 poll() 执行传入的 future, 如果是阻塞的, 我们需要将其再放回去
                        if f.as_mut().poll(context).is_pending() {
                            *future = Some(f);
                        }
                    }
                }
            }
        }

        /// `Spawner` 用于创建一个 task 然后将其发送到消息通道中
        struct Spawner {
            future_sender: SyncSender<Arc<Task>>,
        }

        impl Spawner {
            /// spawn 主要是将 future 转换成一个 Box 放在堆上固定住
            /// 然后将 future 添加到 channel 中进行发送
            fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
                let future = future.boxed();
                let task = Arc::new(Task {
                    future: Mutex::new(Some(future)),
                    future_sender: self.future_sender.clone(),
                });

                self.future_sender
                    .send(task)
                    .expect("future channel has overflow");
            }
        }

        /// `Task` 管理了 future 和 future_sender
        ///
        /// 因为 future 仅在开始会被 Executor 执行器执行一次, 后续都是通过 future 调用 wake() 函数通知.
        /// 所以我们需要管理一个 future_sender 用于将自身 future 发送到 Executor上
        struct Task {
            future: Mutex<Option<BoxFuture<'static, ()>>>,

            future_sender: SyncSender<Arc<Task>>,
        }

        /// 当 Task 实现了 ArcWake 特征后, 它就变成了 Waker
        /// 在调用 wake() 对其唤醒后会将任务复制一份所有权( Arc ), 然后将其发送到任务通道中
        /// 最后我们的执行器将从通道中获取任务, 然后进行 poll 执行
        impl ArcWake for Task {
            /// 实际上, 我们调用 wake() 函数内部就是调用了当前函数
            fn wake_by_ref(arc_self: &Arc<Self>) {
                let cloned = arc_self.clone();

                // 将当前 Task 的 Arc 指针发送到 channel 中, 让执行器去执行
                arc_self
                    .future_sender
                    .send(cloned)
                    .expect("future channel has overflow");
            }
        }

        /// 创建一个 Executor执行器 和 Spawner创造器
        fn new_executor_and_spawner() -> (Executor, Spawner) {
            const MAX_SIZE: usize = 100;
            let (tx, rx): (SyncSender<Arc<Task>>, Receiver<Arc<Task>>) = sync_channel(MAX_SIZE);

            (Executor { ready_queue: rx }, Spawner { future_sender: tx })
        }

        async fn foo() {
            println!("我需要被 await 调用以此执行 poll");
        }

        let (executor, spawner) = new_executor_and_spawner();
        spawner.spawn(async {
            println!("hello!");
            // 通过 await 来触发 foo Future的 poll 执行
            foo().await;
            println!("world");
        });
        drop(spawner);
        executor.run();
    }
}
