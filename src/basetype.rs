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
}