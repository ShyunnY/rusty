pub fn _entry() {
    // (1) Method 方法, 就是关联到对象上的函数
    // Rust 的对象定义和方法定义是分离的, 这种数据和使用分离的方式, 会给予使用者极高的灵活度
    let mut d = Dog::new(String::from("wang"));
    println!("create dog by construct: {:#?}", d);
    println!("dog name: {}", d.get_name());

    // (2) self、&self 和 &mut self
    // 在一个 impl 块内: Self 指代被实现方法的"结构体类型", self 指代此"类型的实例". 我们为哪个结构体实现方法, 那么 self 就是指代哪个结构体的实例
    // **需要注意的是， self 依然有所有权的概念**
    // + self 表示当前结构体实例的所有权转移到该方法中,这种形式用的较少(往往用于把当前的对象转成另外一个对象时使用)
    // + &self 表示该方法对 Rectangle 的不可变借用(我们并不想获取所有权，也无需去改变它，只是希望能够读取结构体中的数据)
    // + &mut self 表示可变借用(想要在方法中去改变当前的结构体)
    d.set_name("wong");
    println!("dog name: {}", d.get_name());
    let dd: Dog = d.convert();
    println!("dd: {:?}", dd);
    // println!("dd: {:?}", d); // 不能再使用 'd' 了, 因为d已经被销毁了

    // (3) rust允许方法名跟结构体字段名相同
    // 一般来说, 方法跟字段同名，往往适用于实现 getter 访问器.
    // 我们可以把 结构体 的字段设置为"私有属性", 只需把它的 new 和 xxx 方法设置为"公开可见", 以此来实现 getter
    println!("与字段同名的方法: {}", dd.name());

    // (4) 自动引用 + 自动解引用
    // 引子: (->)符号去哪了了？
    // Rust 有一个叫 自动引用和解引用的功能. 他会自动进行引用或者解引用, 当调用struct的方法时, 会根据方法签名自动处理
    // 例如:
    // d.get_name(&self) -> (&d).get_name(&self)
    // d.set_name(&mut self) -> (&mut d).set_name(&mut self)
    // d.convert(self) -> d.convert(self)
    // 可以看出, rust会根据方法的接收者来确定如何"自动引用/自动解引用"

    // (5) 关联函数
    // 我们如何为一个结构体定义一个构造器方法？也就是接受几个参数，然后构造并返回该结构体的实例呢？
    // 这种定义在 impl 中且没有 self 的函数被称之为关联函数.
    // 因为它没有 self，不能用 f.read() 的形式调用. 因此它"是一个函数而不是方法"，它又在 impl 中，与结构体紧密关联，因此称为关联函数。
    // 实际上就满足两个条件:
    // * 它是一个函数而不是一个方法(没有接受者)
    // * 它处于结构体的 Namespace 中
    // > Rust 中有一个约定俗成的规则, 使用 new 来作为构造器的名称. 所以我们直接使用 "XXX::new()" 构建一个结构体实例

    // (6) 多个 impl 块
    // Rust 允许我们为一个结构体定义多个 impl 块, 目的是提供更多的灵活性和代码组织性.
    // 例如当方法多了后, 可以把相关的方法组织在同一个 impl 块中, 那么就可以形成多个 impl 块.
    // 举个例子: 我们可以为 File 结构体中组织两个 impl 块, 一个处理 read 另一个处理 write

    // (7) 为枚举实现方法
    // 枚举类型之所以强大，不仅仅在于它好用、可以同一化类型，还在于，我们可以像结构体一样，为枚举实现方法
    let msg = Msg::Message(String::from("vallll"));
    msg.call();

    // (8) Self 和 self 的区别
    // * self 是一个方法的接收者, 指代"调用该方法的实例"
    // * Self 是一个类型别名, 表示"实现当前方法的结构体或枚举类型本身"
}

#[derive(Debug)]
enum Msg {
    Message(String),
}

impl Msg {
    fn call(&self) {
        match self {
            Self::Message(v) => println!("self: {:?}", v),
        }
    }
}

#[derive(Debug)]
pub struct Dog {
    name: String,
}

// impl Dog {} 表示为 Dog 实现方法(impl 是实现 implementation 的缩写)
// 这样的写法表明 impl 语句块中的一切都是跟 Dog 相关联的, 所以其中定义的函数称之为关联函数.
impl Dog {
    // new是 Dog 的关联函数, 因为它的第一个参数不是self, 且new并不是关键字. 这种方法往往用于初始化当前结构体的实例
    pub fn new(name: String) -> Self {
        Dog { name }
    }

    // '&self' 不可变借用当前结构体实例, 表明我们想执行read动作
    pub fn get_name(&self) -> &str {
        &self.name
    }

    // '&self' 可变借用当前结构体实例, 表明我们需要修改当前结构体
    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    // 'self' 获取当前实例的所有权, 然后再返回一个新的Dog实例
    fn convert(self) -> Dog {
        Dog { name: self.name }
    }

    fn name(&self) -> &str {
        &self.name
    }
}
