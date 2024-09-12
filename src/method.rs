pub fn _entry() {
    // 方法, 就是关联到对象上的函数
    // Rust 的对象定义和方法定义是分离的, 这种数据和使用分离的方式, 会给予使用者极高的灵活度
    let d = Dog::new(String::from("wang"));
    println!("create dog by construct: {:#?}", d);
    println!("dog name: {}", d.identity());
}

#[derive(Debug)]
struct Dog {
    name: String,
}

// impl Dog {} 表示为 Dog 实现方法(impl 是实现 implementation 的缩写)
// 这样的写法表明 impl 语句块中的一切都是跟 Dog 相关联的, 所以其中定义的函数称之为关联函数.
impl Dog {
    // new是 Dog 的关联函数, 因为它的第一个参数不是self, 且new并不是关键字. 这种方法往往用于初始化当前结构体的实例
    fn new(name: String) -> Self {
        Dog { name }
    }

    // "&self" 表示借用当前的 Dog 结构体
    fn identity(&self) -> &str {
        &self.name
    }
}
