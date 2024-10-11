use std::thread;

#[allow(dead_code)]
pub fn hello() {
    // () Send 和 Sync
    /*
    Send和Sync是 Rust 安全并发的重中之重!
    但是实际上它们"只是标记特征"(marker trait, 该特征未定义任何行为, 因此非常适合用于标记), 来看看它们的作用:
    1.实现 `Send` 的类型可以在线程间安全的"传递其所有权"("如果不实现则无法在不同线程中通过move传递所有权")
    2.实现 `Sync` 的类型可以在线程间 "安全的共享(通过引用)"
    */
    // () 为裸指针实现 Send trait (让其能够在线程中传递)
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
}
