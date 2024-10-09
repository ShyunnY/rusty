use core::time;
use std::{
    sync::{Arc, Condvar, Mutex, RwLock},
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

                    //let _guard = l1.lock().unwrap();
                    //println!("{i} thread成功锁住lock1");
                }
            });
            handlers.push(handler);
        }

        for ele in handlers {
            ele.join().unwrap();
        }
    }
    // 3.try_lock
    // lock方法不同, try_lock会"尝试去获取一次锁", 如果无法获取会返回一个错误, 因此不会发生阻塞
    /*
       一个有趣的命名规则: 在Rust标准库中, 使用`try_xxx`都会尝试进行一次操作.
       如果无法完成就立即返回, 不会发生阻塞.
    */
    {
        let mutex = Mutex::new(0);
        let _l = mutex.lock().unwrap();

        // 通过try_lock尝试上锁
        // 在这里会报出一个错误:Err("WouldBlock"), 接着线程中的剩余代码会继续执行, 不会被阻塞
        match mutex.try_lock() {
            Ok(v) => println!("result: {}", *v),
            Err(e) => println!("获取锁失败: {}", e),
        };
    }

    // (4) 读写锁 RwLock
    /*
       简单总结下RwLock:
       1.同时允许"多个读", 但最多只能有"一个写"
       2.读和写不能同时存在(互斥性)
       3.读可以使用read、try_rea, 写write、try_write, 在实际项目中, try_xxx会安全的多
    */
    {
        // 读写锁其实就是一个在同一时间只能有一个写或者多个读存在(读和写互斥的)
        let rw = RwLock::new(10);
        {
            // 多个读锁可以共存
            let r1 = rw.read().unwrap();
            let r2 = rw.read().unwrap();

            println!("(4) 读取到值 = {}", *r1);
            println!("(4) 读取到值 = {}", *r2);
        }
        {
            // 读和写, 写和写是互斥的（同一时间只能存在一个写）
            let mut w = rw.write().unwrap();
            *w = 20;

            // 以下代码会导致死锁, 因为读写锁共存了
            // let r = rw.read().unwrap();
            // println!("(4) 读取到值 = {}", *r);
        } // WriteLock写锁在这里才Drop
    }

    // (5) Mutex 还是 RwLock
    /*
       首先简单性上Mutex完胜, 因为使用RwLock你得操心几个问题：
       1.读和写不能同时发生, 如果使用try_xxx解决, 就必须做大量的错误处理和失败重试机制
       2.当读多写少时, 写操作可能会因为一直无法获得锁导致连续多次失败(可能导致写饿死)
       3.RwLock 其实是操作系统提供的，实现原理要比Mutex复杂的多，因此单就锁的性能而言，比不上原生实现的Mutex

       再来简单总结下两者的使用场景:
       1.追求高并发读取时, 使用RwLock. 因为Mutex一次只允许一个线程去读取
       2.如果要"保证写操作"的成功性, 使用Mutex
       3.不知道哪个合适, 统一使用Mutex

       RwLock虽然看上去貌似提供了高并发读取的能力, 但这个不能说明它的性能比Mutex高
       事实上Mutex性能要好不少, 后者唯一的问题也仅仅在于"不能并发读取"

    总结来看: 如果你要使用RwLock要确保满足以下两个条件
    1.并发读
    2.需要对读到的资源进行"长时间"的操作
    */

    // (6) 条件变量(Condvar)控制线程的同步
    // Condvar经常和Mutex一起使用, 可以让线程挂起直到某个条件发生后再继续执行
    {
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair_clone = Arc::clone(&pair);

        thread::spawn(move || {
            let (lock, cvar) = &*pair_clone;
            let mut started = lock.lock().unwrap();

            while !*started {
                println!("等待被唤醒");
                // wait 阻塞当前线程, 直到该条件变量接收到 notification
                // wait首先会释放锁, 然后处于等待通知阶段. 接受到通知之后"需要重新持有锁才能向后执行", 如果拿不到锁也执行不了滴!
                started = cvar.wait(started).unwrap();
            }
            println!("成功被唤醒！")
        });

        thread::sleep(time::Duration::from_millis(100));

        let (lock, cvar) = &*pair;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one(); // notify仅仅是通知另外一个线程中的condVar（具体他是否执行取决于是否能获取锁）
    } // 在这里main释放了锁, 子线程持有锁之后才能进一步执行

    // (7) 信号量Semaphore
    // 在多线程中, 另一个重要的概念就是信号量.
    // 使用它可以让我们精准的控制当前正在运行的任务最大数量
    {
        // todo
    }

    /*
    在很多时候, 消息传递都是非常好用的手段, 它可以让我们的数据在任务流水线上不断流转, 实现起来非常优雅

    但是它并不能优雅的解决所有问题, 因为我们面临的真实世界是非常复杂的!无法用某一种银弹统一解决
    当面临消息传递不太适用的场景时, 或者"需要更好的性能和简洁性时, 我们往往需要用锁来解决这些问题"
    因为锁允许多个线程同时访问同一个资源, 简单粗暴
    */
}
