pub fn _entry() {
    // (1).特征对象(*****)
    // 当 "impl trait" 作为函数返回值时, rust只允许返回同一种类型. 那么我们如何做到类似golang那种动态接口模式呢?
    //
    // Rust 中的特征对象（trait objects）是实现动态分发（dynamic dispatch）的工具,
    // 允许你在编译时"不知道具体类型"的情况下, 操作一组"具有相同特征（trait）的方法的不同类型". 这是 Rust 实现多态的一种方式
    //
    // 特征对象实现了"动态分发", 通过引用（&dyn Trait）或智能指针（如 Box<dyn Trait>）来访问. 这些对象在运行时会根据实际类型找到合适的实现
    //   * 动态分发: 特征对象通过一个虚表（vtable）来实现动态分发. "虚表保存了实现特征的类型的方法地址". 通过这种方式，可以在运行时确定调用哪个具体实现
    //   * 性能考虑: 特征对象的动态分发会有一些性能开销，因为每次调用方法时需要通过虚表查找实现. 然而，它提供了很大的灵活性和便利，尤其在需要处理不同类型的对象时
    //
    // 我们可以通过 "&dyn trait" 引用或者 "Box<dyn trait>" 智能指针的方式来 "创建特征对象"
    // 1. &dyn Trait
    // &dyn Trait 是一个动态特征对象的引用, 它是一种不可变引用或可变引用, 指向一个实现了特征的具体类型的实例.
    // 由于是引用，它不会对对象的所有权产生影响，只是借用它。
    // 特点：
    //  生命周期: &dyn Trait 需要一个生命周期，因为它借用的是一个已有的对象
    //  存储位置: 对象的实际存储位置可以在栈上也可以在堆上
    //  性能: 使用引用不会涉及堆内存分配, 因此通常比 Box 更轻量
    //  场景: 这种方式适用于那些你只需要借用对象而不需要拥有对象的场景
    //
    // 2. Box<dyn Trait>
    // Box<dyn Trait> 是一个动态特征对象的智能指针，指向一个堆上分配的对象。Box 负责对象的所有权，并在 Box 被丢弃时释放内存。
    // 特点:
    //  所有权：Box<dyn Trait> "拥有对象的所有权". 它适用于需要在堆上分配对象的场景.
    //  动态大小：Box 可以存储"动态大小的对象"，因为 Box 本身有固定的大小。
    //  性能：Box 需要额外的堆内存分配，可能会比引用稍微慢一些, 但提供了对象的所有权管理.
    //  场景: 适合那些需要将对象分配到堆上的需求
    //
    // 总结
    //  + &dyn Trait: 用作对实现了 Trait 的对象的不可变或可变引用，适用于对象在函数调用时的临时借用。它不涉及堆内存分配，适合对对象的轻量访问。
    //  + Box<dyn Trait>: 用作拥有 Trait 的对象的堆上分配的智能指针，适用于需要对象的所有权或需要在堆上存储对象的场景。它涉及堆内存分配和管理。

    // (2). 为什么在使用特征对象时, 我们要使用 &dyn trait引用方式或者 Box<dyn trait>只能指针方式?
    // 核心思想就是: "由于rust需要在编译期确定类型的大小, 但是动态特征对象trait只有在 runtime 时才知道真实大小,
    //              我们可以通过&dyn Trait 和 Box<dyn Trait> 在编译期的大小和固定性来解决"
    //
    // 1. &dyn Trait
    //      + &dyn Trait 是对一个实现了某个特征的对象的借用. 由于特征对象的实际类型在编译时未知,编译器无法确定它们的确切大小.
    //        因此，&dyn Trait 的大小是固定的, 但这个固定大小是相对的.("也就是我们在编译期间能够确定这是一个指针大小, 而不是不知道其大小")
    //      + 固定大小: &dyn Trait 的大小在编译时是固定的(因为这是一个借用). 它的大小取决于指针本身的大小(通常是 4 字节或 8 字节，取决于平台),
    //        而不是它所指向的对象的大小. 也就是说, &dyn Trait 是一个指向动态大小对象的指针, 指针的大小是固定的
    //
    // 2. Box<dyn trait>
    // Box<dyn Trait> 是一个"堆上分配的特征对象的智能指针", Box 允许 Rust 在堆上存储动态大小的对象, 并在 Box 被丢弃时自动释放内存.
    //      + 固定大小：Box<dyn Trait> 的大小在编译时是固定的. 尽管 Box 内部存储的对象的大小是不固定的,
    //        但"Box 本身的大小是固定的", 因为它"只需要存储一个指向堆上对象的指针以及可能的元数据"（例如指针的大小通常是 4 字节或 8 字节）
    //      + 动态大小对象: Box 允许存储动态大小的对象, 因为 Box 处理了所有权和内存管理.编译器"只需要知道 Box 自身的大小",
    //        而不需要知道存储在 Box 中对象的大小
    //      + Box的大小都是固定的, 而Box指向的数据大小是不固定,动态的.
    //
    // 总结:
    // * &dyn Trait 的大小是固定的（指针的大小）, 但它指向的对象是动态大小的, 这种固定大小是相对的，因为它只涉及指针的大小.
    // * Box<dyn Trait> 的大小也是固定的（指针的大小）, 并且它管理堆上存储的动态大小对象.
    //   Box 处理对象的所有权和内存分配, 使得对象的实际大小在编译时对 Box 本身来说是不重要的

    let items: Vec<Box<dyn Draw>> = vec![Box::new(Button {}), Box::new(SelectBox {})];
    let screen = Screen { components: items };
    screen.run();

    // 1. 这种情况下, 我们用的是借用特征对象, 此时"所有权不移动"
    let button: &dyn Draw = &Button {};
    drawing_dyn(button);

    // 2. 这种情况下, 我们使用智能指针管理分配在堆上的对象, box会拥有所有权, 此时"所有权移动"
    let select_box = Box::new(SelectBox {});
    drawing_box(select_box);

    // (2). 特征对象的动态分发
    // 首先我们来对比一下 "泛型" 和 "特征对象的区别"
    //  + 泛型是在'编译期'完成处理的: 编译器会 "为每一个泛型参数对应的具体类型" 生成一份代码,这种方式是静态分发(static dispatch).
    //    因为是在编译期完成的, 对于运行期性能完全没有任何影响.
    //  + dyn 特征对象是在 '运行时' 处理的: 在运行时根据 ptr+vptr 来确定实际的特征类型.
    //    当使用特征对象时, Rust必须使用动态分发. 编译器无法知晓所有可能用于特征对象代码的类型,所以它也不知道应该调用哪个类型的哪个方法实现.
    //    为此Rust在运行时"使用特征对象中的指针来知晓需要调用哪个方法". 动态分发也阻止编译器有选择的内联方法代码,这会相应的禁用一些优化.
    //
    // 动态分发的核心点:
    // + 特征对象大小不固定:这是因为对于特征 Draw,类型Button可以实现特征 Draw,类型SelectBox也可以实现特征 Draw,因此特征没有固定大小
    // + 几乎总是使用特征对象的引用方式, 如 &dyn Draw、Box<dyn Draw>
    //      - 虽然特征对象没有固定大小, 但它的引用类型的大小是固定的, 它由"两个指针组成（ptr 和 vptr）", 因此占用两个指针大小
    //      - 一个指针 ptr 指向实现了特征 trait 的具体类型的实例(类似下面例子中的 Button )
    //      - 另一个指针 vptr 指向一个虚表vtable, vtable 中保存了实现trait特征实例对于可以调用的实现于特征的方法.
    //        当调用方法时, 直接从 vtable 中找到方法并调用. 之所以要使用一个 vtable 来保存各实例的方法,
    //        是因为实现了特征的类型有多种, 这些类型拥有的方法各不相同, 当将这些类型的实例都当作特征来使用时
    //        (此时，它们全都看作是特征类型的实例), 有必要区分这些实例各自有哪些方法可调用
    //        e.g. Message实现了 Display trait, vtable就保存了Message类型以及实现的Display方法, 将其看成Display实例
    //
    // 总结:
    // 1.当一个类型实现了某个特征, 此时就将该类型实例看成trait的实例, 他不能用自己的方法了, 只能用trait的方法了(鸭子类型)
    // 2.xxx是哪个特征对象的实例, 它的 vtable 中就包含了该特征的方法.

    // (3). Self和self的区别
    // + Self: 指代特征或者方法"类型的别名"
    // + self: 指代当前的"实例对象"

    // (4). 特征对象的限制
    // 不是所有特征都能拥有特征对象, 只有"对象安全的特征"才行. 当一个特征的所有方法都有如下属性时,它的对象才是安全的:
    // 1.方法的返回类型不能是"Self"(当特征的方法返回 Self 时, 这个方法依赖于具体的类型, 这里 Self 代表实现这个特征的具体类型.
    //   在编译时无法知道 Self 的具体类型, 因此无法创建一个指向特征对象的指针, 因为我们需要知道返回值的具体大小和布局)
    // 2.方法"没有任何泛型"参数(由于不同的调用可能会使用不同的 T, 这会导致不确定性, 使得编译器无法创建一个一致的指针来指向特征对象)
    //
    // 换句话说: 我们在使用trait对象的时候也需要一个指针指向真正的类型实例, 那么我们反推的时候也是如此.
    // rust需要知道对象的内存布局和大小, 这都需要真实的类型实例介入
}

// 偷偷说一句: 其实你还可以用成 &mut dyn Draw
fn drawing_dyn(d: &dyn Draw) {
    d.draw();
}

fn drawing_box(d: Box<dyn Draw>) {
    d.draw();
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for ele in self.components.iter() {
            ele.draw();
        }
    }
}

trait Draw {
    fn draw(&self);
}

struct Button;

impl Draw for Button {
    fn draw(&self) {
        println!("按钮button drawing...")
    }
}

struct SelectBox;

impl Draw for SelectBox {
    fn draw(&self) {
        println!("盒子select box drawing...")
    }
}
