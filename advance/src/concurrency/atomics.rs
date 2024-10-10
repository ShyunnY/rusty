use core::time;
use std::{
    sync::atomic::{AtomicI32, Ordering},
    thread,
};

#[allow(dead_code)]
pub fn hello() {
    // (1) 使用Atomic作为全局变量
    // 还有一点值得注意: 和Mutex一样, Atomic的值"具有内部可变性", 你无需将其声明为mut
    {
        static DATA: AtomicI32 = AtomicI32::new(0);

        // 一百个线程并发进行原子写
        // 如果发生了因为多个线程同时修改导致了脏数据, 那么这两个必将不相等
        // 但是他可是 Atomic 原子, 所以是相等的！
        fn inc() {
            for _ in 1..=100 {
                thread::spawn(|| {
                    DATA.fetch_add(1, Ordering::Relaxed);
                });
            }
        }

        inc();
        thread::sleep(time::Duration::from_secs(1));
        println!("(1) Got data: {}", DATA.load(Ordering::Relaxed));
        assert_eq!(100, DATA.load(Ordering::Relaxed));
    }

    // (2) 内存顺序
    /*
        内存顺序是指 CPU 在访问内存时的顺序, 该顺序可能受以下因素的影响:
        1.代码中的"先后顺序"
        2.编译器优化导致在编译阶段发生改变("内存重排序 reordering")
        3.运行阶段因 CPU 的缓存机制导致"顺序被打乱"
        总而言之就是指令顺序被打乱了: 本来是 a->b->c, 结果变成了 a->c->b

        对于第二点: 编译器会针对一些代码进行优化, 例如声明了 `x=1 ... x=2` 在二者之间如果没有使用 x , 那么编译期可能将 x=1 优化掉
        对于第三点: 不同的cpu缓存同步需要时间, core1的缓存可能还没同步到core2中
    */

    // (3) 限定内存顺序的5个规则
    // 在理解了内存顺序可能存在的改变后, 你就可以明白为什么Rust提供了 `Ordering::Relaxed` 用于限定内存顺序了(该枚举提供了5个成员)
    /*
        1.Relaxed: 这是最宽松的规则, 它对编译器和 CPU "不做任何限制", 可以乱序
        2.Release释放: 设定内存屏障(Memory barrier), "保证它之前的操作永远在它之前, 但是它后面的操作可能被重排到它前面"
        3.Acquire获取: 设定内存屏障, "保证在它之后的访问永远在它之后, 但是它之前的操作却有可能被重排到它后面," 往往和Release在不同线程中联合使用
        4.AcqRel: 是 Acquire 和 Release 的结合, "同时拥有它们俩提供的保证"
          比如你要对一个 atomic 自增 1, 同时希望该"操作之前和之后的读取或写入操作不会被重新排序"
        5.SeqCst顺序一致性: SeqCst就像"是AcqRel的加强版", 它不管原子操作是属于读取还是写入的操作, 只要某个线程有用到SeqCst的原子操作
          线程中该"SeqCst操作前的数据操作绝对不会被重新排在该SeqCst操作之后", 且该"SeqCst操作后的数据操作也绝对不会被重新排在SeqCst操作前"
    */
    {
        // example
    }
}
