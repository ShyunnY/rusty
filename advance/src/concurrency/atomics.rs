use core::time;
use std::{
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc,
    },
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

        6. 如果不知道怎么选择, 那么我们就优先使用SeqCst, 虽然会稍微减慢速度但是慢一点也比出现错误好
        7. 多线程只计数 `fetch_add` 而不使用该值触发其他逻辑分支的简单使用场景, 可以使用Relaxed
    */

    // (4) 在多线程中使用 Atomic 原子
    {
        // 在多线程中使用 Atomic 原子需要用 Arc 对其进行包裹(因为Atomic也具有所有权)
        let data = Arc::new(AtomicI32::new(0));
        for _ in 1..=100 {
            let data = data.clone();
            thread::spawn(move || {
                data.fetch_add(1, Ordering::Relaxed);
            });
        }

        thread::sleep(time::Duration::from_secs(1));
        println!("(2) Got data: {}", data.clone().load(Ordering::Relaxed));
        assert_eq!(100, data.load(Ordering::Relaxed));
    }

    // (5) Atomic 能替代锁吗???
    /*
       那么原子类型既然这么全能, 它可以替代锁吗？答案是不行：
       1.对于复杂的场景下，锁的使用简单粗暴，不容易有坑
       2.std::sync::atomic包中仅提供了数值类型的原子操作
         AtomicBool, AtomicIsize, AtomicUsize, AtomicI8, AtomicU16等, 而锁可以"应用于各种类型"
       3.在有些情况下必须使用锁来配合,例如使用Mutex配合Condvar
    */

    // (6) Atomic 的应用场景:
    /*
       事实上 `Atomic` 虽然对于用户不太常用, 但是对于高性能库的开发者、标准库开发者都非常常用
       它是并发原语的基石, 除此之外还有一些场景适用:
       1."无锁"(lock free): 不用加锁的数据结构
       2."全局变量": 例如全局自增 ID, 在后续章节会介绍
       3."跨线程计数器": 例如可以用于统计指标
    */
}
