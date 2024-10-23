use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

#[allow(dead_code)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn new(thread_num: usize) -> Self {
        assert!(thread_num > 0);

        let mut workers = Vec::with_capacity(thread_num);
        let (sender, receiver) = mpsc::channel();

        let sender = Some(sender);
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..thread_num {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    /// 这里我们对泛型参数 F 进行约束
    ///
    /// `F: FnOnce() + Send + 'static`
    ///
    /// * FnOnce: 作为闭包约束是因为: 闭包作为任务只需被线程执行一次即可
    /// * Send: 毕竟闭包需要从一个线程传递到另一个线程(闭包内的所有数据也需要实现 Send 哦)
    /// * 'static: **意味着闭包本身及其捕获的变量都不能包含引用, 或者它们包含的引用必须具有 'static 生命周期**。
    ///   因为我们不确定闭包是什么时候被执行, 如果在闭包内进行了外部引用. 假设外部数据被销毁了, 此时就变成了悬垂引用.
    ///   如果我们想使用引用, 只能使用 'static 声明周期的引用。
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // 闭包的大小在编译时是未知的, 因为它们可以捕获不同数量的变量。
        // 直接将闭包发送到 channel 中会导致编译错误, 因为 channel '需要知道发送的数据的大小'。
        // 所以我们将 f 作为一个特征对象, 将其进行固定化
        let boxed = Box::new(f);

        // 我们不能直接使用 unwrap, 这会导致 sender 所有权被转移出来
        // 所以我们应该先获取其 '引用', 再调用 send
        self.sender.as_ref().unwrap().send(boxed).unwrap();
    }
}

impl Drop for ThreadPool {
    /// 在 `drop(self: &mut self)`中, self 仅仅是一个可变借用
    /// 我们想获取其内部字段的所有权显然是不可能的
    ///
    /// 此时我们将其字段修改为 [Option] , 然后通过 `Option::take()` 拿走内部值的所有权, 并且留下一个 `None` 在风中凌乱...
    fn drop(self: &mut Self) {
        // Drop sender, 让线程可以退出循环
        if let Some(sender) = self.sender.take() {
            drop(sender);
        }

        // 调用 join 可以等待线程停止
        for worker in self.workers.iter_mut() {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

#[allow(dead_code)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// 由于 mpsc channel 的 Receiver 只有一个
    /// 我们希望多个线程可以使用并且在同一时间内只能有一个线程能够使用
    /// 所以我们就使用 Arc + Mutex 的智能指针来包裹
    /// 每一个 Worker 都可以安全的持有 receiver, 同时不必担心一个任务会被重复执行多次
    ///
    /// 在 `thread::spawn(move || loop{...})` 为什么我们使用 `loop` 而不是 `while let` ?
    ///
    /// * `Mutex`` 结构体没有提供显式的 `unlock`, 要依赖作用域结束后的 drop 来自动释放
    /// * `let job = receiver.lock().unwrap().recv().unwrap();` 在这行代码中由于使用了 let, 右边的任何临时变量会在 let 语句结束后立即被 drop, 因此锁会自动释放
    /// * 然而 `while let` (还包括 `if let` 和 `match`) 直到最后一个花括号后, 才触发 `drop`
    ///
    /// 总结：单独使用 let 声明一个变量, 它会丢弃 **等号右边除最后一个值外的其它所有的临时变量**
    /// 而对于 `if let、while let 或 match`, 只有当它的 **整个作用域结束时, 才会丢弃等号右边除最后一个值外的其它所有的临时变量**
    ///
    /// 在 `let job = receiver.lock().unwrap().recv().unwrap();` 等号右边最后一个值以外的临时变量就会丢弃了: 也就是说 Mutex 会调用 [Drop] 了
    ///
    /// 如果要用一个终极规则或语法来说的话就是：无论是 let，还是 if let、while let 、match，只是在它的作用域结束时, 才会丢弃等号右边除最后一个值外的其它所有的临时变量。
    /// * 对于 let 来说，它并不开启一个子作用域, 而是使用它所在的作用域范围
    /// * 对于 if let、while let 、match，它们会开启一个新的子作用域，所以要等到子作用域结束
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = Some(thread::spawn(move || loop {
            match receiver.lock().unwrap().recv() {
                Ok(job) => {
                    println!("Worker {} handle a job;", id);

                    job();
                }
                Err(_) => {
                    println!("Worker {} has shutdown;", id);
                    break;
                }
            }
        }));

        Worker { id, thread }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
