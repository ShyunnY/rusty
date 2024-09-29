pub fn _entry() {
    println!("用于介绍注释是怎么使用的.")

    // (1) 代码注释
    // 1. 行注释, 使用 "//". 没什么好说的了
    // 2. 块注释, 使用 " /* ......*/ ", 只需要将注释内容使用 /* */ 进行包裹即可
    // 你会发现 Rust 的代码注释跟其它语言并没有区别, 主要区别其实在于"文档注释"这一块
    /*
       我
       是
       块
       注
       释
    */

    // (2) 文档注释
    // 当查看一个 crates.io 上的包时, 往往需要通过它提供的文档来浏览相关的功能特性、使用方式, 这种文档就是通过文档注释实现的.
    // Rust提供了`cargo doc`的命令, 可以用于把这些文档注释转换成 HTML 网页文件, 最终展示给用户浏览
    // 这样用户就知道这个包是做什么的以及该如何使用
    //
    // 1. 文档行注释
    // 使用 "///" 在代码块上进行文档行注释
    // 文档注释中的代码有几点需要注意:
    // * 文档注释需要位于`lib`类型的包中, 例如 src/lib.rs 中
    // * 文档注释可以使用 markdown语法! 例如`# Examples`的标题, 以及代码块高亮
    // * 被注释的对象需要使用`pub`对外可见, 记住：文档注释是给用户看的, 内部实现细节不应该被暴露出去
    //
    // 2. 文档块注释
    // 使用 "/** */" 在代码块上进行文档块注释(实际上只是为了避免多写几个"///", 看起来也不美观, 社区还是推荐使用文档行注释)
    //
    // 3. 查看文档滴命令 `cargo doc`
}
