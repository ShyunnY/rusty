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

    // (4) const 值泛型
    // 基于值的泛型（value-based generics）指的是一种将值作为泛型参数传递的方式.
    // 这与传统的基于类型的泛型不同, 后者通常以类型作为参数. 基于值的泛型允许我们在编译时将具体的常量值（如整数、布尔值、字符串等）传递给泛型
    // Rust 的基于值的泛型主要体现在其类型系统对常量值的支持上. 通过 const 泛型参数，我们可以在编译时指定常量值
    //
    // 我们可以这样理解 类型泛型和值泛型
    //   * 类型泛型: 可以使用多个不同的类型(聚焦于不同类型)
    //   * 值泛型: 针对某个const类型, 可以使用多个不同的值(聚焦于同一类型不同的值)
    let a1: [i32; 4] = [1, 2, 3, 4];
    let a2: [i32; 2] = [1, 2];
    _display_arr(a1);
    _display_arr(a2);

    // (5) 泛型性能
    // 在 Rust 中泛型是零成本的抽象，意味着你在使用泛型时，完全不用担心性能上的问题
    // Rust 通过在编译时进行泛型代码的 "单态化(monomorphization)" 来保证效率.
    // 单态化是一个通过填充编译时使用的具体类型, 将通用代码转换为特定代码的过程.
    // 编译器所做的工作正好与我们创建泛型函数的步骤相反，编译器寻找所有泛型代码被调用的位置并针对具体类型生成代码。

    // 例如我们使用 Some, 在coding时我们编写的是Some(1.1), 可以看出泛型类型是 f64
    // 当编译时, rust编译器会进行单态化, 将 "通用代码转为特定代码"!
    // 也就是说:  Some(1.1) -> Some_f64(1.1) 这样处理让代码在运行期可以做到 Non-Runtime 开销
    let some: Option<f64> = Some(1.1);
    println!("some: {:?}", some);
}

// N是一个基于值的泛型参数, 我们通过 const 修饰, 并指明其值是基于 usize 的
fn _display_arr<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("arr: {:?}", arr)
}

// 我们可以同时指定多个泛型参数, 不同的字段使用不同的泛型参数
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: T,
    z: U,
}

// 我们需要在 impl 上声明与 Struct 一致的泛型参数
// Q: 为什么我们需要在 impl 后面还要重复指定 <T,U> 呢? Point<T,U> 不是已经指定了吗？
// A: 实际上当 Struct 具有泛型参数时, 我们可以在编写 impl 方法时, 可以指定为哪些实际类型实现方法
// (也就是泛型的实际类型是 impl 指定的类型才能使用方法)
// 例如: 我们希望为 T=i32 类型实现一个foo()方法, 为 T=String 类型实现一个bar()方法, 默认类型实现 size() 方法
// * impl<i32,U> Point<i32,U>    t=i32,u=u时才能调用
// * impl<String, U> Point<String, U> t=String,u=u时才能调用
// * impl<T, U> Point<T, U> t=t,u=u随时都能用

impl<T, U> Point<T, U> {
    // 我们不需要在函数签名上再此写 T , 而是直接用
    pub fn size(val: T) {}
}

impl<i32, U> Point<i32, U> {
    fn foo() {}
}
impl<String, U> Point<String, U> {
    fn bar() {}
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
