// 链表在Rust中之所以这么难, 完全是因为循环引用和自引用的问题引起的
// 这两个问题可以说综合了 Rust 的很多难点, 难出了新高度
// 所以本模块来解决 循环引用 和 自引用的问题

// 循环引用
pub mod circle_ref;
// 自引用
pub mod self_ref;
