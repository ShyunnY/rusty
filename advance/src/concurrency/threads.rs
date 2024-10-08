use core::time;
use std::{
    cell::Cell,
    sync::{Arc, Barrier, Condvar, Mutex, Once},
    thread,
};

use thread_local::ThreadLocal;

#[allow(dead_code)]
pub fn hello() {
    // (1) 创建线程
    // 使用 `thread::spawn`` 可以创建一个子线程 (spawn意味着产生!)
    // 线程调度的方式往往取决于你使用的操作系统,
    // 我们 "千万不要依赖线程的执行顺序" ! 线程的调度是不可预估的!
    {
        thread::spawn(|| {
            for ele in 1..=3 {
                println!("sub thread: {}", ele);
            }
        });

        // 由于main线程结束会导致子线程可能无法调度, 所以使main线程休眠让core调度子线程
        println!("main thread sleep...");
        thread::sleep(time::Duration::from_millis(100));
    }
    /*
       有几点需要注意!
       1.线程内部的代码"使用闭包函数来执行"
       2.main 线程一旦结束, 程序就"立刻结束". 因此需要保持它的存活, 直到其它子线程完成自己的任务
       3.`thread::sleep`` 会让当前线程休眠指定的时间, 随后其它线程会被调度运行,
          因此就算你的电脑只有一个 CPU 核心, 该程序也会表现的如同多 CPU 核心一般, 这就是并发！
    */

    // (2) 等待子线程的结束
    {
        let handler = thread::spawn(|| {
            for ele in 1..=3 {
                println!("(2). sub thread: {ele}");
            }
        });

        handler.join().unwrap(); // join: 阻塞当前的线程, 等待spawn的子线程执行完毕

        println!("(2). main thread!");
    }

    // (3) 在线程闭包中使用 move
    {
        let arr = vec![11, 22, 33, 44, 55];

        let handler = thread::spawn(move || {
            arr.iter().for_each(|x| println!("(3) sub thread: {x}"));
        });

        handler.join().unwrap();
    }

    // (4) 线程是如何结束的
    {
        // let t1 = thread::spawn(|| {
        //     thread::spawn(|| loop {
        //         println!("(4) i am gloria!");
        //     })
        // });

        // t1.join().unwrap();
        // thread::sleep(time::Duration::from_millis(3));
    }

    // (5) 线程屏障 Barrier
    // 在Rust中, 可以使用 Barrier 让多个线程都"执行到某个点后", 才继续一起往后执行
    // 也就是通过 barrier 控制所有线程需要满足当前的条件才能向后执行
    {
        let mut handlers: Vec<thread::JoinHandle<()>> = Vec::with_capacity(3);
        let barrier = Arc::new(Barrier::new(3));

        for index in 1..=3 {
            // 我们需要通过Arc智能指针在多个线程中安全的引用Barrier
            let b = barrier.clone();
            let handler = thread::spawn(move || {
                println!("{} already wait barrier", index);
                // 添加一个线程屏障, 等待所有线程到达屏障后才开放后续的执行
                b.wait();
                println!("{} begin exec", index);
            });
            handlers.push(handler);
        }

        for handler in handlers {
            handler.join().unwrap();
        }
    }

    // (6) 线程局部变量
    // *thread_local!宏实现: 太麻烦了
    // *thread-local第三方create实现
    {
        let counter = Arc::new(ThreadLocal::new());
        let mut handlers = Vec::with_capacity(3);

        for _ in 0..=2 {
            let c = Arc::clone(&counter);
            let handler = thread::spawn(move || {
                let cc = c.get_or(|| Cell::new(0u32));
                cc.set(cc.get() + 1);
            });

            handlers.push(handler);
        }

        for handler in handlers {
            handler.join().unwrap();
        }

        let result = Arc::try_unwrap(counter).unwrap();
        let total = result.into_iter().fold(0, |x, y| x + y.get());

        println!("total: {}", total);
    }

    // (7) 通过条件控制线程挂起和执行
    {
        let lock = Arc::new((Mutex::new(false), Condvar::new()));
        let l = Arc::clone(&lock);

        thread::spawn(move || {
            let (lock, cvar) = &*l;
            let mut started = lock.lock().unwrap();
            *started = true; // 通过deref修改值
            println!("changing stared");
            cvar.notify_one(); // 使用 cond_var 进行通知
        });

        let (l, cvar) = &*lock;
        let mut started = l.lock().unwrap(); // 获取互斥锁守卫
        while !*started {
            started = cvar.wait(started).unwrap(); // 等待守卫（condVar会自动获取当前线程中的Mutex中的guard守卫）
        }

        println!("started changed");
    }

    // (8) 只会调用一次的函数
    {
        static mut FLAGS: u32 = 0u32;
        static INIT: Once = Once::new();

        thread::spawn(move || {
            INIT.call_once(|| unsafe {
                FLAGS = 1;
            });
        });

        thread::spawn(move || {
            INIT.call_once(|| unsafe {
                FLAGS = 2;
            });
        });

        thread::sleep(time::Duration::from_millis(500));
        println!("(8) flags: {}", unsafe { FLAGS });
    }
}
