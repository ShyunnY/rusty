//! 这是docs文档模块!

//! 用于编写docs的笔记
pub mod docs;

/// `add_one` 将指定值加 1
/// ## Examples
///
/// ```rust
/// let arg = 5;
/// let answer = rs::add_one(arg);  // rs代表package名, 在 Cargo.toml 中可以看到
///
/// assert_eq!(6,answer);
/// ```
///
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// `div` 对两个数进行相除
/// ## Examples
///
/// ```rust
/// let (x,y) = (10,5);
/// let result = rs::div(x,y);
///
/// assert_eq!(2,result);
/// ```
/// # Panics
/// ```rust,should_panic
/// let (x,y) = (10,0);
/// let result = rs::div(x,y);
/// ```
pub fn div(x: i32, y: i32) -> i32 {
    if x == 0 || y == 0 {
        panic!("Divide by zero error")
    }

    x / y
}

pub use self::gem::get_me_gem;

#[doc(alias = "bar")]
pub struct Foo;

pub mod gem {
    pub fn get_me_gem() {
        println!("Get me G.E.M. !!!")
    }
}
