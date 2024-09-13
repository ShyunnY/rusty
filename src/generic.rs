use std::fmt::Debug;

pub fn _entry() {
    // 泛型
    // 什么是泛型? 实际上泛型就是一种多态. 泛型主要目的是为程序员提供编程的便利, 减少代码的臃肿, 同时可以极大地丰富语言本身的表达能力
    // 一般来说泛型会和trait一起使用, 以此约束实际类型

    // (1) 结构体泛型
    // 我们可以在结构体中使用泛型, 让部分字段可以通用. 但是有以下几点需要注意
    // * 提前声明,跟泛型函数定义类似,首先我们在使用泛型参数之前必需要进行声明 "Struct<T>",接着就可以在结构体的字段类型中使用 T 来替代具体的类型
    // * 使用同一泛型参数的字段必须要是"同一个类型"! (不同类型也不属于一个泛型参数呀!)
    let p: Point<i32, String> = Point {
        x: 1,
        y: 100,
        z: String::from("value"),
    };
    println!("T generic point: {:?}", p);

    // (2) 枚举泛型
    // 实际上就是在枚举中为某些枚举字段引入泛型, 我们常见的 Option<T> 和 Result<T,E> 就是很经典的枚举泛型
    let result: Result<i32, &str> = Result::Err("find you!");
    println!("T enum generic: {:?}", result);

    // (3) 方法泛型
    // 使用泛型参数前依然需要提前声明：impl<T>, 只有提前声明了我们才能在 Struct<T> 中使用它.
    // 这样 Rust 就知道 Struct 的尖括号中的类型是泛型而不是具体类型.
    // 需要注意的是: "这里的 Struct<T> 不再是泛型声明,而是一个完整的结构体类型"
    // 因为我们定义的结构体就是 "Struct<T>" 而不再是 "Struct"
}

// 我们可以同时指定多个泛型参数, 不同的字段使用不同的泛型参数
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: T,
    z: U,
}

// 我们需要在 impl 上声明与 Struct 一致的泛型参数
impl<T, U> Point<T, U> {
    // 我们不需要在函数签名上再此写 T , 而是直接用
    pub fn say(val: T) {}
}

// 只有当前的Point的 T=i32,U=i32 才能使用此 impl 块的关联函数
impl Point<i32, i32> {
    // 为泛型中某个真实类型实现方法, 这代表只有使用指定类型才能调用
    pub fn say_1() {}

    // Point结构体的泛型参数是 <T,U>
    // 而当前函数的mixup用的是 <K,V> 泛型参数
    // 说白了你可以理解为: 一个是"结构体泛型", 一个是"函数泛型"
    fn mixup<K, V>() {}
}
