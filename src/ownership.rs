pub fn _entry(){
    let s_stack: &str = "hello";
    println!("分配在 `栈` 上的字符串 s_stack: {s_stack}");

    // String类型被分配到堆上, 因此可以动态伸缩
    let mut s_heap: String = String::from("world");
    s_heap.push_str(",shyunn");
    println!("分配在 `堆` 上的字符串 s_stack: {s_heap}");

    // 以下代码没有发生所有权转换, 而是栈内数据拷贝
    // 因为其是基本数据类型, 不需要分配内存到堆上, 直接在栈内拷贝即可.
    let a = 1;
    let b = a;
    println!("不涉及所有权转换: b={b}");

    //> **我们可以将所有权理解为儿子(值)认爸爸(变量)**
    //> rust认为变量是被绑定到值上的.
    //> 也就是说在复杂类型下, 一个儿子在一个作用域中只有一个爸爸

    // 由于 String 被分配到堆上, 所以此时会发生所有权转换.
    // rust在变量离开作用域时会自动调用 drop 函数清理内存, 如果不转移所有权会导致对同一个内存空间(String)进行二次释放
    //
    // 以下例子中, c中的 point,len,cap 被拷贝到d, 并且使得c无效了(我们称这种动作为 "移动")
    let c = String::from("hello");
    let d = c;
    println!("此时发生了所有权的转换: d={d}");

    // 这里的 e只是引用了"world"字符串, 所以并不存在所有权. 本质上 f 是进行了拷贝操作
    let e:&str = "world";
    let f = e;
    println!("e: {e} f: {f}");


    // 深拷贝(克隆)
    // 首先, Rust 永远也不会自动创建数据的 “深拷贝”. 因此, 任何自动的复制都不是深拷贝, 可以被认为对运行时性能影响较小
    let g = String::from("g_origin");
    let h = g.clone();
    println!("g: {g} h(clone): {h}");

    // 浅拷贝(拷贝)
    // 浅拷贝只发生在栈上，因此性能很高
    // 以下例子进行了浅拷贝, 不会存在所有权转换的问题(基本数据类型并没有带有所有权)
    let i = 1;
    let j = i;
    println!("j: {j}");

    // 这里可以给出一个通用的规则: 任何基本类型的组合可以 Copy , 不需要分配内存或某种形式资源的类型是可以 Copy 的Copy代表不会具有所有权移动的问题
    // 实现了 Copy 特征的类型无需所有权转移, 可以直接在赋值时进行 "数据拷贝"

    // ** Move所有权: 转移所有权仅仅是复制一份`栈`中的指针, 再将新的指针赋予新的变量，然后让拥有 `旧指针的变量失效` **
    // 所有权很强大，避免了内存的不安全性，但是也带来了一个新麻烦： 总是把一个值传来传去来使用它。 传入一个函数，很可能还要从该函数传出去，结果就是语言表达变得非常啰嗦
    _move_demo();
    _borrowing();
    _mut_borrowing();
}

fn _borrowing(){
    // 获取变量的引用, 称之为借用(borrowing)[注意这是不可变的]. 正如现实生活中，如果一个人拥有某样东西，你可以从他那里借来，当使用完毕后，也必须要物归原主
    // (借用本质就是,我可以拿你的值, 但是销毁还是你来干哈, 我不帮你干销毁的事)
    // 以下例子中: y 绑定了 x的引用, 我们可以理解为 y 借用了 x的值
    let x = 5;
    let y = &x;
    println!("y borrow x value: {y}");
    assert_eq!(x,*y);   // 比较值需要"解引用"

    // "&" 符号即是引用，它们允许你使用值，但是不获取所有权
    // 简单来说: 我们将 rust 中的引用理解为 "借用"
    let z = String::from("hello,borrowing!");
    let len = _cal_string_length(&z);    // 这里我们借用了 "z", 没有触发所有权转移
    println!("str z length: {len}");
}

fn _mut_borrowing(){
    // 获取变量的可变引用, 称之为可变借用(mut borrowing)
    // 以下例子中: y 绑定了 x的可变引用, 我们可以理解为 y 可变地借用了 x的值
    let mut x = 5;
    println!("before x: {x}");
    let y = &mut x;
    *y = 1;
    println!("after x: {x}");

    // 可变引用同时只能存在一个. 不过可变引用并不是随心所欲、想用就用的，它有一个很大的限制： 同一作用域，特定数据只能有一个可变引用
    //
    // 可变引用与不可变引用不能同时存在
    //
    // 注意，引用的作用域 s 从创建开始, 一直持续到它最后一次使用的地方

    // 以下例子首先不可变的借用了 s1, 然后可变的借用了 s1
    let mut s1 = String::from("hello,world");
    let s2 = &s1;
    let s3 = &s1;
    println!("s2: {s2}");   // s2 不可变借用作用域结束
    println!("s3: {s3}");   // s3 不可变借用作用域结束

    // 此时可以开始进行可变借用
    let s4 = &mut s1;
    // 注意, 此时不能访问s1, 因为此时s1被s4进行可变借用, s1随时都会被修改
    // 我们可以理解为, 我的宝马借给别人了, 此时我想换成奔驰, 那也得等别人的宝马还给我才能换
    *s4 = String::from("world,hello!"); // s4 可变借用作用域结束
    // 注意, 此时可以访问s2, 因为此时s1被s4进行的可变借用已经结束了
    println!("s1: {s1}");

    // 借用规则总结规则如下:
    //
    // 1). 同一时刻，你只能拥有要么一个可变引用(只能被一个人修改), 要么任意多个不可变引用(可以被多个人读取)
    // 2). 引用必须总是有效的(不能引用一个空的值)
}

// 悬垂引用也叫做悬垂指针，意思为指针指向某个值后，这个值被释放掉了，而指针仍然存在，其指向的内存可能不存在任何值或已被其它变量重新使用。
// 在 Rust 中编译器可以确保引用永远也不会变成悬垂状态：当你获取数据的引用后，
// 编译器可以确保数据不会在引用结束前被释放，要想释放数据，必须先停止其引用的使用。
// fn _dangling_ref() -> &String{
//     let s = String::new();   // 1). 创建一个s
//     &s   // 2). 返回s的借用
// }// 但是此时 s 已经被 Drop 了, 所以s的借用称为悬挂指针, 这违反了 rust 的设计

// &String 表明这是个 "借用的String 类型"
fn _cal_string_length(s: &String) -> usize{
    s.len() // s离开作用域并不会执行 Drop 操作, 因为它不曾真正拥有过 s ~
}

fn _move_demo(){
    let s1 = String::from("hello-1");
    let s2 = String::from("hello-2");

    let s3 = _get_ownership(s2);

    println!("s1: {s1}");
    // println!("s2: {s2}");    // s2所有权已经被移走了
    println!("s3: {s3}");
}
// 1). s3超出作用域导致失效, 执行 Drop
// 2). s2在调用get_ownership时已经将所有权移动到函数栈内, 此时s2已经没有所有权了, 什么都不做
// 3). s1 执行 Drop

fn _get_ownership(s : String) -> String{
    s
}