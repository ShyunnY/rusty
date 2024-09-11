use std::vec;

pub fn _entry() {
    // 模式匹配最常用的就是 match 和 if let
    // _match_if_let();
    // _option();
}

fn _pattern() {
    // 模式
    // 模式是 Rust 中的特殊语法, 它用来匹配类型中的结构和数据, 它往往和 match 表达式联用
    // 以实现强大的模式匹配能力。模式一般由以下内容组合而成：
    // 1). 字面值
    // 2). 解构的数组、枚举、结构体或者元组
    // 3). 变量
    // 4). 通配符
    // 5). 占位符

    // (1) match 分支
    // match 的"每个分支就是一个模式", 因为 match 匹配是穷尽式的,
    // 因此我们往往需要一个特殊的模式 _，来匹配剩余的所有情况
    //
    // match VALUE {
    //     PATTERN => EXPRESSION,
    //     PATTERN => EXPRESSION,
    //     PATTERN => EXPRESSION,
    // }

    // （2）if let 分支
    // if let 往往用于匹配一个模式，而忽略剩下的所有模式的场景
    //
    // if let PATTERN = SOME_VALUE {
    //
    // }

    // (3) while let 分支
    // 一个与 if let 类似的结构是 while let 条件循环,
    // NOTE: "它允许只要模式匹配就一直进行 while 循环"
    //
    // while let Some(top) = stack.pop() {
    //     println!("{}", top);
    // }

    // (4) for 分支
    // 使用 enumerate 方法产生一个迭代器，该迭代器每次迭代会返回一个 (索引，值) 形式的元组，然后用 (index,value) 来匹配
    // for (index, value) in v.iter().enumerate() {
    //     println!("{} is at index {}", value, index);
    // }

    // (5) let 语句
    // let也是一种模式匹配, 匹配的值绑定到变量上: let PATTERN = EXPRESSION;
    // 因此，在 Rust 中,变量名也是一种模式，只不过它比较朴素很不起眼罢了
    //
    // let x = 'a'; // 将匹配的值绑定到 x 上
    // let (x, y, z) = (1, 2, 3);   // 将一个元组与模式进行匹配(模式和值的类型必需相同！)

    // (6) fn 函数参数匹配
    //
    // a,b 就是一个模式, 匹配一个元组
    // fn foo((a, b): (i32, i128)) {}
}

fn _option() {
    // (1). Option
    // Option属于枚举, Some => Option::Some, None => Option::None
    // 用于解决rust中变量是否有值的问题
    // 简单解释就是：一个变量要么有值：Some(T), 要么为空：None
    let some: Option<i32> = Some(1024);
    let none: Option<i32> = None;

    match some {
        Some(val) => {
            println!("some val: {val}")
        }
        _ => (),
    }

    match none {
        Some(_) => (),
        None => {
            println!("none val!")
        }
    }
}

fn _match_if_let() {
    // match target {
    // 模式1 => 表达式1,
    // 模式2 => {
    //     语句1;
    //     语句2;
    //     表达式2
    // },
    // _ => 表达式3
    // }

    // (1).基本使用
    // match 的匹配必须要穷举出所有可能，因此这里用 _ 来代表未列出的所有可能性
    // * match的每一个分支都必须是一个'表达式'，且 "所有分支的表达式最终返回值的类型必须相同"
    // * X | Y，类似逻辑运算符 或，代表该分支 "可以匹配 X 也可以匹配 Y"，只要满足一个即可
    // 其实 match 跟其他语言中的 switch 非常像，_ 类似于 switch 中的 default
    let coin = Coin::Nickel;
    match coin {
        Coin::Penny => {
            println!("penny");
        }
        // _ 代表 default 分支
        _ => {
            println!("match default");
        }
    }

    // 不同分支之间使用逗号分隔.
    // 当 match 表达式执行时, 它将目标值 coin 按顺序依次与每一个分支的模式相比较,
    // 如果模式匹配了这个值，那么模式之后的代码将被执行。
    // 如果模式并不匹配这个值，将继续执行下一个分支。
    //
    // 每个分支相关联的代码是一个表达式，而表达式的结果值将作为整个 match 表达式的返回值。
    // 如果分支有多行代码，那么需要用 {} 包裹，同时最后一行代码需要是一个表达式.
    //
    // match 本身也是一个表达式, 因此可以用它来赋值
    let ret = match coin {
        Coin::Dime => 1,
        _ => 100,
    };
    println!("match ret: {ret}");

    // (2).模式绑定
    // 模式匹配的另外一个重要功能是从模式中取出绑定的值, 也就是拿出对应模式的值
    let c1 = Coin::Quarter(UsState::Alaska);
    match c1 {
        // 我们在这取出了模式中的值, 在处理 Option 的时候也是这样操作, Some(val) 匹配Some模式并取出值绑定到val上
        Coin::Quarter(state) => {
            println!("c1 state: {:?}", state);
        }
        _ => (),
    }

    // (3).穷尽匹配: match 的匹配必须穷尽所有情况, 但是我们可以使用 _ 覆盖其余的情况
    let num: u8 = 3;
    match num {
        1 => {
            println!("num is 1")
        }
        2 => {
            println!("num is 2")
        }
        // 要不处理每一个情况, 要不用'_'处理剩余情况. 总之就是要处理所有情况
        _ => {
            println!("num gt 2")
        }
    }

    // (4). if let
    // 有时会遇到只有一个模式的值需要被处理，其它值直接忽略的场景.
    // 例如: 我们只想要对 Some(3) 模式进行匹配, 不想处理任何其他 Some<u8> 值或 None 值
    //
    // 公式: 当你只要匹配一个条件且忽略其他条件时就用 if let ，否则都用 match
    let v = Some(1);
    if let Some(1) = v {
        println!("111!")
    }

    // (5). matches宏
    let my: Vec<MyEnum> = vec![MyEnum::Foo, MyEnum::Foo, MyEnum::Bar];
    let new: Vec<&MyEnum> = my
        .iter()
        .filter(|x| {
            // 太啰嗦了
            // match x {
            //     MyEnum::Foo => true,
            //     _ => false
            // }

            // 使用macthes宏
            matches!(x, MyEnum::Bar)
        })
        .collect();
    println!("my: {:?}", my);
    println!("new: {:?}", new);

    // (6). 变量遮蔽
    // 无论是 match 还是 if let, 这里都是一个新的代码块, 而且这里的绑定相当于新变量.
    // 如果你使用同名变量，会发生变量遮蔽.
    // 最好不要使用同名, 避免难以理解. 如下:
    let shadow = Some(999);
    println!("before shadow: {:?}", shadow);
    if let Some(shadow) = shadow {
        println!("shadowing: {}", shadow); // 存在同名变量时, 并且作用域不相同会导致出现变量遮蔽现象
    }
    println!("after shadow: {:?}", shadow);
}

#[derive(Debug)]
enum MyEnum {
    Foo,
    Bar,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
