pub fn _hello() {
    println!("===> 生命周期高级篇");

    // (1). 不太聪明的生命周期检查
    // 1. 例子一
    // 我们先调用了 "mutable", 然后在调用 "unmutable"
    // 正常来说, 我们应该是可以调用的. 看起来可变借用只是在 "mutable"函数内生效, 为什么会导致外部也进行了可变借用呢?
    //
    // 原理就是: 若存在多个输入生命周期, 且其中一个是 &self 或 &mut self, 则 &self 的生命周期被赋给所有的输出生命周期
    // 也就是说, mutable参数引用的生命周期被赋于给了返回值上, 所以此时 ff 的引用其实是 "&mut f"的引用
    // 因为 ff 引用生命周期在 mutable 中就被参数赋予了
    // 这就解释了可变借用为啥会在 main 函数作用域内有效, 最终导致 f.unmutable() 无法再进行不可变借用
    // 因为在作用域内: 违背了可变借用与不可变借用不能同时存在的规则
    {
        #[derive(Debug)]
        struct Foo;

        impl Foo {
            #[allow(dead_code)]
            fn mutable(&mut self) -> &Self {
                &*self
            }

            #[allow(dead_code)]
            fn unmutable(&self) {}
        }

        let mut f = Foo;
        let ff: &Foo = f.mutable(); // => Foo::mutable(&mut f), ff的引用生命周期就继承了f引用生命周期, 所以ff此时是不可变引用
                                    // f.unmutable();
        println!("{:?}", ff);
    }

    // (2) 无界生命周期
    // 不安全代码(unsafe)经常会凭空产生引用或生命周期, 这些生命周期被称为是"无界(unbound)"的
    // 无界生命周期往往是在解引用一个裸指针(裸指针 raw pointer)时产生的,
    // 换句话说: 它是"凭空产生"的, 因为"输入参数根本就没有这个生命周期" (输入参数根本就不存在这个生命周期啊!)
    //
    // 我们在实际应用中, 要尽量避免这种无界生命周期.
    // 最简单的避免无界生命周期的方式就是"在函数声明中运用生命周期消除规则"
    // 若一个输出生命周期被消除了, 那么必定因为"有一个输入生命周期与之对应"
    // 也就是说: 输出生命周期和输入生命周期是相关联的
    {
        #[allow(dead_code)]
        fn f<'a, T>(x: *const T) -> &'a T {
            // 参数 x 是一个裸指针, 它并没有任何生命周期.
            // 然后通过 "unsafe" 操作后, 它被进行了解引用, 变成了一个 Rust 的标准引用类型.
            // 该类型必须要有生命周期, 也就是 'a
            //
            // 可以看出 'a 是凭空产生的, 因此它是无界生命周期.
            // 这种生命周期由于没有受到任何约束, 因此它想要多大就多大, 这实际上比 'static 要强大
            unsafe {
                // 先对这个裸指针x进行解引用, 然后再返回一个引用(这谁知道这个x的生命周期是多长啊！)
                &*x
            }
        }
    }

    // (3) 生命周期约束 HRTB
    // 在 Rust 中, 生命周期（lifetimes）是一种"泛型类型", 用于描述多个引用之间的关系, 确保引用在需要时有效.
    // 生命周期的主要目的是"防止悬垂引用"（dangling references）, 即引用超出其指向数据的生命周期.
    // "生命周期约束跟特征约束类似", 都是通过形如 'a: 'b 的语法, 来说明"两个生命周期的长短关系"
    // 'a: 'b  这说明 a 最起码跟 b 一样命长, 用公式表达等于 "a >= b"
    //
    {
        // 在这个结构体中, 同时声明生命周期参数和泛型(其实生命周期参数也是一种泛型参数哦~)
        // x,y 都拥有自己的生命周期约束, 'b: 'a 说明 **"b>=a"**
        // 结构体字段x引用了T, 因此r的生命周期'a 必须要比T的生命周期更短(被引用者的生命周期必须要比引用长)
        // 我们想想, 如果引用者的生命周期比被引用者长, 那不就成了垂悬引用了吗!
        #[allow(dead_code)]
        struct DoubleRef<'a, 'b: 'a, T> {
            x: &'a T,
            y: &'b T,
        }

        // 再来看一个例子:
        #[allow(dead_code)]
        struct Foo<'a> {
            msg: &'a str,
        }
        // 我们将 self.msg 的生命周期设置为 _anno 的 'b
        // 如果我们想编译通过, 必须添加上 'a: 'b
        // 这代表生命周期关系: a>=b (a的命起码要大于等于b)
        // 仔细想想: 如果我们不进行约束. 用户以为 self.msg 跟 'b 一样对其进行使用, 结果 'a 提前释放了
        // 此时不就成为了 非法引用, 垂悬引用了？
        impl<'a: 'b, 'b> Foo<'a> {
            #[allow(dead_code)]
            fn bar(&self, _anno: &'b str) -> &'b str {
                self.msg
            }
        }
    }

    // (4) 闭包函数的消除规则
    //
    {
        // 先看个例子, 明明是一样的作用, 只是一个是函数实现另外一个是闭包实现, 为什么后者无法编译？
        // 按照生命周期第一法则: 只有一个引用参数, 那么返回值生命周期与引用参数生命周期一致
        // 为啥在闭包内就失效了？
        // 直接给出原因:
        // 这个问题可能很难被解决, 建议大家遇到后, 还是老老实实用正常的函数, 不要秀闭包了
        // 对于函数的生命周期而言, 它的消除规则之所以能生效是因为它的"生命周期完全体现在签名的引用类型上"
        // 在函数体中无需任何体现
        //
        // 因此编译器可以做各种编译优化, 也很容易根据参数和返回值进行生命周期的分析, 最终得出消除规则
        // 可是闭包并没有函数那么简单, 它的生命周期分散在参数和闭包函数体中(主要是它没有确切的返回值签名)
        #[allow(dead_code)]
        fn func(x: &i32) -> &i32 {
            &x
        }
        // let closure = |x: &i32| -> &i32{
        //     x
        // }

        // Rust 语言开发者目前其实是有意"针对函数和闭包实现了两种不同的生命周期消除规则"
        // 其实也有方法解决滴, 就是使用 Fn trait 来解决
        //
        // 在 Fn trait 中
        fn fun<T, F>(func: F) -> F
        where
            F: Fn(&T) -> &T,
        {
            func
        }

        let result = fun(|x: &i32| -> &i32 { x });
        assert_eq!(*result(&10), 10);
    }

    // (5) NLL(Non-Lexical Lifetime)
    // 简单来说就是: 引用的生命周期正常来说应该从借用开始一直持续到作用域结束, 但是这种规则会让多引用共存的情况变得更复杂
    // 所以NLL其实就是: 引用的生命周期从借用处开始, 一直"持续到最后一次使用"的地方(称其为非词法生命周期)
    {
        let mut msg = String::new();
        let a = &msg;
        let b = &msg;
        println!("{a}-{b}"); // 实际上 a,b 的生命周期在此就会结束了, 因为后续并没有对其进行使用

        let c = &mut msg;
        println!("{c}"); // c 的生命周期在此结束, 后续也没有使用 c
    }

    // (6) Reborrow 再借用
    // 再借用本质上是 Rust 编译器在处理函数调用或代码块时, 自动创建临时借用的过程.
    // 再借用允许你在保持原有借用有效的同时, 临时创建一个新的借用
    // 我们先来看一个例子
    {
        #[allow(dead_code)]
        #[derive(Debug)]
        struct Point {
            x: i32,
            y: i32,
        }

        impl Point {
            #[allow(dead_code)]
            fn move_to(&mut self, x: i32, y: i32) {
                self.x = x;
                self.y = y;
            }
        }

        let mut point = Point { x: 1, y: 2 };
        let p: &mut Point = &mut point;
        let pp: &Point = &*p; // 这里通过对p进行了解引用和再借用, 此时在pp的生命周期内不能使用p(因为p是可变借用)
        println!("{:?}", pp);
        // 对于再借用而言, pp 再借用时不会破坏借用规则, 但是你不能在它的生命周期内再使用原来的借用 p
        p.move_to(3, 4);
    }

    // (7)生命周期消除规则补充
    // 除了三大基础生命周期消除规则, 其实rust还提供了其他消除规则
    //
    // 1. impl块消除
    {
        // impl<'a> Reader for BufReader<'a> {
        //     // methods go here
        //     // impl内部实际上没有用到'a
        // }
        //
        // 我们可以写成以下形式, 通过 '_ 告诉编译器, 我们并没有使用到生命周期
        // 注意: 生命周期参数也是类型的一部分(他是个泛型参数), 在实现的过程中, 不能把类型给丢了!
        // impl Reader for BufReader<'_> {
        //     // methods go here
        // }
    }

    // (8) 一个复杂的例子
    {
        #[allow(dead_code)]
        struct Interface<'b, 'a: 'b> {
            manager: &'b mut Manager<'a>,
        }

        #[allow(dead_code)]
        impl<'b, 'a: 'b> Interface<'b, 'a> {
            pub fn noop(self) {
                println!("interface consumed");
            }
        }

        #[allow(dead_code)]
        struct Manager<'a> {
            text: &'a str,
        }

        #[allow(dead_code)]
        struct List<'a> {
            manager: Manager<'a>,
        }

        #[allow(dead_code)]
        impl<'a> List<'a> {
            // 我们给予了一个额外的生命周期参数 'b
            // 这个'b 给予了 Interface 中的 manager, 'a 给了 Interface 中 manager 的text
            // 所以此时 Interface 对 manager 的可变借用跟list不挂钩了, 之前是挂钩的, 因为用的都是 'a
            pub fn get_interface<'b>(&'b mut self) -> Interface<'b, 'a>
            where
                'a: 'b,
            {
                Interface {
                    manager: &mut self.manager,
                }
            }
        }

        #[allow(dead_code)]
        fn xx() {
            let mut list = List {
                manager: Manager { text: "hello" },
            };

            list.get_interface().noop();

            println!("Interface should be dropped here and the borrow released");

            // 下面的调用可以通过，因为Interface的生命周期不需要跟list一样长
            use_list(&list);
        }

        // 因为 use_list 对 List 进行了不可变借用, 并且使用到其内部的 manager
        // 必须保证 manager 此时是不存在可变借用的
        // 如果按照原来的 'a , 那么此时 Interface 中的 manager 还是持有跟list一样长的可变借用, 这违反了借用法则
        // 所以给予了 'b 之后, Interface 中的 manager 没有持有这么长的可变借用了, 此时可以用
        fn use_list(list: &List) {
            println!("{}", list.manager.text);
        }
    }
}
