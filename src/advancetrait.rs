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
