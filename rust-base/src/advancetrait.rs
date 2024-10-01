use std::fmt::Debug;

pub fn _entry() {
    // (1). 关联类型
    // 关联类型可以使特征定义更加灵活和简洁, 特别是在你希望为特征提供一个或多个类型时.
    // 通过关联类型, Rust 能够在实现特征时提供具体类型, 而不需要在每个方法中重复指定泛型参数
    // ** "关联类型的参数通常由特征的`实现者`设置" **
    // 具体来说: 在定义一个特征时, 你可以在特征内部声明一个关联类型, 而在实现该特征时具体的类型实现者需要"为这个关联类型提供一个具体类型"
    // 我们常常使用 "Self::XXX" 在trait中使用关联类型
    let d: Demo = Demo {};
    let item: <Demo as Display>::Instance = d.copy();
    println!("item: {item}");

    // (2). 默认泛型类型参数
    // 当使用泛型类型参数时, 可以为其"指定一个默认的具体类型",(用户不指定, 则使用默认的泛型类型)
    // 例如: <T: XXX>, 当用户没有指定T的类型时, 则使用默认的XXX类型
    // 默认类型参数主要用于两个方面:
    // + 当你定义一个泛型类型或 trait 时，可以为其指定一个默认类型，这样用户在使用时可以省略这个类型
    // + 使用默认参数可以让你的代码在某些情况下更灵活. 用户可以选择使用默认类型, 也可以根据需要显式指定其他类型
    let default_container: Container = Container::new(100); // 此时我们没有指定实际的泛型参数, 其使用i32的默认类型
    println!("default container: {}", default_container.get());
    let string_container: Container<String> = Container::new(String::from("value")); // 此时指定为string类型
    println!("string container: {}", string_container.get());

    // (3). 调用同名的方法
    // 当struct存在某个方法, 该struct实现的一个或多个trait中也存在同名方法, 此时存在以下默认顺序
    // 1. 优先调用类型上的方法(调用struct实例的同名方法时)
    // 2. 如果想调用实现的trait上的方法, 我们需要显示调用(调用特征上的方法)
    // 3. 如何调用完全限定方法(即方法没有 self 参数, 是个关联函数)
    //
    // 知识点:
    // * 其实我们调用self的方法时, xx.method() 是一个语法糖, 完整语法其实是: XXX::method(&self). 所以我们通过这种方式可以显示指明要调用谁的方法
    // * 完全限定语法: <Type as Trait>::function(receiver_if_method, next_arg, ...);
    let h = Human {};
    h.fly(); // 执行Human的方法

    Pilot::fly(&h); // 执行Pilot trait的方法
    Wizerd::fly(&h); // 执行Wizerd trait的方法

    // 如果我们想调用实现了Pilot的baby_name(), 就需要完全限定语法
    Human::baby_name();
    // 在尖括号中，通过 as 关键字，我们向 Rust 编译器提供了类型注解, 告诉rust Pilot就是Human而不是其他实现
    // 完全限定语法可以用于任何函数或方法调用, 那么我们为何很少用到这个语法？
    // 原因是 Rust 编译器能根据上下文自动推导出调用的路径, 因此大多数时候，我们都无需使用完全限定语法。
    // "只有当存在多个同名函数或方法, 且 Rust 无法区分出你想调用的目标函数时", 该用法才能真正有用武之地
    <Human as Pilot>::baby_name(); // 将human当作pilot并调用其关联函数
    <Human as Pilot>::fly(&h); // 将human当作pilot并调用其关联函数
    <Human as Wizerd>::fly(&h); // 将human当作wizerd并调用其关联函数

    // (4). 为特征约束一个特征
    // 简而言之就是: 如果当前特征定义了其他特征约束, 那么实现当前特征的同时还需要实现其他特征
    // e.g. trait A: B + C 实现A的同时还需要实现B

    // (5). 在外部类型上实现外部特征(绕开孤儿规则)
    // 使用 "newtype" 模式. 简而言之: 就是为一个元组结构体创建新类型. 该元组结构体封装有一个字段, 该字段就是希望实现特征的具体类型(曲线救国)
    // e.g. 我们想为Vec实现外部trait, 我们将其包装进Wrapper中, 为Wrapper实现外部trait, 但是操作时还是使用内部的Vec, 这不就绕开了吗～
    // "该封装类型是本地的, 因此我们可以为此类型实现外部的特征"
    // "newtype" 不仅仅能实现以上的功能, 而且它在运行时没有任何性能损耗, 因为在编译期, 该类型会被自动忽略.
    //
    // 知识点: Rust 提供了一个特征叫 Deref, 实现该特征后可以自动做一层类似类型转换的操作,
    // 可以将 Wrapper 变成 Vec<String> 来使用. 这样就会像直接使用数组那样去使用 Wrapper, 而无需为每一个操作都添加上 self.0
    // *Defer原来跟这是一起用的, soga！！！*
    // *没有什么是包一层解决不了的, 如果不行那就再包一层 -- 鲁迅*
}

// 我们可以使用元组进行包裹我们想要实现的外部类型
struct Wrapper(Vec<u8>);

impl std::fmt::Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", 1)
    }
}

// 实现trait A时, 类型还需要实现Debug和Copy
trait A: Debug + Copy {
    fn hello();
}

// 这样写也可以, 另外一种语法
trait B where
    Self: Debug + Clone,
{
}

trait Pilot {
    fn baby_name();
    fn fly(&self);
}

trait Wizerd {
    fn fly(&self);
}

impl Pilot for Human {
    fn fly(&self) {
        println!("pilot 让你飞! ")
    }

    fn baby_name() {
        println!("my baby is pilot")
    }
}

impl Wizerd for Human {
    fn fly(&self) {
        println!("wizerd 让你飞! ");
    }
}

struct Human;

impl Human {
    fn fly(&self) {
        println!("喜欢装13就让你飞! ")
    }

    fn baby_name() {
        println!("my baby is human")
    }
}

// struct Container<T = i32> 这里的 T 表示了作用域内的其他T都使用这个T, 所以这个T是老大
struct Container<T = i32> {
    value: T,
}

// impl<T> 这里的 T 表示了作用域内的其他T都使用这个T, 所以这个T是老大
impl<T> Container<T> {
    fn new(value: T) -> Self {
        Container { value }
    }

    fn get(&self) -> &T {
        &self.value
    }
}

struct Demo;

impl Display for Demo {
    // 泛型需要在多处使用, 而这里的关联类型只需要一处使用
    // 关联类型的参数通常由特征的`实现者`设置"
    type Instance = i32;

    // 相当于将 Demo 中的Instance设置为100了
    fn copy(&self) -> Self::Instance {
        100
    }

    type Bound = ();
}

trait Display {
    // 声明一个关联类型参数, 让实现者填充其真实的类型
    type Instance;

    // 声明一个带有 trait bound 特征约束的关联类型参数, 实现者必须提供满足其约束的真实类型
    type Bound: Copy + Clone + Debug;

    fn copy(&self) -> Self::Instance;
}
