pub fn _entry() {
    // 在 Rust 中, 可以使用 use 关键字把路径提前引入到当前作用域中, 随后的调用就可以省略该路径, 极大地简化了代码

    // 1. 基本引入方式
    // (1) 绝对路径引入模块: 我们使用 use + 绝对路径方式引入模块(直接从crate根开始向下寻找)
    {
        use crate::uses::mod_a::mod_b::mod_c;
        mod_c::inline("use + \"绝对路径\"");
    }
    // (2) 相对路径引入: 我们使用 use + 相对路径引入模块(由于mod_a属于users下的模块, 并且_entry也属于users的模块, 所以直接调用即可)
    {
        use self::mod_a::mod_b::mod_c;
        mod_c::inline("use + \"相对路径\"");
    }
    // (3) 引入模块函数函数?
    // 从使用简洁性来说引入函数自然是更甚一筹, 但是在某些时候引入模块会更好:
    //  * 需要引入同一个模块的多个函数
    //  * 作用域中存在同名函数
    //
    // 建议: 优先使用最细粒度(引入函数、结构体等)的引用方式, 如果引起了某种麻烦(例如前面两种情况), 再使用引入模块的方式
    {
        use self::mod_a::mod_b::mod_c::inline;
        inline("最小粒度的函数");
    }

    // 2. 避免同名引用
    // 正常来说我们只要保证"同一个模块中不存在同名项"就行, 模块之间、包之间的同名, 谁管得着谁啊.
    // 但我们真遇到这种情况后, 看看如何处理?
    //
    // (1) 模块引入截至到上一层, 也就是通过:  "模块::XXX" 的方式去区分
    // 避免同名冲突的关键, 就是使用 "父模块" 的方式来区分调用
    {
        // 即使有同名函数 inline(), 我们也可以根据其父模块进行区分
        use self::mod_a::mod_b::mod_c;
        use self::mod_a::mod_b::mod_d;
        mod_c::inline("xxoo");
        mod_d::inline();
    }
    // (2) 使用 as 作为模块的别名
    // 对于同名冲突问题, 还可以使用 as 关键字来解决, 它可以赋予引入模块一个 '船新版本'
    {
        // 如果我们导入的模块粒度真的这么小, 此时可以使用 as 起别名避免冲突
        use self::mod_a::mod_b::mod_c::inline as c_inline;
        use self::mod_a::mod_b::mod_d::inline as d_inline;

        c_inline("ooxx");
        d_inline();
    }

    // 3. 导入项再导出
    // 当外部的模块项被引入到当前模块中时, 它的可见性自动被设置为私有的
    // 如果你希望允许其它外部代码引用我们的模块项, 那么可以对它进行再导出
    // 当你希望将内部的实现细节隐藏起来或者按照某个目的组织代码时, 可以使用 "pub use 再导出".
    //
    // 例如统一使用一个模块来提供对外的 API, 那该模块就可以引入其它模块中的 API，然后进行再导出
    // 最终对于用户来说，所有的 API 都是由一个模块统一提供的
    {
        // 看起来mod_d是在uses下的, 其实他是在 "mod_b" 下滴 ~
        crate::uses::mod_d::inline();
    }

    // 4. 使用第三方包
    // * 在 Cargo.toml 文件的 [dependencies] 添加指定的依赖
    // * 然后我们使用 `cargo run/cargo build` 会自动下载依赖
    //
    // 1. crates.io, lib.rs
    // * crates.io: 官方提供的crate包管理网站, 我们都是在这上面下载crate
    // * lib.rs: 非官方提供的社区包管理网站, 搜索功能很强大, 内容展示更合理. 推荐!
    // * 在网上找到想要的包, 然后将你想要的包和版本信息写入到 Cargo.toml 中
    {
        use rand::Rng;
        let rnd = rand::thread_rng().gen_range(0..=100);
        println!("random number: {}", rnd);
    }

    // 5. 使用 "{}" 简化引入的方式
    {
        // (1). 对于以下的引入方式真的是鬼见愁啊! 明明都是一个父模块下的产物, 偏要重复写三行导入！
        // use std::collections::BTreeMap;
        // use std::collections::HashMap;
        // use std::collections::HashSet;
        //
        // 我们可以使用 "{}" 简化, 一行足以~
        // use std::collections::{BTreeMap, HashMap, HashSet};

        // (2) 在引入模块时使用self
        // self 可以用于替代模块自身, 它在模块中的两个用途:
        // * use self::xxx: 表示加载当前模块中的 xxx. (此时 self 可省略)
        // * use xxx::{self, yyy}: 表示加载当前路径下模块 xxx 本身, 以及模块 xxx 下的 yyy
        // use std::io::{self, IntoInnerError}; // 引入io模块自身以及其模块下的 IntoInnerError
    }

    // 6. 使用 "*" 引入模块下的所有依赖项
    // 对于之前一行一行引入 std::collections 的方式, 我们还可以使用: "xxx::ooo::*" 导入所有"公共依赖"项目
    // 当使用 * 来引入的时候要格外小心, 因为你很难知道到底哪些被引入到了当前作用域中, 有哪些会和你自己程序中的名称相冲突
    //
    // 对于编译器来说, 本地同名类型的优先级更高(所以需要注意是否与本地类型产生命名冲突了)
    {
        // use std::collections::*; // 引入collections下所有 "公共项"
    }

    // 7. 受限的可见性
    // 限制可见性的语法：
    // pub(crate) 或 pub(in crate::a) 就是限制可见性语法，前者是限制在整个包内可见，后者是通过绝对路径，限制在包内的某个模块内可见，总结一下：
    // * pub 意味着可见性无任何限制
    // * pub(crate) 表示在当前crate内可见, 外部crate不可见!
    // * pub(self) 仅在当前模块可见, 类似与文件系统的 "./" 下可见
    // * pub(super) 在父模块和当前模块都可见, 类似于文件系统的 "../." 下可见
    // * pub(in <path>) 表示在某个路径代表的模块中可见, 其中 path 必须是父模块或者祖先模块
    {
        // 在以下模块生命中, 我们希望 J 在模块a内可见, 模块a外不可见
        pub mod a {
            mod b {
                use std::println;

                fn _demo() {
                    println!("{}", super::c::J);
                }
            }

            // 我们指定了模块 c 和常量 J 的可见范围都只是 a 模块中, a 之外的模块是完全访问不到它们的
            // 通过 super 的方式指定
            pub mod c {
                use std::println;

                pub(super) const J: i32 = 4;

                pub(super) fn _demo() {
                    println!("{}", J);
                }

                fn _demo1() {
                    _demo();
                }

                pub mod d {
                    pub(super) fn _demo2() {
                        super::_demo();
                    }
                }
            }
        }

        // 我们在这里是无法访问 _demo2 的, 因为他只在他的祖先模块中可以被访问.
        // 当前编写代码的作用域与 mod a 是兄弟模块, (所以兄弟的儿子我们不能教育!)
        // a::c::d::_demo2();

        // let aa = a::c::J; // 此时在模块外不可以获取了
    }

    // 8. 多文件之间的组织方式, 详情看 package.md
}

// 使用 pub use 即可实现. 这里 use 代表引入 mod_d 模块到当前作用域, pub 表示将该引入的内容再度设置为可见
// 也就是说: 将mod_d模块导入到users下, 让外部使用的人认为mod_d是users模块下. 其实他是mod_b模块下的.
pub use mod_a::mod_b::mod_d;

mod mod_a {
    pub mod mod_b {
        pub mod mod_c {
            pub fn inline(way: &str) {
                println!("引入方式: {}", way);
            }
        }
        pub mod mod_d {
            pub fn inline() {
                println!("李逵?李鬼!")
            }
        }
    }
}
