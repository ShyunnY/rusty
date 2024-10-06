use core::time;
use std::{
    rc::Rc,
    sync::Arc,
    thread::{self, sleep},
};

#[allow(dead_code)]
pub fn hello() {
    // Rc和Arc的故事
    // Rust 所有权机制"要求一个值只能有一个所有者"
    /*
       但是考虑以下情况:
       在图数据结构中, 多个边可能会拥有同一个节点, 该节点直到没有边指向它时, 才应该被释放清理
       在多线程中多个线程可能"会持有同一个数据", 但是你受限于 Rust 的安全机制, "无法同时获取该数据的可变引用"
    */
    // Rust 在所有权机制之外又引入了额外的措施来简化相应的实现：通过引用计数的方式
    // "允许一个数据资源在'同一时刻拥有多个所有者'"(注意是拥有多个所有者)
    // 这种实现机制就是 Rc 和 Arc! 前者适用于单线程, 后者适用于多线程

    // (1) Rc<T>
    // 引用计数(reference counting), 顾名思义:
    // 通过记录一个数据"被引用的次数来确定该数据是否正在被使用"
    // 当引用次数"归零时", 就代表该数据不再被使用, 因此可以被清理释放
    // 而 Rc 正是引用计数的英文缩写。
    // 当我们希望在堆上"分配一个对象供程序的多个部分使用且无法确定哪个部分最后一个结束时"
    // "就可以使用 Rc 成为数据值的所有者"
    //
    // rust的所有权就是用于管理内存的生命周期, 而Rc通过引用的方式来避免数据被清理
    // 如果我们使用简单的借用, 那么我们需要考虑借用的数据的生命周期(避免悬挂指针)
    // "Rc让我们能够安全的引用数据, 不需要担心数据在借用阶段会被清理的问题"
    {
        // 智能指针 Rc<T> 在创建时, 还会将内部的引用计数加 1
        let s = String::from("xxx");
        let x = Rc::new(s); // rc + 1
        println!("rc x 的引用计数: {}", Rc::strong_count(&x));
        let _y = x.clone(); // rc + 1
        println!("rc x 的引用计数: {}", Rc::strong_count(&x));
    }

    // (2) Rc::clone
    // 不要被 clone 字样所迷惑, 以为所有的 clone 都是深拷贝!
    // 这里的 clone "仅仅复制了智能指针并增加了引用计数", 并没有克隆底层数据
    // 因此 x 和 y 是"共享"了底层的字符串 s, 这种复制效率是非常高的
    // 当然你也可以使用 a.clone() 的方式来克隆
    // 但是从可读性角度，我们更加推荐 `Rc::clone` 的方式
    // 核心: "仅仅是复制了指针以及增加了Rc内部cell的引用计数器!"

    // (3) 观察引用计数的变化
    // a、b、c 三个智能指针引用计数都是同样的, 并且共享底层的数据
    // 当 a、b 超出作用域后, "引用计数会变成 0"
    // 最终"智能指针和它指向的底层字符串都会被清理释放"
    {
        let a = Rc::new(String::from("xxoo"));
        let b = Rc::clone(&a);
        println!("create c before: {}", Rc::strong_count(&b)); // rc=2
        {
            let c = Rc::clone(&b);
            println!("create c: {}", Rc::strong_count(&c)); // rc=3
        } // c是智能指针Rc且实现了Drop trait, 当超出作用域会自动调用Drop使得引用计数器-1
        println!("create after c: {}", Rc::strong_count(&b)); // rc=2
    }

    // (4) 不可变的引用
    // 事实上: Rc<T> 是指向底层数据的"不可变的引用", 因此你无法通过它来修改数据
    // 这也符合 Rust 的借用规则: 要么存在多个不可变借用, 要么只能存在一个可变借用
    //
    // 但是实际开发中我们往往需要对数据进行修改, 这时单独使用 Rc<T> 无法满足我们的需求
    // 需要配合其它数据类型来一起使用, 例如内部可变性的 RefCell<T> 类型以及互斥锁 Mutex<T>

    // (5) 一个例子
    // 原因是在 drop 之前, 存在三个指向 Foo 的智能指针引用
    // 我们仅仅 drop 掉其中一个智能指针引用, 而不是 drop 掉真实的数据
    // 外面还有两个引用指向底层的 owner 数据, 引用计数"尚未清零"
    {
        struct Foo;

        struct Bar {
            f: Rc<Foo>,
        }

        let f: Rc<Foo> = Rc::new(Foo);
        let bar_1 = Bar { f: Rc::clone(&f) };
        let bar_2 = Bar { f: Rc::clone(&f) };

        // 此时即使我们手动drop了f, bar_1和bar_2依旧持有不可变引用, 并且数据没有被销毁
        drop(f);
        println!("bar_1 strong count: {:?}", Rc::strong_count(&bar_1.f));
        println!("bar_2 strong count: {:?}", Rc::strong_count(&bar_2.f));
    }

    // (5) Rc的小总结
    /*
    Rc 简单总结:
    1. Rc/Arc "是不可变引用", 你无法修改它指向的值,只能进行读取.
       如果要修改需要配合后面章节的内部可变性 RefCell 或互斥锁 Mutex
    2. 一旦最后一个拥有者消失(引用计数清零时), 则资源会自动被回收
       这个生命周期是在编译期就确定下来的
    3. Rc 只能用于"同一线程内部", 想要用于线程之间的对象共享你需要使用`Arc`
    4. Rc<T> 是一个智能指针, 实现了 Deref 特征, 因此你无需先解开 Rc 指针
       再使用里面的 T, 而是可以直接使用 T
    5. Rc实现了Drop, 当超出作用域时将会自动调用 Rc::drop 将引用计数减少
     */

    // (6) 多线程中的无力Rc
    // Rc<T> 不能在线程间安全的传递, 实际上是因为它没有实现 Send 特征
    // 而`Send`特征是恰恰是多线程间传递数据的关键!
    // 当然还有更深层的原因：由于 Rc<T> 需要管理引用计数, 但是该计数器并没有使用任何并发原语
    // 因此无法实现原子化的计数操作, 最终会导致计数错误

    // (7) Arc
    // Arc 是`Atomic Rc`的缩写, 顾名思义: "原子化的 Rc<T> 智能指针"
    // 原子化是一种并发原语, Arc也许是在Rc的基础上为管理引用计数添加了并发原语
    {
        let msg = Arc::new(String::from("xxoo"));
        for _ in 1..=3 {
            let tmp = Arc::clone(&msg);
            let _h = thread::spawn(move || println!("多线程执行: {}", tmp));
        }
        sleep(time::Duration::from_millis(200)); // sleep for concurency exec
    }

    // 一个小总结:
    /*
       在Rust中, 所有权机制保证了"一个数据只会有一个所有者"
       Rust提供了Rc/Arc让数据看起来拥有了多个所有者(其实是引用啦!)

       Rc 和 Arc 的区别在于, 后者是原子化实现的引用计数, 因此是线程安全的可以用于多线程中共享数据
       这两者都是"***只读***"的, 如果想要实现内部数据可修改
       (必须配合内部可变性 RefCell 或者互斥锁 Mutex 来一起使用)
    */
}
