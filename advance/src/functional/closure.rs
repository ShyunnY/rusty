use std::vec;

#[allow(dead_code)]
pub fn hello() {
    // closure 闭包
    // 闭包是一种匿名函数, 它有以下两个特点:
    // * 它可以"赋值给变量也可以作为参数传递给其它函数"
    // * 不同于函数的是, 它允许"捕获调用者作用域中的值"
    // Example:
    let x = 1;
    // 求和闭包函数赋值给了sum, 同时它还捕获了作用域中的x
    let sum = |y: i32| -> i32 { x + y };
    println!("sum: {}", sum(10));

    // Rust的闭包与函数最大的不同就是它的参数是通过 "|parm1|" 的形式进行声明, 如果是多个参数就 |param1, param2, ...|
    // 下面给出闭包的形式定义:
    /*
       |param1, param2,...| {
           语句1;
           语句2;
           返回表达式
       }

        // 如果只有一个返回表达式的话, 定义可以简化为:
        |param1| 返回表达式
    */
    //
    // 闭包函数需要注意的几个点:
    //  * 闭包中最后一行表达式返回的值, 就是闭包"执行后"的返回值(注意是执行后！执行后！执行后！)
    //  * let action = ||... 只是把闭包赋值给变量 action, 并不是把闭包执行后的结果赋值给 action
    //    因此这里 action 就相当于闭包函数, 可以跟函数一样进行调用: action()

    // (1) 闭包的类型推导
    // 闭包并不会作为 API 对外提供, 因此它可以享受编译器的类型推导能力, 无需标注参数和返回值的类型
    // 但有些时候为了可读性, 我们也会手动标注类型
    // 注意: 如果你只进行了声明, 但是没有使用. 编译器会提示你为"闭包参数和闭包返回值添加类型标注", 因为它缺乏必要的上下文
    // 虽然闭包的类型推导很好用, 但是它不是泛型. 当编译器推导出一种类型后, 它就会"一直使用该类型"(注意这个细节)
    {
        let closure = |x| x;
        let _ret = closure(String::new()); // 此时rust编译期推导出closure闭包的参数是String类型, 后续将一直使用String类型

        // let ret = closure(1);    // 无法通过编译, 因为闭包参数被推断成String类型了
    }

    // (2) 一个小例子, 构建一个使用闭包的Cache结构体
    {
        // F: Fn(i32) -> i32 意味着 db 的类型是 F, 该类型必须实现了相应的闭包特征 Fn(i32) -> i32
        // Fn(i32) -> i32 表示该闭包的参数是i32类型, 返回值是i32类型
        //
        // 需要注意的是, 其实 Fn 特征"不仅仅适用于闭包, 还适用于函数"!
        // 因此 db 字段除了使用闭包作为值外, 还能使用一个"具名的函数来作为它的值"
        struct Cache<F, V>
        where
            F: Fn(V) -> V,
            V: Copy,
        {
            db: F,
            data: Option<V>,
        }

        impl<F, V> Cache<F, V>
        where
            F: Fn(V) -> V,
            V: Copy,
        {
            fn new(db: F) -> Self {
                Self { db, data: None }
            }

            fn query(&mut self, key: V) -> V {
                match self.data {
                    Some(val) => val,
                    None => {
                        let ret = (self.db)(key); // 注意: 使用结构体内的字段闭包需要使用 "(self.field)()" 方式
                        self.data = Some(ret);
                        ret
                    }
                }
            }
        }

        let db_query = |x: i32| -> i32 {
            println!("命中数据库!");
            x + 100
        };
        let mut cache = Cache::new(db_query);
        println!("first query: {}", cache.query(100));
        println!("first query: {}", cache.query(100));
    }

    // (2) 捕获环境变量
    // 闭包还拥有一项函数所不具备的特性: 捕获作用域中的值
    {
        let x = 1;
        let sum = || -> i32 { x + 1 }; // x被闭包捕获啦！ 对于函数而言, 并不能做到
        println!("sum: {}", sum());
    }
    //
    // 当闭包从环境中捕获一个值时, "会分配内存去存储这些值".(Golang中需要通过变量逃逸来实现)
    // 对于有些场景来说, 这种额外的内存分配会成为一种负担.
    // 与之相比, 函数就不会去捕获这些环境值, 因此定义和使用函数不会拥有这种内存负担

    // (3) 三种Fn闭包特征: 你, 值得拥有!
    // 闭包捕获变量有三种途径, 恰好对应函数参数的三种传入方式: 转移所有权、可变借用、不可变借用, 因此相应的 Fn 特征也有三种
    // * 转移所有权, FnOnce: 该类型的闭包会拿走被捕获变量的所有权. Once 顾名思义, 说明该闭包只能运行一次
    // * 可变借用, FnMut: 它以可变借用的方式捕获了环境中的值, 因此可以修改该值
    // *
    //
    // 注意: 闭包也是有所有权的!!! 一定要牢记, 但是所有权取决于闭包对象捕获的变量类型:
    // 闭包自动实现Copy特征的规则是: 只要闭包捕获的类型都实现了 Copy 特征的话, 这个闭包就会默认实现Copy特征
    // 如果闭包捕获的变量没有实现 Copy trait, 那么这个闭包对象将无法 Copy.
    //
    // 做个总结:
    // * 闭包对象在传递时, 是Copy还是Move取决于闭包捕获的上下文变量
    // * 变量属于 可变引用或者所有权, 此时闭包对象的所有权就给传递过去了, 后续就不能在再次用了
    // * 变量属于 不可变引用, 此时闭包对象的Copy就传递过去了, 后续还能反复用
    {
        // FnOnce: 只能运行一次的闭包, FnOnce 只是一个约束
        // 如果真的想拿取所有权, 还是需要通过 move 关键字将所有权转移进去

        fn fn_once<F>(func: F)
        where
            F: FnOnce() + Copy, // FnOnce是拿走捕获变量的所有权, 如果想运行多次则需要添加 Copy 约束
                                // 注意: "即使 F 是 FnOnce 约束, 但是其本质上只是不可变借用, 所以该闭包是可 Copy 的"
        {
            func(); // 其实只是说闭包对象只能执行一次
            func();
        }

        let arr = vec![1, 2, 3];
        println!("origin address: {:p}", &arr);
        let demo = || println!("vec len: {}, address: {:p}", arr.len(), &arr);
        fn_once(demo);
        println!("after address: {:p}", &arr);

        // 如果你想强制闭包取得捕获变量的所有权, 可以在参数列表前添加 move 关键字
        // 这种用法通常用于闭包的生命周期大于捕获变量的生命周期时, 例如将闭包返回或移入其他线程
        let _d = move || println!("vec: {:?}, address: {:p}", arr, &arr); // 这里的arr进行了COPY
        _d();
    }

    {
        // FnMut 可变的参数捕获闭包
        fn update_closure<F>(mut func: F)
        where
            F: FnMut(),
        {
            func();
            func();
        }

        let mut msg = String::from("hello!"); // 1. 声明一个可变的字符串
        let update = || msg.push_str("closure!"); // 2. 声明一个可变的闭包对象
        update_closure(update); // 3. 将闭包对象的所有权转移进去了
        println!("msg: {}", msg);
        // 细心如你一定发现, 为什么这里我们没有将 update 闭包声明为mut, 但是依旧可以传递给 update_closure 作为参数呢？
        //
        // 事实上, FnMut只是trait的"名字", 声明变量为FnMut和要不要mut没啥关系
        // FnMut是推导出的特征类型, mut是rust语言层面的一个修饰符, 用于声明一个绑定是可变的.
        // Rust从特征类型系统和语言修饰符两方面保障了我们的程序正确运行
        // 我们在使用FnMut类型闭包时需要捕获外界的可变借用, 因此我们常常搭配mut修饰符使用
        // "但我们要始终记住, 二者是相互独立的"

        // 我们可以在看一个例子:
        println!("分割线====");
        {
            fn exec<F>(mut func: F)
            where
                F: FnMut(),
            {
                func();
            }

            let s1 = String::from("move");
            let c1 = || println!("s1: {s1}");
            exec(c1); // c1持有了所有权, 此时闭包对象的所有权也进去了exec, 所以此时闭包对象是 Clone 而不是 Copy 的

            let s2 = String::from("immutable");
            let c2 = || println!("s2: {s2}");
            exec(c2);
            exec(c2); // c2持有的是不可变引用, 此时闭包对象的Copy进去了exec, 所以此时闭包对象是可Copy的

            let mut s3 = String::from("immutable");
            let c3 = || {
                s3.push_str("string");
                println!("s3: {s3}");
            };
            exec(c3); // c3持有的是可变引用, 此时闭包对象的所有权进去了exec, 所以此时闭包对象是 Clone 而不是 Copy 的
        }
    }

    {
        // Fn 不可变的参数捕获闭包, 它以不可变借用的方式捕获环境中的值

        // 在这里, 闭包对象仅仅是以不可借用的方式使用了 str , 所以此时闭包对象是可 Copy 多次使用的
        let str = String::from("immutable");
        let closure = || println!("str: {str}");
        closure();
    }

    // (3) move 和 Fn
    // 我们讲到了 move 关键字对于 FnOnce 特征的重要性, 但是实际上使用了 move 的闭包依然可能实现了 Fn 或 FnMut 特征
    // 因为一个闭包实现了哪种 Fn 特征取决于该闭包如何"使用被捕获的变量", 而不是取决于闭包"如何捕获它们"
    // move 本身强调的就是后者, 闭包如何捕获变量
    {
        let str = String::from("move and fn");
        // closure实际上还是 Fn 而不是 FnOnce
        let closure = move || println!("str: {str}"); // 即使这里用了 move, 但是在闭包内还是使用不可变引用
        closure();
        closure();

        // 我们在上面的闭包中使用了 move 关键字, 所以我们的闭包捕获了它
        // 但是由于闭包对 s 的使用"仅仅是不可变借用", 因此该闭包实际上还实现了 Fn 特征
        // 细心的读者肯定发现我在上段中使用了一个'还'字, 这是什么意思呢？
        // "因为该闭包不仅仅实现了 FnOnce 特征, 还实现了 Fn 特征"
    }

    // (4) 三种 Fn 的关系
    // 实际上, 一个闭包并不仅仅实现某一种 Fn 特征, 规则如下：
    //  * 所有的闭包都自动实现了 FnOnce 特征, 因此任何一个闭包都至少可以被调用一次
    //  * 在闭包内 "没有返回所捕获变量的所有权" 的闭包自动实现了 FnMut 特征
    //  * 不需要对捕获变量进行改变的闭包自动实现了 Fn 特征
    // 换句话说: 闭包到底是哪种特征取决于你在闭包中如何使用捕获的变量, 而不是取决于闭包如何捕获它们, 跟是否使用 move 没有必然联系
    // 所以我们可以认为, Fn的trait都是一种约束关系, 真实是哪个Fn类型取决于你如何在闭包中使用
    {
        // 规则1: 即使我们是以不可变形式在闭包内使用, 但是我们依旧可以将其传递给 FnOnce trait
        // 规则2: 他没有返回变量的所有权, 所以自动实现了 FnMut
        // 规则3: 他没有进行改变, 所以也实现了 Fn
        let msg = String::new();
        let _rule1 = || println!("{msg}");
        fn_once(_rule1);
        fn_mut(_rule1);

        // 规则2:

        fn fn_once<F: FnOnce()>(f: F) {
            f();
        }

        fn fn_mut<F: FnMut()>(mut f: F) {
            f();
        }

        // 我们来看看这三个特征的简化版源码:
        // 从特征约束能看出来
        //  * Fn 的前提是实现 FnMut,
        //  * FnMut 的前提是实现 FnOnce
        //  * 因此要实现 Fn 就要同时实现 FnMut 和 FnOnce
        {
            pub trait Fn<Args>: FnMut<Args> {
                fn call(&self, args: Args) -> Self::Output;
            }

            pub trait FnMut<Args>: FnOnce<Args> {
                fn call_mut(&mut self, args: Args) -> Self::Output;
            }

            pub trait FnOnce<Args> {
                type Output;

                fn call_once(self, args: Args) -> Self::Output;
            }
        }
    }

    // (5) 闭包作为返回值
    // 相信大家对于如何使用闭包作为函数参数, 已经很熟悉了
    // 但是如果要使用闭包作为函数返回值, 该如何做？
    // 笔记: 每一个闭包实例都有独属于自己的类型, 即使于两个签名一模一样的闭包, 它们的类型也是不同的
    {
        // 方式一: 此路不通!
        // Rust 要求函数的参数和返回类型, 必须有"固定的内存大小"!
        // 例如i32就是4个字节, 引用类型是8个字节. 总之, 绝大部分类型都有固定的大小, 但是不包括特征.
        // 因为特征类似接口, 对于编译器来说, 无法知道它后面藏的真实类型是什么, 因为也无法得知具体的大小
        // fn ret_closure() -> Fn() {
        //     || {}
        // }

        // 方式二: 有点像了
        // impl Trait 可以用来返回一个实现了指定特征的类型
        // 那么这里 impl Fn() 的返回值形式, 说明我们要返回一个闭包类型, 它实现了 impl Fn() 特征
        // 但是这种有很大一个局限性: 所有返回值都必须返回一种类型! (rust在编译时会给你偷摸优化成静态分发, 不像dyn那种动态分发)
        // fn ret_closure() -> impl Fn() {
        //     || {}
        // }

        // 方式三: 来了老弟
        // 通过Box的方式接管所有权并进行动态派发
        fn ret_closure(click: bool) -> Box<dyn Fn() -> i32> {
            if click {
                Box::new(|| -> i32 { 10 })
            } else {
                Box::new(|| -> i32 { 20 })
            }
        }
    }

    // (6) 闭包的生命周期: 具体可看文档

    // (7) 补充一句
    // * 闭包也是有所有权的, 但是不同的闭包类型不一样: 有些闭包有Copy特征可以反复使用(Fn), 有些闭包没有Copy只能使用一次(FnMut,FnOnce)

    {
        // fn d<F: FnMut + Copy>() {} // 可以看出这只能使用一次, 不可能实现Copy滴
        // fn d<F: Fn + Copy>() {} // 可以看出这可以使用Copy
    }
}
