use advancetrait::_entry;

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

fn main() {
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
}
