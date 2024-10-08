use std::fmt::Debug;

#[allow(dead_code)]
pub fn hello() {
    // Box智能指针
    /*
       由于 Box 是简单的封装, 除了"将值存储在堆上外", 并没有其它性能上的损耗
       而性能和功能往往是鱼和熊掌不可兼得, 因此 Box 相比其它智能指针功能较为单一, 可以在以下场景中使用它:
       1.特意的将数据分配在堆上
       2.数据较大时，又不想在转移所有权时进行数据拷贝
       3.类型的大小在编译期无法确定，但是我们又需要固定大小的类型时
       4.特征对象，用于说明对象实现了一个特征，而不是某个特定的类型
    */

    // (1) 主动将数据分配到堆上
    // 智能指针往往都实现了 Deref 和 Drop 特征
    {
        // 将 i32=3 分配到堆中
        // 创建一个智能指针指向了存储在堆上的 3, 并且 x 持有了该指针
        let x = Box::new(3);
        println!("box heap i32: {}", x); // 这里自动调用了 Deref, 所以我们可以直接打印数据
        println!("box heap i32 manul deref: {}", *x + 3); // 这里没有调用Deref, 所以我们需要手动解引用

        // ===> 由于智能指针实现了 Drop trait, 所以超出此作用域将会被自动释放
    }
    // 将一个简单的值分配到堆上并没有太大的意义. 将其分配在栈上, 由于寄存器、CPU 缓存的原因, 它的性能将更好

    // (2) 避免栈上数据的拷贝
    // 当栈上数据转移所有权时, 实际上是把数据拷贝了一份, 最终新旧变量各自拥有不同的数据, 因此所有权并未转移(Copy了一份, 所有权并未转换)
    // 而堆上则不然, 底层数据"并不会被拷贝", "转移所有权仅仅是复制一份栈中的指针, 再将新的指针赋予新的变量"
    // 然后让拥有旧指针的变量失效, 最终完成了所有权的转移(Move过程, 将所有权进行移动)
    {
        // 栈上分配的基本数据实现了Copy特征, 当数据过大时, 这是一个很耗时的过程...

        let arr: [i32; 3] = [1, 2, 3]; // 栈内分配的数组
        let arr_copy = arr; // 将其copy一份给 arr_copy (此时并未发生所有权变更)
        println!("arr: {:?}, arr_copy: {:?}", arr, arr_copy);

        // 创建一个放在堆中的数组, 此时arr_heap具有引用和所有权
        let arr_heap = Box::new([1, 2, 3]);
        // 由于数据在堆上, 因此仅仅拷贝了智能指针的结构体, 底层数据并没有被拷贝. 所有权进行移动了
        let arr_heap_copy = arr_heap;
        println!("arr heap: {:?}", arr_heap_copy);
        // println!("{:?}", arr_heap);
    }
    // 大块的数据为何应该放入堆中, 此时 Box 就成为了我们最好的帮手

    // (3) 将动态大小类型变为 Sized 固定大小类型
    // Rust 需要在编译时知道类型占用多少空间, 如果一种类型在编译时无法知道具体的大小, 那么被称为动态大小类型 DST
    {
        // 其中一种无法在编译时知道大小的类型是递归类型: 在类型定义中又使用到了自身, 或者说该类型的值的一部分可以是相同类型的其它值
        // 这种值的嵌套理论上可以无限进行下去, 所以 Rust 不知道递归类型需要多少空间
        // 例如以下代码, rust不能确定List的Sized是多大
        // enum List {
        //     Cons(List),
        //     None,
        // }

        // 我们可这样解决:
        // 只需要将 List 存储到堆上, 然后使用一个智能指针指向它, 即可完成从 DST 到 Sized 类型(固定大小类型)的华丽转变
        // 因为智能指针是具有Sized的!!!
        enum List {
            Cons(u32, Box<List>), // 使用Box包裹即可～
            None,
        }
    }

    // (4) 特征对象
    // 在 Rust 中, 想实现不同类型组成的数组只有两个办法: 枚举和特征对象.
    // 前者限制较多，因此后者往往是最常用的解决办法
    {
        // 我们想存放不同真实类型的特征对象
        let _var: Vec<Box<dyn Debug>> = vec![Box::new(1), Box::new("hello")];
    }
    // 其实特征也是 DST 类型, 而特征对象在做的就是将 DST 类型转换为固定大小类型

    // (5) Box的内存布局
    // Box在栈上其实就是以一个结构体形式存储, 结构体内部存在一个指针指向堆上真正的数据.
    {
        let arr: Vec<Box<i32>> = vec![Box::new(1), Box::new(2), Box::new(3)];
        for ele in arr.iter() {
            // 由于rust不会为表达式自动添加Deref, 所以我们需要进行手动解引用
            // 第一次解引用将 &Box => Box
            // 第二次解引用将 Box => i32
            println!("box vec ele: {}", **ele + 1);
        }
    }

    // (6) Box::leak 主动泄漏内存
    /*
       Q: 什么是内存泄漏？
       A: 内存泄漏是指程序在分配内存后, 由于某些原因"未能释放不再使用的内存", 导致这部分"内存无法被操作系统回收"
          从而造成内存资源的浪费. 在长时间运行的程序中, 内存泄漏可能导致可用内存逐渐减少, 最终影响程序性能甚至导致程序崩溃
          简而言之: 就是我希望这部分内存rust不要帮我管理啦, 使其不要被回收哦
    */
    // Box 中还提供了一个非常有用的关联函数: Box::leak, 它可以"消费掉 Box 并且强制目标值从内存中泄漏"
    {
        #[derive(Debug)]
        struct Config {
            app_name: String,
            version: String,
        }

        // 在之前的学习中, 如果我们希望返回一个输出引用, 那必定至少需要有一个输入引用.
        // 我们无法使其引用栈内数据（悬挂引用）
        //
        // 这里我们通过 Box::leak 主动泄漏内存, 使其具有 'static 生命周期, 在整个程序生命中都可以进行引用
        // 这里的 'static 是忽悠编译期的, 真正让其具有 “static” 生命周期还得是看 Box::leak
        fn leak_config() -> &'static Config {
            let config = Box::new(Config {
                app_name: String::from("kubernetes"),
                version: String::from("1.30.0"),
            });

            Box::leak(config)
        }

        let c: &Config = leak_config();
        println!("static config: {:?}", c);
    }
    // 注意哈!!!
    // 其实你标注的 'static 只是用来"忽悠编译器"的. 一旦超出作用域, 照样被释放回收
    // 而使用 Box::leak 就可以将一个运行期的值转为 'static
    //
    /*
       光看上面的描述, 大家可能还是云里雾里、一头雾水
       那么我说一个简单的场景, 你需要一个"在运行期初始化的值, 但是可以全局有效, 也就是和整个程序活得一样久"
       那么就可以使用 Box::leak

       例如有一个Config的结构体实例, 它是在运行期动态插入配置内容, 那么就可以将其转为全局有效
       虽然 Rc/Arc 也可以实现此功能, 但是 Box::leak 是性能最高的
    */

    // 总结
    // Box 背后是调用 jemalloc 来做内存管理, 所以"堆上的空间无需我们的手动管理"
    // 与此类似, 带 GC 的语言中的对象也是借助于 Box 概念来实现的
    // "一切皆对象 = 一切皆 Box"
}
