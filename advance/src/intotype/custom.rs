use std::{fmt::Display, panic};

#[allow(dead_code)]
pub fn hello() {
    // (1) newtype
    // 何为 newtype？简单来说, 就是"使用元组结构体的方式将已有的类型包裹起来": struct Meters(u32);
    // 那么此处 Meters 就是一个 newtype, 它使用元组结构体包裹了一个 u32
    //
    // 为何需要 newtype？Rust 这多如繁星的 Old 类型满足不了我们吗？这是因为:
    // * 自定义类型可以让我们给出更有意义和可读性的类型名(我们可以针对某些类型包装一个更符合上下文语义的newtype)
    // * 对于某些场景, 只有 newtype 可以很好地解决(例如我们需要绕过trait孤儿规则)
    // * 隐藏内部类型的细节(掩耳盗铃方式...)
    //
    // 1. 为外部类型实现外部特征
    // 如果 "在外部类型上实现外部特征" 必须使用 newtype 的方式
    // 否则你就得遵循孤儿规则: 要为类型 A 实现特征 T, 那么 A 或者 T 必须至少有一个在当前的作用范围内(孤儿规则)
    // 但是!
    // 我们可以使用newtype包装外部类型, 然后我们为newtype实现trait, 这样不就ok了嘛~
    {
        struct Arr(Vec<i32>);

        // 看起来我们是为了Arr实现外部trait, 实际上我们是为了元组内部的 Vec 实现外部trait
        impl Display for Arr {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "[{:?}]", self.0)
            }
        }

        let arr = Arr(vec![1, 2, 3]);
        println!("arr: {}", arr);
    }
    //
    // 2. 更好的可读性及类型异化
    // 我们可以根据代码上下文的语境, 设计一个可读性更高的类型
    // 例: 当前我们需要计算路径长, 则我们可以为u32设计一个newtype: struct Meter(u32)
    {
        struct Meter(u32);
    }
    //
    // 3. 隐藏内部类型的细节
    // 众所周知, Rust 的类型有很多自定义的方法, 假如我们把某个类型传给了用户
    // 但是又不想用户调用这些方法, 就可以使用 newtype
    /*
        这种方式在同一个模块中, 还是可以通过 ".0" 方式获取元组结构体内真实类型字段, 但是在外部模块将无法获取
        所以这可以让我们隐藏内部real type的方法, 只暴露某些方法给外部
    */

    // (2) 类型别名(Type Alias)
    // 除了使用 newtype, 我们还可以使用一个更传统的方式来创建新类型: 类型别名
    // 这里的类型别名实际上跟 Golang 的类型别名使用姿势一致
    // 但是: 类型别名并不是一个独立的全新的类型, 而是某一个类型的别名.
    {
        // 注意: 编译期依旧会将 Meter 当成 u32 看待, 类型别名只是给用户看的而已
        type Meter = u32;

        // 以下代码将顺利编译通过, 但是如果你使用 newtype 模式, 该代码将无情报错
        let x: u32 = 10;
        let m: Meter = 20;
        println!("u32 + Meter = {}", x + m); // 编译期依旧将其看为 u32 类型
    }
    // 做一个总结:
    /*
       1.类型别名"仅仅是别名", 只是为了让可读性更好, "并不是全新的类型!", newtype 才是！
       2.类型别名"无法实现为外部类型实现外部特征等功能", "而 newtype 可以!"(因为类型别名本质上不是新类型, 只是起了个名字而已)
    */

    // (2).类型别名除了让类型可读性更好, 还能减少模版代码的使用:
    // 更香的是, 由于它只是别名, 因此我们可以用它来调用真实类型的所有方法, 甚至包括 "?" 符号！
    // 例如我们如果需要使用 std::io::Error 与 Result 进行结合使用:
    {
        // 这种写法太冗长了, 我想偷懒!
        fn demo1<T>(t: T) -> Result<T, std::io::Error> {
            Ok(t)
        }

        // 定义一个类型别名
        type IOResult<T> = Result<T, std::io::Error>;

        // 然后使用类型别名, 看起来很棒!
        fn demo2<T>(t: T) -> IOResult<T> {
            Ok(t)
        }
    }

    // (3) !永不返回类型
    // 在函数那章的学习, 我们曾经介绍过 !
    // 类型: ! 用来说明"一个函数永不返回任何值(发散函数)"
    // 当时可能体会不深, 在学习了更多姿势后, 保证你有全新的体验:
    {
        let x = 10;
        // 必须保证 match 的各个分支返回的值是同一个类型
        let _y = match x {
            10 => 20,
            _ => {
                // 使用println!将会报错, 因为println! 宏返回一个元组, 但是我们需要返回一个i32
                // println!("no match type, expect i32, actual ()")

                // 使用 panic! 不会报错, 因为panic!宏的返回值是 “!”, 其实就代表不返回类型, 所以可以通过编译
                // 既然没有任何返回值, 那自然不会存在分支类型不匹配的情况
                panic!("1");

                // panic源码:
                /*
                    我们可以看到其函数签名返回值位置是个 "!" => 就是永不返回的发散函数啦~
                    pub fn panic_any<M: 'static + Any + Send>(msg: M) -> ! {}
                */
            }
        };
    }
}
