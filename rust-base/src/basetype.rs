pub fn _base(){
    // 部分情况下需要显式指定数据类型
    let guess :u8 = "43".parse().expect("Not Number!");
    println!("显式指定数据类型 u8: {:?}",guess);

    // 整数类型: i8,i32,i64  u8,u32,u64
    let big: u64 = u64::MAX;
    println!("u64 big: {big}");

    // 补码循环溢出: u8设置257 -> 1
    let a: u8 = 255;
    let b = a.wrapping_add(2);
    println!("补码循环溢出: {b}");

    // 浮点类型: f32,f64
    let x = 2.0;
    let y :f32 = 2.1;
    println!("f64: {x}, f32: {y}");

    // 二进制为00000010
    let a:i32 = 2;
    // 二进制为00000011
    let b:i32 = 3;

    println!("(a & b) value is {}", a & b);

    println!("(a | b) value is {}", a | b);

    println!("(a ^ b) value is {}", a ^ b);

    println!("(!b) value is {} ", !b);

    println!("(a << b) value is {}", a << b);

    println!("(a >> b) value is {}", a >> b);

    let mut a = a;
    // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    a = a << b;
    println!("(a << b) value is {}", a);

    // 语法糖, 用于生成 range
    // 1..5 代表生成 [1,4]
    // 1..=5 代表生成 [1,5]
    let range1 = 1..5;
    let range2 = 1..=5;
    println!("range1: {:#?}",range1);
    println!("range2: {:#?}",range2);

    // 序列只允许用于数字或字符类型，原因是："它们可以连续"
    // 同时编译器在编译期可以检查该序列是否为空，"字符和数字值是 Rust 中仅有的可以用于判断是否为空的类型"
    for i in 'a' ..= 'e'{
        println!("range bytes: {i}");
    }

    // 字符类型
    let b1: char = '😊';
    let b2: char = '🀄';
    println!("rust支持 ascii 和 Unicode: {b1} {b2}");

    // 布尔值, 没啥好说的了
    let yes: bool = true;
    let no: bool = false;
    println!("布尔值: yes={yes} no={no}");

    // 单元类型: (), 唯一的值也是 ()
    // 函数不指定返回值时, 默认返回的就是单元类型 ()
    // 例如常见的 println!() 的返回值也是单元类型 ()
    // 再比如，你可以用 () 作为 map 的值，表示我们不关注具体的值，只关注 key
    // 这种用法和 Go 语言的 struct{} 类似，可以作为一个值用来占位，但是完全不占用任何内存
    let _u: () = ();

    // 语句和表达式
    // + 语句: 完成了一个具体的操作, 但是并没有返回值, 因此是语句(表现为以分号结尾)
    // + 表达式: 表达式会进行求值, 然后返回一个值(表现为无分号结尾). 例如 "5 + 6" 会返回11, 所以这是一个表达式
    //          表达式不能包含分号.这一点非常重要, 一旦你在表达式后加上分号, 它就会变成一条语句, 再也不会返回一个值, 请牢记!
    //
    // 表达式如果不返回任何值, 会隐式地返回一个 (): 我们在编写函数时, 没有指定返回值类型实际上就是默认返回了一个 ()
    //
    // let a = { let b = 1; }; 此时 a 就是一个 ()


    // 这是语句, 执行了一个变量绑定操作, 并且没有返回值
    let _stmt = "hello,world";

    // 这是表达式
    let ret = {
        let num = 1;
        num + 2
    };
    println!("expr ret: {ret}");

    // 模拟三元运算
    let ret = if ret % 2 == 0{
        '奇'
    }else {
        '偶'
    };
    println!("expr ret: {ret}");

    let _a = { let _b = 1;};

    println!("func 1 + 100 = {}",_add_two(1,100,0));
    // _never_return();
}

// add_two rust的函数名和变量名习惯用 "蛇形命名"
// 可以使用 '_' 代表不使用的匿名参数 (golang也是使用这种方式)
fn _add_two(a: i32,b: i32, _:u32) -> i32{
    a + b
}

// never_return 永不返回的 "发散函数"!
// 当用 "!" 作函数返回类型的时候,
// 表示该函数永不返回( diverge function ).
// 特别的，这种语法往往用做会导致程序崩溃的函数：
fn _never_return() -> !{
    loop {
        println!("xo")
    }
}