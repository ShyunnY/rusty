pub fn _entry(){
    let s_stack: &str = "hello";
    println!("分配在 `栈` 上的字符串 s_stack: {s_stack}");

    // String类型被分配到堆上, 因此可以动态伸缩
    let mut s_heap: String = String::from("world");
    s_heap.push_str(",shyunn");
    println!("分配在 `堆` 上的字符串 s_stack: {s_heap}");

    // 以下代码没有发生所有权转换, 而是栈内数据拷贝
    // 因为其是基本数据类型, 不需要分配内存到堆上, 直接在栈内拷贝即可.
    let a = 1;
    let b = a;
    println!("不涉及所有权转换: b={b}");

    // 由于 String 被分配到堆上, 所以此时会发生所有权转换.
    // rust在变量离开作用域时会自动调用 drop 函数清理内存, 如果不转移所有权会导致对同一个内存空间(String)进行二次释放
    let c = String::from("hello");
    let d = c;
    println!("此时发生了所有权的转换: d={d}");
}