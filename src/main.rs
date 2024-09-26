use std::error::Error;

mod advancetrait;
mod basetype;
mod compound;
mod err;
mod flow;
mod generic;
mod haaashmap;
mod lifetime;
mod matchpattern;
mod method;
mod ownership;
mod traitobj;
mod traits;
mod variable;
mod vector;

// 同时我们又一次看到了Box<dyn Error> 特征对象, 因为 std::error:Error 是 Rust 中抽象层次最高的错误
// 其它标准库中的错误都实现了该特征, 因此我们可以 "用该特征对象代表一切错误"
// 就算 main 函数中调用任何标准库函数发生错误, 都可以通过 Box<dyn Error> 这个特征对象进行返回
//
// 至于 main 函数可以有多种返回值, 那是因为实现了 std::process::Termination 特征
fn main() -> Result<(), Box<dyn Error>> {
    // variable::_var();
    // basetype::_base();
    // ownership::_entry();
    // compound::_entry();
    // flow::_entry();
    // matchpattern::_entry();
    // method::_entry();
    // generic::_entry();

    // 特征
    // traits::_entry();

    // 特征对象
    // traitobj::_entry();

    // 深入特征
    // advancetrait::_entry();

    // vector 集合
    // vector::_entry();

    // haaaaash map 哈希map
    // haaashmap::_entry();

    // lifetime 生命周期
    // lifetime::_entry();

    // err错误处理
    err::_entry();

    Ok(())
}
