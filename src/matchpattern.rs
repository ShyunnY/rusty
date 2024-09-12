pub fn _entry() {
    // 模式匹配最常用的就是 match 和 if let
    // 什么叫模式匹配? 本质上就是让 模式与表达式进行匹配, 如果匹配成功代表true, 匹配失败代表false
    // if let Some(5) == some {}    // 当模式匹配成功才会执行作用域内部的代码

    // _match_if_let();
    // _option();
    _all_patterns();
}

fn _all_patterns() {
    // (1). 匹配字面量值
    let x: i32 = 3;
    match x {
        1 => println!("1"),
        2 => println!("2"),
        _ => println!("anything"),
    }

    // (2). 匹配命名变量
    let x = Some(5);
    let y = 10;
    match x {
        Some(10) => println!("x = 10"), // Some中的值不匹配x
        Some(y) => println!("x = {y}"), // 这里进行了变量遮蔽, 在match作用域内, y=Some中匹配的值
        _ => println!("anything"),
    }
    println!("y = {y}");

    // (3). 单分支多模式 (可以使用" | "语法匹配多个模式，它代表 或的意思)
    let x = 1;
    match x {
        1 | 2 => println!(r#""1 | 2""#),
        _ => println!("anything!"),
    }

    // (4). 通过序列 "..=" 或者 ".." 匹配值的范围, "..=" 或者 ".." 语法允许你匹配一个闭区间序列内的值
    // 序列只允许用于数字或字符类型, 原因是：它们可以连续，同时编译器在编译期可以检查该序列是否为空.
    // 字符和数字值是 Rust 中仅有的可以用于判断是否为空的类型。
    let x = 5;
    match x {
        1..5 => println!(r#"in [1,5)"#),  // 匹配 [1,5)
        3..=5 => println!(r#"in [1,5]"#), // 匹配 [3,5] 区间
        _ => println!("anything!"),
    }

    // (5). 模式匹配解构结构体
    let mut p = Point { x: 1, y: 2 };
    let Point { x, y } = p; // 直接解构同名字段
    let Point { x: a, y: b } = p; // 使用其他变量名解构匹配
    println!("同名解构 point x={x} y={y}");
    println!("不同名解构 point x={a} y={b}");

    p.x = 0;
    match p {
        Point { x: 0, y } => println!("point x=0, y={y}"), // 匹配 x=0 && y!=0
        Point { x, y: 0 } => println!("point x={x} y=0"),  // 匹配 x!=0 && y=0
        Point { x, y } => println!("point x={x} y={y}"),   // 匹配 x!=0 && y!=0
    }

    // (6). 解构枚举
    // 这里老生常谈一句话, 模式匹配一样要类型相同: 也就是说 "expr" 需要与匹配的 "pattern" 一样的类型.
    // 如果匹配 Message::Move , 那么我们需要确保与其定义的类型一致, 既使用 Message::Move { x, y }
    let msg = Message::People(Stmt::Hello(String::from("hello, shyunny")));
    match msg {
        Message::Quit => println!("msg is quit"),
        Message::Move { x, y } => println!("msg x = {x}, y = {y}"),
        Message::Write(str) => println!("msg str = {str}"),
        Message::People(Stmt::Hello(people_stmt)) => {
            println!("msg people stmt hello: {people_stmt}") // 嵌套枚举解构, 实际上对于嵌套的结构体也是一样方式使用
        }
    }

    // (7). 解构复杂结构体和数组
    // 元组和结构体混合结构(双打hhh)
    let ((a, b), Point { x, y }) = (('a', 'b'), p);
    println!("complete a = {a} b = {b}");
    println!("complete point x = {x} y = {y}");
    // 数组
    let arr: [i32; 2] = [00, 99];
    let [a1, a2] = arr; // "注意数组结构用的是中括号!!!"
    println!("arr a1 = {a1} a2 = {a2}");
    // 不定长数组
    let arr = [11, 22, 33];
    let [a, ..] = arr; // match head
    println!("head: a = {a}");
    let [.., b] = arr;
    println!("tail: b = {b}");

    // (8). 使用 '_' 忽略模式中的值
    // 有时忽略模式中的一些值是很有用的, 比如在 match 中的最后一个分支使用 _ 模式匹配所有剩余的值.
    // 我们也可以在另一个模式中使用 _ 模式, 使用一个以下划线开始的名称, 或者使用 .. 忽略所剩部分的值
    let numbers = (5, 4, 3, 2, 1);
    match numbers {
        // 可以在一个模式中的多处使用下划线来忽略特定值
        (_, 4, _, _, _1) => println!("1 || 4"),
        _ => println!("anything"),
    }

    // 使用下划线开头忽略未使用的变量. 如果创建一个变量又不使用, rust会给出一个警告, 认为这是一个bug
    // 使用 '_' 和使用以下划线开头的名称有些微妙的不同：
    // * 比如 '_x' 仍会将值绑定到变量
    // * 而 "_" 则完全不会绑定
    // 举个例子
    let some = Some(String::from("hello,world"));
    if let Some(_s) = some {
        println!("found string!"); // 此时 String 的所有权已经移动到 _s 变量上, 后续我们不能再用了
    }
    let some1 = Some(String::from("hello,world"));
    if let Some(_) = some1 {
        println!("found string!"); // 此时 String 的所有权没有进行移动, 因为'_'并不会绑定变量, 后续我们还可以用 some1
    }
    println!("some1: {:?}", some1);

    // 用 ".." 忽略剩余值
    // 注意: '..' 不能放在两边, 否则会导致产生歧义
    let tuple: (i32, i32, i32, i32) = (1, 2, 3, 4);
    match tuple {
        (head, ..) => println!("tuple head: {head}"),
    }

    // (9). 匹配守卫
    // 匹配守卫（match guard）是一个位于 match 分支模式之后的 "额外if条件", 它能为分支模式提供更进一步的匹配条件.
    let some = Some(5);
    let y = 3;
    match some {
        Some(10) => println!("some is 10"),
        // 使用了匹配守卫, 进行进一步的匹配
        Some(n) if n > y => println!(" some n({n}) > y({y})"),
        _ => println!("nothing!"),
    }
    // 也可以在匹配守卫中使用 '或运算符 |' 来指定多个模式, 同时匹配守卫的条件会作用于所有的模式
    let x = 4;
    let y = false;
    match x {
        // 等价于 => ((1|2|3) && y == true)
        1 | 2 | 3 if y => println!("ok"),
        _ => println!("error"),
    }

    // (10). '@'绑定
    // @（读作 at）运算符允许为一个字段绑定另外一个变量
    match 2 {
        // 放在这里相当于将匹配的值绑定到 num 变量上
        num @ (1 | 2 | 3) => println!("num = {num}"),
        _ => {}
    }
    // '@'的前面是绑定, 后面是解构
    // 下面就是一个例子 ⬇️
    let p @ Point { x, y } = Point { x: 1, y: 2 };
    println!("x={x} y={y}");
    println!("p: {:?}", p);
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    People(Stmt),
}

enum Stmt {
    Hello(String),
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
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
