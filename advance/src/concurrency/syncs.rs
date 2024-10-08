use core::time;
use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
    thread,
};

#[allow(dead_code)]
pub fn hello() {
    // 线程同步: Lock锁, Condvar条件变量, Semaphore信号量
    /*
       在多线程编程中, "同步性极其的重要"!
       当你需要同时访问一个资源、控制不同线程的执行次序时, 都需要使用到同步性

       共享内存可以说是同步的灵魂, 因为消息传递的底层实际上也是"通过共享内存"来实现, 两者的区别如下:
       * 共享内存相对消息传递能节省多次内存拷贝的成本
       * 共享内存的实现`简洁`的多
       * 共享内存的`锁竞争更多`

       消息传递适用的场景很多，我们下面列出了几个主要的使用场景:
       * 需要可靠和简单的(简单不等于简洁)实现时
       * 需要模拟现实世界，例如用消息去通知某个目标执行相应的操作时
       * 需要一个任务处理流水线(管道)时, 等等......

       而使用共享内存(并发原语)的场景往往就比较简单粗暴: "需要简洁的实现以及更高的性能时"

       1.消息传递类似一个`单所有权`的系统: "一个值同时只能有一个所有者", 如果另一个线程需要该值的所有权, 需要"将所有权通过消息传递进行转移(所有权会转移)"
       2.而共享内存类似于一个`多所有权`的系统: 多个线程可以同时访问同一个值(所有权会存在多个, 共享一块内存)
    */

    // (1) 单线程使用锁
    {
        // 如果将 Copy 特征的类型传递进去, 这会导致Mutex捕获的是Copy后的数据, 而不会修改原始的值(除非我们传递引用)
        // 填坑: 可变引用也需要看成是一种声明的类型
        let mut num = 0;
        println!("addr: {:p}", &num);
        let mutex = Mutex::new(&mut num);

        // Mutex在超出作用域后, 会通过Drop来释放锁
        {
            let mut result = mutex.lock().unwrap();
            println!("addr: {:p}", *result);
            let a: &mut i32 = *result;
            *a = 20;

            // mutex.lock(); // 锁还没有被Drop, 此时会发生死锁
        }
        println!("(1) num = {:?}", num);
    }

    // (2) 多线程使用锁
    {
        #[derive(Debug)]
        struct Foo {
            counter: u32,
        }

        impl Foo {
            fn inc(&mut self) {
                self.counter += 1;
            }
        }

        let mutex = Arc::new(Mutex::new(Foo { counter: 0 }));
        let mut handlers = Vec::with_capacity(3);

        for index in 1..=3 {
            let m = mutex.clone();
            let handler = thread::spawn(move || {
                println!("(2) {index} thread try to lock");
                let mut foo = m.lock().unwrap(); // Mutex返回的其实是引用, rust会自动进行Deref解引用来匹配方法
                (*foo).inc();
                println!("(2) {index} thread success to get the lock! begin inc...");
            });

            handlers.push(handler);
        }

        for handler in handlers {
            handler.join().unwrap();
        }
        println!("foo: {:?}", *mutex);
    }

    // (3) 死锁
    // 死锁是: 死锁是指两个或多个进程在执行过程中因争夺资源而造成的一种僵局, 当进程处于这种状态时, 它们都在等待对方释放资源, 从而无法向前推进...
    //
    // 1.单线程死锁
    // 只要你在另一个锁"还未被释放时"去申请新的锁, 就会触发
    // 锁释放需要依靠作用域之后自动Drop
    {
        let mutex = Mutex::new(0);
        let _a = mutex.lock().unwrap();
        // let _b = mutex.lock().unwrap();  // 还没释放呢, 就尝试再次获取锁, 就会导致死锁
    }
    // 2.多线程死锁
    {
        let mut handlers = Vec::with_capacity(2);
        let lock1 = Arc::new(Mutex::new(false));
        let lock2 = Arc::new(Mutex::new(false));

        for i in 1..=2 {
            let l1 = lock1.clone();
            let l2 = lock2.clone();
            let handler = thread::spawn(move || {
                // 线程一
                if i % 2 == 1 {
                    let _guard = l1.lock().unwrap();
                    println!("{i} thread获取了 lock1 的锁, 尝试锁lock2");

                    // 如果sleep的话, 将会让线程二上执行器. 此时就会导致死锁
                    // thread::sleep(time::Duration::from_millis(20));

                    let _guard = l2.lock().unwrap();
                    println!("{i} thread成功锁住lock2");
                } else {
                    let _guard = l2.lock().unwrap();
                    println!("{i} thread获取了 lock2 的锁, 尝试锁lock1");

                    let _guard = l1.lock().unwrap();
                    println!("{i} thread成功锁住lock1");
                }
            });
            handlers.push(handler);
        }

        for ele in handlers {
            ele.join().unwrap();
        }
    }
}
