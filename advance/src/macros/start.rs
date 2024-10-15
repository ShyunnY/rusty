#[allow(dead_code)]
pub fn hello() {
    use crate::myvec;

    let v = myvec![1, 2, 3];
    println!("macro vec: {:?}", v);
}

// 声明宏编写
#[macro_export]
macro_rules! myvec {
    () => {
        panic!("*** must provide to element! ***")
    };
    /*
       $(...),*
       1. $ 是一个占位符，用于匹配并捕获模式。
       2. (...) 是被重复的模式，可以是任意有效的 Rust 语法结构。
       3. , 是分隔符，用于分隔重复模式中的各个实例。
       4. * 表示匹配零次或多次

    */
    // ($x:expr),* 表示($x:expr) 可以匹配零个或多个, 使用","作为分隔符, 用于分隔重复模式中的各个实例
    ($($x: expr),*) => {{
        use std::vec::Vec;

        let mut v = Vec::new();
        println!("Build Vec by macro_rules");
        $(
            v.push($x);
        )*
        v
    }};
}
