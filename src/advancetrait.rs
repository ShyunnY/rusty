use std::fmt::Debug;

pub fn _entry() {
    // (1). 关联类型
    // 关联类型可以使特征定义更加灵活和简洁, 特别是在你希望为特征提供一个或多个类型时.
    // 通过关联类型, Rust 能够在实现特征时提供具体类型, 而不需要在每个方法中重复指定泛型参数
    // **"关联类型的参数通常由特征的`实现者`设置"**
    // 具体来说: 在定义一个特征时, 你可以在特征内部声明一个关联类型, 而在实现该特征时具体的类型实现者需要"为这个关联类型提供一个具体类型"
    let d = Demo {};
    let item: <Demo as Display>::Instance = d.copy();
    println!("item: {item}");
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
