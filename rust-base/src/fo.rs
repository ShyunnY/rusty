pub fn _entry() {
    println!("格式化输出!");

    // 格式化输出
    // 一说到到格式化输出, 可能很多人立刻就想到 "{}". 但是 Rust 能做到的远比这个多的多, 本章节我们将深入讲解格式化输出的各个方面
    // {} 的符号是占位符, 会被 println! 后面的参数依次替换. 可以理解为 Golang 中使用 "%" 进行占位

    // (1) 格式化的三大金刚: print!, println!, format!
    // * print! 将格式化文本输出到标准输出, '不带换行符'
    // * println! 同上, 但是在'行的末尾添加换行符'
    // * format! 将格式化文本'输出到 String 字符串'
    // 在实际项目中, 最常用的是 println! 及 format!, 前者常用来调试输出, 后者常用来生成格式化的字符串
    let info: String = format!("{} world!", "hello");
    println!("hello,world!");
    println!("{info}");

    // (2) 格式化的两大护法: eprint!, eprintln!
    // 使用方式跟 print!, println! 很像, 但是它们输出到'标准错误输出'
    // 在实际项目中, 如果是调试的话我们还是使用 println! 和 print!
    eprint!("this is eprint\n");
    eprintln!("this is eprintln");

    // (3) 非常厉害的 "{}" 格式化符号
    // 与其它语言常用的 %d, %s 不同. Rust 特立独行地选择了 "{}" 作为格式化占位符
    // 事实证明, 这种选择非常正确, 它帮助用户减少了很多使用成本.
    // 你无需再为特定的类型选择特定的占位符, 统一用 {} 来替代即可, 剩下的类型推导等细节只要交给 Rust 去做
    // 与 {} 类似, {:?} 也是占位符:
    //  * {} 适用于实现了 "std::fmt::Display" 特征的类型, 用来以更优雅、更友好的方式格式化文本, 例如展示给用户
    //  * {:?} 适用于实现了 "std::fmt::Debug" 特征的类型, 用于调试场景
    // 其实两者的选择很简单, 当你在写代码需要调试时, 使用 {:?}，剩下的场景选择 {}
    //
    // 对于数值、字符串、数组，可以直接使用 {:?} 进行输出. 但是对于结构体, 需要派生Debug特征后, 才能进行输出, 总之很简单
    //
    // 与大部分类型实现了 Debug 不同, 实现了 Display 特征的 Rust 类型并没有那么多, 往往需要我们自定义想要的格式化方式
    // 我们可以使用以下三种方式解决:
    //  * 使用 {:?} 或 {:#?}
    //  * 为自定义类型实现 Display trait
    //  * 使用 newtype 为外部类型实现 Display trait
    //
    // “{:#?}” 和 “{:?}” 几乎一样, 只是前者能够更美观的输出内容(二者都需要类型派生 Debug trait)
    // 因此对于 Display 不支持的类型, 可以考虑使用 {:#?} 进行格式化, 虽然理论上它更适合进行调试输出
    //
    // 1. 为自定义类型实现 Display trait, 使其可以通过 {} 进行输出
    let p = Person {
        name: String::from("shyunn"),
        age: 21,
    };
    println!("{}", p); // 我们为 Person 类型实现了 Display trait, 所以此时可以使用 {} 进行格式化输出

    //
    // 2. 使用 newtype 的方式为外部类型实现外部特征
    let arr = Array(vec![1, 2, 3, 4, 5]);
    println!("{}", arr); // 我们使用 newtype 的方式对外部类型进行了包装, 然后又再实现了外部类型, 所以此时可以使用 {} 进行格式化输出

    // (4) 神奇的位置参数
    // 除了按照依次顺序使用值去替换占位符之外, 还能"让指定位置的参数去替换某个占位符": 例如 {1}, 表示用第二个参数替换该占位符(索引从 0 开始)
    // 这比 Golang 香多了啊, 太神奇了
    let prefix = "hello";
    let suffix = "rust";
    println!("{0}-{1}", prefix, suffix);
    println!("{1}-{0}", prefix, suffix); // 让下标为 '1' 的参数找到其对应的占位符

    // (5) 具名参数
    // 我们还可以为参数指定名称, 需要注意的是: 带名称的参数必须放在不带名称参数的后面
    // 因为如果我们同时指定了位置参数和具名参数时, 位置参数需要用到下标
    println!("arg: {arg}", arg = 1); // "arg = 1" 是一个表达式, 所以可以直接使用

    // (6) 格式化参数
    // **格式化参数中都会使用 {:XXOO} 使用, 一定要携带 ':'**
    // 格式化输出, 意味着"对输出格式会有更多的要求". 例如只输出浮点数的小数点后两位
    //
    // 精读
    println!("保留后两位: {:.2}", 1.111);
    println!("保留后两位并且带符号: {:+.2}", 1.111);
    println!("不保留小数: {:.0}", 1.111);
    //
    // 进制
    println!("二进制: {:#b}!", 3);
    println!("八进制: {:#o}!", 27);
    println!("十进制: {}!", 27);
    println!("小写十六进制: {:#x}!", 15);
    println!("大写十六进制: {:#X}!", 27);
    //
    // 指数
    println!("指数: {:2e}", 100);
    //
    // 指针地址 这个很核心！！！
    let msg = String::new();
    println!("msg的地址: {:p}", &msg);

    // 在格式化字符串时捕获环境中的值
    let env_val = "我是环境中的值";
    println!("捕获环境中的值: {env_val}");
}

struct Array(Vec<i32>);

impl std::fmt::Display for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "数组是: ({:?})", self.0)
    }
}

struct Person {
    name: String,
    age: i32,
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "小弟名叫{}, 今年{}岁", self.name, self.age)
    }
}
