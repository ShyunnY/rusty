use std::{array::IntoIter, collections::HashMap, vec};

#[allow(dead_code)]
pub fn hello() {
    // 迭代器 Iterator

    // (1) for循环与迭代器
    // 从用途来看, 迭代器跟 for 循环颇为相似, 都是去遍历一个集合
    // 但是实际上它们存在不小的差别, 其中最主要的差别就是: "是否通过索引来访问集合"
    //
    // Rust中没有使用索引, 它把 arr 数组当成一个迭代器, 直接去遍历其中的元素
    // 从哪里开始, 从哪里结束, 都无需操心.
    // 因此严格来说, Rust 中的 for 循环是编译器提供的语法糖, "最终还是对迭代器中的元素进行遍历"
    {
        // 数组实现了 IntoIterator 特征, Rust 通过 for 语法糖
        // 自动把"实现了该特征的数组类型转换为迭代器"（你也可以为自己的集合类型实现此特征）
        // 也就是说, rust只是将其转换成迭代器方式进行遍历
        for ele in 1..=3 {
            println!("rust通过迭代器遍历: {ele}");
        }
    }

    // (2) 惰性初始化
    // 在 Rust 中, 迭代器是惰性的, 意味着如果你不使用它, 那么它将不会发生任何事
    // 这种惰性初始化的方式确保了创建迭代器不会有任何额外的性能损耗, 其中的元素也不会被消耗
    // "只有使用到该迭代器的时候, 一切才开始"
    {
        let arr = vec![11, 22, 33];
        let arr_iter = arr.iter(); // 创建了一个迭代器, 但是啥事都不会发生
        for &ele in arr_iter {
            // 此刻开始遍历, 才会让迭代器开始迭代
            println!("惰性初始化, 现在才生效 ele: {}", ele);
        }
    }

    // (3) next 方法
    // 对于 for 如何遍历迭代器, 还有一个问题, 它如何取出迭代器中的元素？
    // 答案就是通过 next 方法来取出元素:
    {
        // 我们可以看到 Iterator 的源码, 其通过 next 方法返回关联类型 Item
        #[allow(dead_code)]
        trait Iterator {
            type Item;
            // 注意: 需要可变引用, 迭代器的消耗是可变的
            fn next(&mut self) -> Option<Self::Item>;

            // 省略其余有默认实现的方法
        }
    }
    // 迭代器之所以成为迭代器, 就是因为"实现了 Iterator 特征"
    // 要实现该特征, 最主要的就是"实现其中的 next 方法", 该方法控制如何从集合中取值, 最终返回值的类型是关联类型 Item
    {
        // 我们可以手动调用next方法消耗元素
        let arr = vec![12, 23, 34];
        let mut arr_iter = arr.into_iter(); // iter具有所有权的, 同时需要声明为mut可变
        assert_eq!(Some(12), arr_iter.next());
        assert_eq!(Some(23), arr_iter.next());
        assert_eq!(Some(34), arr_iter.next());
        assert_eq!(None, arr_iter.next());

        // 果不其然, 将 arr 转换成迭代器后, 通过调用其上的 next 方法, 我们获取了 arr 中的元素
        // 有三点需要注意:
        //  * next 方法返回的是 Option 类型，当"有值时返回 Some(i32)，无值时返回 None"
        //  * 遍历是 "按照迭代器中元素的排列顺序" 依次进行的，因此我们严格按照数组中元素的顺序取出了 Some(1)，Some(2)，Some(3)
        //  * 手动迭代必须将迭代器"声明为 mut 可变"，因为调用 next "会改变迭代器其中的状态数据（当前遍历的位置等）"
        //    "而 for 循环去迭代则无需标注 mut，因为它会帮我们自动完成"(for的语法糖帮我们将迭代器标注为 mut 了)
        //  * 总之，next 方法对迭代器的遍历是"消耗性的"，每次消耗它一个元素，最终迭代器中将没有任何元素，只能返回 None。
    }
    // 那么我们也可以模拟一个迭代器来实现遍历获取元素
    {
        let arr = vec![999, 888, 777];

        // 这里进行了一个模式匹配, match匹配了一个变量值并将其绑定到 iter 上
        let ret = match arr.into_iter() {
            mut iter => loop {
                // 通过 loop 循环不断消耗迭代器中的元素, 直到用完
                match iter.next() {
                    Some(val) => println!("arr: {val}"),
                    None => break,
                }
            },
        };
        ret
    }
    {
        // 这个例子解释了上面的语法
        fn add() -> i32 {
            1
        }
        match add() {
            result => {
                println!("{}", result);
            }
        }
    }

    // (4) IntoIterator trait
    // 如果本身就是一个迭代器, 该怎么办？ 实际上, 迭代器自身也实现了 IntoIterator, 标准库早就帮我们考虑好了
    {
        // 摘选自 IntoIterator
        // #[inline]
        // fn into_iter(self) -> I {
        //     self
        // }

        // 所以可以出现以下奇怪的代码
        let arr = vec![111, 222, 333, 444];
        for ele in arr.into_iter().into_iter().into_iter() {
            println!("奇怪的 ele: {ele}");
        }
    }
    //
    // iterator三剑客: into_iter, iter, iter_mut
    // 这三者的区别:
    //  * into_iter 会夺走集合的所有权和集合中元素的所有权
    //  * iter 仅对集合以及集合中的元素进行借用
    //  * iter_mut 仅对集合以及集合中的元素进行可变借用
    // 知识点: 集合中的元素是属于集合的, 所以迭代器想要拿走元素的所有权就必然需要先拿走集合的所有权。 借用和不可变借用同理
    {
        // 1. into_iter
        let vals = vec![1, 2, 3];
        for ele in vals.into_iter() {
            println!("into_iter 夺走了集合和集合元素所有权, 值为{ele}");
        }
        // println!("{:?}", vals);  // 此时不可再用

        // 2. iter
        let vals = vec![4, 5, 6];
        for ele in vals.iter() {
            println!("iter 对集合和集合元素进行了不可变借用, 值为{ele}");
        }

        // 3. iter_mut
        let mut vals = vec![7, 8, 9];
        for ele in vals.iter_mut() {
            println!("iter 对集合和集合元素进行了可变借用, 值为{ele}");
        }

        // 有两点需要注意一下:
        //  * .iter() 方法实现的迭代器, 调用 next 方法返回的类型是 "Some(&T)"(不可变借用)
        //  * .iter_mut() 方法实现的迭代器, 调用 next 方法返回的类型是 "Some(&mut T)"(可变借用)
    }
    // Iterator 和 IntoIterator 的区别
    // 这两个其实还蛮容易搞混的, 但我们只需要记住:
    //  * Iterator 就是迭代器特征, 只有实现了它才能称为迭代器, 才能调用 next (Iterator trait 代表了迭代器)
    //  * 而 IntoIterator 强调的是某一个类型如果实现了该特征, 它可以通过 into_iter, iter 等方法变成一个迭代器(转换使用而已)
    // 一句话概括: **Iterator和IntoIterator分别翻译为"迭代器"和"可迭代对象"**

    // (5) 消费者适配器和迭代器适配器
    // 1. 消费者适配器
    // 消费者是迭代器上的方法, 它"会消费掉迭代器中的元素, 然后返回其类型的值".(一般用于收尾工作, 将迭代器上所有元素进行消费)
    // 这些消费者都有一个共同的特点: 在它们的定义中, 都依赖 next 方法来消费元素
    // 因此这也是为什么迭代器要实现 Iterator 特征, 而该特征必须要实现 next 方法的原因
    {
        // 只要迭代器上的某个方法 A 在其内部调用了 next 方法, 那么 A 就被称为消费性适配器:
        // 因为 next 方法会消耗掉迭代器上的元素, 所以方法 A 的调用也会消耗掉迭代器上的元素
        // 其中一个例子是 sum 方法, 它会"拿走迭代器的所有权", 然后通过"不断调用 next 方法对里面的元素进行求和"
        {
            // 注意: 内部调用 next 的方法都叫消费者, 因为他消费了集合内部的元素嘛
            let arr = vec![1, 2, 3];
            let arr_iter = arr.iter();
            let result: i32 = arr_iter.sum();
            println!("sum result: {}", result);
            // 注意! 此时不能使用 arr_iter 了, 因为它的所有权被 sum 消费者拿走了
            // println!("iter: {:?}", arr_iter);
        }
    }
    //
    // 2. 迭代器适配器
    // 既然消费者适配器是消费掉迭代器, 然后返回一个值.
    // 那么迭代器适配器, 顾名思义: "会返回一个新的迭代器". 这是实现链式方法调用的关键：`v.iter().map().filter()...`
    // 与消费者适配器不同, 迭代器适配器"是惰性的", 意味着你"需要一个消费者适配器来收尾", 最终将迭代器转换成一个具体的值
    {
        let arr = vec![1, 2, 3];
        // 这里使用了 迭代器适配器map, 由于迭代器适配器是惰性的, 我们必须通过消费者适配器来执行真正的消费动作
        // 才能够使得迭代器适配器执行
        // Vec<_> 意味着告诉编译器帮我们进行推导
        let result: Vec<_> = arr.iter().map(|x| x + 1).collect();
        println!("result: {:?}", result);
    }
    //
    // 3. collect
    // collect() 方法就是一个消费者适配器, 使用它可以将一个迭代器中的元素收集到指定类型中
    // 由于collect太强大了, 可以收集成很多类型. 所以我们需要进行类型标注, 告诉rust编译期我们想收集成什么类型
    {
        // zip 是一个迭代器适配器, 它的作用就是将两个迭代器的内容压缩到一起
        // 形成 Iterator<Item=(ValueFromA, ValueFromB)> 这样的新的迭代器
        // 在此处就是形如 [(name1, age1), (name2, age2)] 的迭代器
        let names = vec!["lty", "shyunny"];
        let age = vec![21, 21];
        // 同样的这里必须显式声明类型, 然后 HashMap 内部的 KV 类型可以交给编译器去推导
        // 最终编译器会推导出 HashMap<&str, i32>, 完全正确！
        let result: HashMap<_, _> = names.into_iter().zip(age.iter()).collect();
        println!("hashMap result: {:#?}", result);
    }
    // 4. 闭包作为适配器参数
    // 我们还可以使用闭包来作为迭代器适配器的参数, 它最大的好处不仅在于可以就地实现迭代器中元素的处理
    // "还在于可以捕获环境值"
    {
        let mut arr = vec![1, 2, 3];
        let cursor = 2;
        let result: Vec<_> = arr
            .iter_mut()
            .map(|x| {
                if *x == cursor {
                    *x += 1
                }
                x
            })
            .collect();

        println!("closure in iter: {:?}", result);
    }

    // (6) 实战: 让自定义类型实现一个 Iterator
    {
        #[derive(Debug)]
        struct Counter {
            data: u32,
        }

        impl Counter {
            fn new() -> Self {
                Counter { data: 0 }
            }
        }

        impl Iterator for Counter {
            type Item = u32;

            fn next(&mut self) -> Option<Self::Item> {
                if self.data < 10 {
                    self.data += 1;
                    Some(self.data)
                } else {
                    None
                }
            }
        }

        let mut counter = Counter::new();
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));

        // 实现 Iterator 特征的其它方法
        // 可以看出, 实现自己的迭代器非常简单, 但是 Iterator 特征中不仅仅是只有 next 一个方法
        // 那为什么我们只需要实现 Iterator 呢？
        // "因为其它方法都具有默认实现, 所以无需像 next 这样手动去实现"
        // 而且这些默认实现的方法其实都是基于 next 方法实现的

        // 流程如下:
        // 1. into_iter将 Counter 转换为一个迭代器
        // 2. filter 迭代器适配器惰性执行, 当消费者适配器调用 next 时才会开始作用
        // 3. sum 消费者适配器, 真正执行 next 动作并且调用 迭代器适配器
        let counter = Counter::new();
        let result: u32 = counter.into_iter().filter(|x| *x != 1).sum();
        println!("result: {}", result);
    }

    // (7) enumerate, 用于获取迭代器中数据的索引和值
    // 在之前的流程控制章节, 针对 for 循环, 我们提供了一种方法可以获取迭代时的索引
    // 学习当前迭代器内容后, 相信你有了船新的理解
    //  1. 首先 arr.iter() 创建迭代器
    //  2. 其次 调用 Iterator 特征上的方法 enumerate, 该方法产生一个新的迭代器, 其中每个元素均是元组 (索引，值)
    //  3. for循环隐式调用 next 进行迭代, 直到返回 None 就结束迭代
    {
        let arr = [0, 1, 2, 3];
        // 看起来好麻烦哦
        for ele in arr.iter().enumerate() {
            println!("index={},val={}", ele.0, ele.1);
        }

        // 因为 enumerate 是迭代器适配器, 因此我们可以对它返回的迭代器调用其它 Iterator 特征方法
        let result: i32 = arr
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != 0)
            .map(|(_, v)| v)
            .sum();
        println!("result: {}", result);
    }

    // (8) 迭代器的性能
    // 作为程序员, 对代码的性能肯定是有一定的偏爱
    // 前面提到, 要完成集合遍历, 既可以使用 for 循环也可以使用迭代器
    // 那么二者之间该怎么选择呢, 性能有多大差距呢？
    {
        // 直接上测试结果:
        /*
           test bench::bench_for  ... bench:     998,331 ns/iter (+/- 36,250)
           test bench::bench_iter ... bench:     983,858 ns/iter (+/- 44,673)
        */

        // 迭代器是 Rust 的 零成本抽象（zero-cost abstractions）之一, 意味着"抽象并不会引入运行时开销"
        // 编译器还可以通过循环展开（Unrolling）、向量化、消除边界检查等优化手段, 使得迭代器和 for 循环都有极为高效的执行效率.
        // 所以请放心大胆的使用迭代器, 在获得更高的表达力的同时, 也不会导致运行时的损失, 何乐而不为呢！
        // 一个题外话: Rust在语言的抽象设计上, 注入了很多零成本抽象实现方式, 这大大的减少了 Runtime 开销, 提升了很大的性能
    }
}
