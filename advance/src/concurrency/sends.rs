use std::{
    sync::{Arc},
    thread, vec,
};

#[allow(dead_code)]
pub fn hello() {
    // (1) Rc 和 Arc 源码对比
    // {
    //     // Rc源码片段
    //     impl<T: ?Sized> !marker::Send for Rc<T> {}
    //     impl<T: ?Sized> !marker::Sync for Rc<T> {}
    //
    //     // Arc源码片段
    //     unsafe impl<T: ?Sized + Sync + Send> Send for Arc<T> {}
    //     unsafe impl<T: ?Sized + Sync + Send> Sync for Arc<T> {}
    // }
    // "!代表移除特征的相应实现"
    // Rc<T>的Send和Sync特征被特地移除了实现
    // Arc<T>则相反, 实现了Sync + Send
    // 再结合之前的编译器报错, 大概可以明白了：Send和Sync是在线程间安全使用一个值的关键

    // (2) Send 和 Sync
    /*
        Send和Sync是 Rust 安全并发的重中之重!
        但是实际上它们 "只是标记特征"(marker trait, "该特征未定义任何行为", 因此非常适合用于标记), 来看看它们的作用:
        1.实现 `Send` 的类型可以在线程间安全的"传递其所有权"("如果不实现则无法在不同线程中通过move传递所有权")
        2.实现 `Sync` 的类型可以在线程间 "安全的共享(通过引用)"
        3.由上可知, 若类型 T 的引用 &T 是Send, 则T是Sync(引用可以在多个线程中send发送, 那么类型必然是 Sync 的)
    */

    // (3) 为裸指针实现 Send trait (让其能够在线程中传递)
    {
        // 使用 newType 的方式
        // 复合类型中有一个成员没实现Send, 该复合类型就不是Send!
        // 因此我们 "需要手动为它实现"
        #[derive(Debug)]
        struct MutI32(*mut i32);

        unsafe impl Send for MutI32 {}

        let mut num = 100;
        let p: MutI32 = MutI32(&mut num);

        // 需要将所有权转移进去(启动一个新的线程情况下都需要这样做)
        thread::spawn(move || {
            println!("{:?}", p);

            unsafe { *p.0 = 10 }
        })
        .join()
        .unwrap();
    }

    // (4) 为裸指针实现 Sync
    /*
       需要注意以下几点须知:
       1.线程是 "无法直接" 去借用其它线程的变量, 原因在于编译器无法确定主线程main和子线程t谁的生命周期更长(这就导致会出现非法借用)
       2.所以我们需要使用Arc(其实如果我们不用在多个线程中拥有其所有权也可以不用的)
    */
    {
        #[derive(Debug)]
        struct MutI32(*mut i32);

        // 由于我们需要在多个线程中共享其引用, 所以我们需要实现 Sync trait
        unsafe impl Sync for MutI32 {}
        // 由于我们需要将所有权发送到新的线程中, 所以我们需要实现 Send trait
        unsafe impl Send for MutI32 {}

        let mut data = 999;
        let p = Arc::new(MutI32(&mut data));
        let n = Arc::clone(&p);

        thread::spawn(move || {
            println!("实现了Sync的裸指针值: {:?}", *n);
        })
        .join()
        .unwrap();
    }

    // (5) 总结:
    /*
        1.实现 Send 的类型可以在线程间安全的"传递其所有权", 实现 Sync 的类型可以"在线程间安全的共享(通过引用)"
        2.绝大部分类型都实现了 Send 和 Sync, 常见的未实现的有：裸指针、Cell、RefCell、Rc 等
        3.可以为自定义类型实现Send和Sync，但是"需要unsafe代码块"
        4.可以为部分 Rust 中的类型实现Send、Sync, 但是需要"使用newtype", 例如文中的裸指针例子(newtype)大法
    */

    // (6) Q&A:
    /*
        Q: 为什么仅仅声明了 "unsafe impl Sync for MyBox {}" 就可以通过编译了?
        A: 对于`unsafe impl Sync for MyBox {}`这样的代码
           它实际上是在告诉编译器: "我知道这个类型 (MyBox in this case) 是 Sync 的"
           即它可以在多个线程之间安全地共享, "尽管编译器无法验证这一点!!!(这需要我们自己保证)"
           这可能是因为实际上 MyBox 类型的实现确实是安全的,或者因为程序员在使用这个类型时会采取额外的措施来确保线程安全性
           在这种情况下使用 unsafe 是一种在代码中"做出显式声明的方式": 表示程序员"自己负责确保线程安全", 而编译器无法提供静态检查
           unsafe 关键字允许绕过编译器的某些安全检查, 包括对多线程安全性的检查. 如果出问题将是你自己负责

        Q: 如何理解: "如果 T 为 Sync 则 &T 为 Send，如果 &T 为 Send 则 T 为 Sync"
        A: 一个类型要在线程间安全的共享的前提是, 指向它的引用必须能在线程间传递
           因为如果引用都不能被传递, 我们就无法在多个线程间"使用引用去访问同一个数据了"
    */
    {
        let arr = vec![1, 2, 3, 4];
        let _res: i32 = arr.iter().filter(|&x| x % 2 == 0).sum();
    }
}
