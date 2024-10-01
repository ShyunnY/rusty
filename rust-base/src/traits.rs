use std::{
    fmt::{Debug, Display},
    ops::Add,
};

pub fn _entry() {
    // (0). 特征(trait)是啥？
    // 特征跟接口很类似, 特征定义了一组可以被共享的行为, 只要实现了特征你就能使用这组行为

    // (1). 为类型实现trait特征
    // 如果不同的类型具有相同的行为, 那么我们就可以定义一个整合这些行为的特征, 然后为这些类型实现该特征.
    // 定义特征是把一些方法组合在一起, 目的是 "定义一个实现某些目标所必需的行为的集合"
    // 语法: "impl Trait for Type" 读作为类型实现一个trait特征
    let wb = WeiBo;
    let wx = WeiXin;
    dbg!(wb.summary_platform());
    dbg!(wx.summary_platform());

    // (2). 特征定义与实现的位置(孤儿规则)
    // 我们可以将trait通过pub关键字设置为公开, 那么别人可以实现我们编写的trait
    // 关于特征实现与定义的位置, 有一条非常重要的原则: 如果你想要为类型 A 实现特征 T, 那么 A 或者 T 至少有一个是在当前作用域中定义的
    // 举个例子:
    // + 我们可以为 WeiXin 实现 std::fmt::Debug Trait, 因为类型A(WeiXin)处于当前作用域
    // + 我们可以为 String 实现 Summary, 因为特征T(Summary)处于当前作用域
    // - 我们不能为 String 实现 std::fmt::Debug Trait, 因为类型A(String)和特征T(std::fmt::Debug)都不处于当前作用域. 跟你八杆子打不着关系
    //
    // 该规则被称为孤儿规则，可以确保其它人编写的代码不会破坏你的代码，也确保了你不会莫名其妙就破坏了风马牛不相及的代码

    // (3). 默认实现
    // 你可以在特征中定义具有默认实现的方法. 这样其它类型"无需再实现该方法", 或者也可以选择"重载该方法"
    // 默认实现允许调用相同特征中的其他方法, 哪怕这些方法没有默认实现. 如此特征可以提供很多有用的功能而只需要实现指定的一小部分内容
    wb.summarize();
    wx.summarize();

    // (4). 使用trait特征作为函数参数
    // 可以使用任何实现了特征的类型作为函数的参数, 同时在函数体内, 还可以调用该特征的方法
    // item: &impl Summary => 实现了Summary的类型, 并且是不可变借用
    _display_summary(&wb, &wx);

    // (5). 特征约束(trait bound)
    // 实际上 _display_summary(item: &impl Summary) 是一个语法糖, 另外表现形式是: _display_summary<T: Summary>(item: &T)
    // 形如 T: Summary 被称为特征约束, 这意味着传入的泛型类型T必须实现 Summary, 也就是被 Summary 约束了.
    // 如果我们想要强制函数的两个参数是同一类型呢？(&impl Summary)语法就无法做到这种限制, 他可以使用实现了Summary Trait的不同的类型. 此时我们只能使特征约束来实现

    // (6).where 约束
    // 当特征约束变得更多的时候, 这会导致函数签名变得很复杂, 我们可以通过 "where" 关键字进行整理约束
    wx.summary_fn(1, 2);

    // (7). 基于特征约束进行有条件的实现方法
    // 在 impl 实现结构体方法时, 我们可以对泛型参数进行trait约束.
    // 只有real type实现了该trait才能够使用方法
    // 泛型参数、参数、返回值都在一起, 方便快速的阅读, 同时每个泛型参数的特征也在新的代码行中通过特征约束进行了约束.
    // 也可以有条件地实现特征
    let foo = Foo {
        msg: String::from("value"),
    };
    foo.print_msg();

    // (8). 将trait作为函数的返回值
    // 在一种场景下非常非常有用, 那就是返回的真实类型非常复杂.
    // 你不知道该怎么声明时(毕竟 Rust 要求你必须标出所有的类型), 此时就可以用 impl Trait 的方式简单返回
    // 例如, 闭包和迭代器就是很复杂, 我们可以返回一个 "impl Interator" 告诉调用者此函数返回了一个 迭代器
    // **"但是这种返回值方式有一个很大的限制: 只能有一个具体的类型"**
    // 这是因为我们没有使用 "动态分发-dny trait"
    //
    // 在这种情况下我们称其为 "静态派发", 既rust在编译期就可以确定trait的实际类型, 从而调用该类型的实现方法
    // 如果希望返回同一个trait的多个不同类型, 这需要依赖于动态派发, 也就是说我们只能在运行期才能知道调用的是谁的方法.
    // 这就是为什么在 “impl Summary” 语法下只进行了静态派发, 所以在函数中只能返回同一个类型.
    // 所以 impl trait 只能返回一个类型, "不能返回不同的类型"
    let _ = get_summary();

    // (9). 实战案例, 通过泛型参数和trait bound特征限制, 实现一个返回切片最大值的函数
    let demo_arr = [11, 22, 33, 99];
    let max = find_max(&demo_arr);
    println!("max={max}");

    // (10). derive 派生trait
    // 形如 #[derive(Debug)] 的代码已经出现了很多次, 这种是一种特征派生语法.
    // "被 derive 标记的对象会自动实现对应的默认特征代码，继承相应的功能" (有点类似于注解的玩法???)
    //
    // * Debug 特征,它有一套自动实现的默认代码,当你给一个结构体标记后，就可以使用 println!("{:?}", s) 的形式打印该结构体的对象
    // * Copy 特征,它也有一套自动实现的默认代码,当标记到一个类型上时，可以让这个类型自动实现 Copy 特征，进而可以调用 copy 方法，进行自我复制。
    // 总之，derive 派生出来的是 Rust 默认给我们提供的特征，在开发过程中极大的简化了自己手动实现相应特征的需求，当然，如果你有特殊的需求，还可以自己手动重载该实现。

    // (11). 调用特征方法需要引入库
    // 如果你要使用一个特征的方法, 那么你需要将该特征引入当前的作用域中.
    // 例如我们使用了 Display, Debug 特征, 我们需要将特征引入到当前作用域中: "use std::fmt::{Debug, Display};"
    let b1 = Bar { money: 1.1 };
    let b2 = Bar { money: 2.3 };
    let b3 = b1 + b2;
    println!("b3.money = {}", b3.money);
}

struct Bar<T: Add<T, Output = T>> {
    money: T,
}

impl<T: Add<T, Output = T>> Add for Bar<T> {
    // "关联类型的一个主要优点是它为 trait 提供了灵活的返回类型, 而不是强制要求返回 Self 类型"
    // 起始就是让用户在实现trait的时候才去指定关联类型, 这大大的提高了灵活性, 可以不局限于返回泛型参数或者Self
    //
    // 在这里, 我们将Output类型指定为 Bar, 代表我们返回 Bar 类型.
    // 如果不使用关联类型, 那么我们只能使用 T 或者 Self, 这看起来一点都不灵活
    type Output = Bar<T>;

    fn add(self, rhs: Self) -> Self::Output {
        let money: T = self.money + rhs.money;
        Bar { money }
    }
}

fn find_max<T>(arr: &[T]) -> T
where
    // 由于我们需要比较并且希望进行栈上赋值, 所以我们需要限制泛型参数实现 PatiaOrd 和 Copy 特征
    T: PartialOrd + Copy,
{
    let mut max: T = arr[0];
    // NOTE: 这里使用了模式匹配以及隐式解构
    // arr.iter()返回了 &T 类型, 我们通过 &ele 匹配其值, 然后rust会自动帮我们进行解引用
    // 即 &ele 匹配 &T, 然后隐式解构成 ele(T)
    for &ele in arr.iter() {
        if ele > max {
            max = ele
        }
    }

    max
}

fn get_summary() -> impl Summary {
    WeiBo {}
}

// 我们将参数类型作为泛型, 并且约束其必须实现 Summary 特征
// 那么说明调用此函数时, 必须传入类型一样的并且实现了Summary trait的 real type
fn _display_summart_bound<T: Summary>(item1: &T, item2: &T) {
    item1.summarize();
    item2.summarize();
}

// 当我们不要求 item 和 i 一定是同一类型可以使用这种语法
// 反之我们无法约束 item 和 i 的类型是否是同一个
fn _display_summary(item: &impl Summary, i: &impl Summary) {
    item.summarize();
    i.summarize();
}

trait Summary {
    // 我们声明方法的同时, 提供默认实现. 这样可以让实现trait的类型只需要实现很小一部分的方法即可
    fn summarize(&self) {
        println!("summary in @{} platform.", self.summary_platform());
    }

    // 特征只定义行为看起来是什么样的, 而不定义行为具体是怎么样的.
    // 因此我们只定义特征方法的签名, 而不进行实现，此时方法签名结尾是 ";"
    fn summary_platform(&self) -> &str;

    // 我们通过 "where" 关键字约束了泛型参数
    // * 对于泛型参数T, 我们要求其实现 Display和Clone trait
    // * 对于泛型参数U, 我们要求其实现 Display和Debug trait
    fn summary_fn<T, U>(&self, t: T, u: U)
    where
        T: Display + Clone,
        U: Display + Debug,
    {
        println!("summary where trait bound...{t},{u}");
    }
}

struct WeiXin;

impl Summary for WeiXin {
    fn summary_platform(&self) -> &str {
        "weixin"
    }
}

struct WeiBo;

impl Summary for WeiBo {
    fn summary_platform(&self) -> &str {
        "weibo"
    }
}

struct Foo<T> {
    msg: T,
}

impl<T> Foo<T>
where
    T: Display + Clone,
{
    fn print_msg(&self) {
        println!("method trait bound: self.msg={}", self.msg);
    }
}
