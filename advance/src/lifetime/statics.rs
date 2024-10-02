use std::fmt::Display;

#[allow(dead_code)]
pub fn hello() {
    // &'static 和 T: 'static 的故事
    //
    // 先来看两个例子:
    {
        // Exmaple1:
        //
        // &str 是硬编码在程序中的, 生命周期与程序有关(字符串字面值就具有 'static 生命周期:)
        let msg: &str = "shyunny"; // 其实就等价于 'static str
        display_author(msg);

        fn display_author(author: &'static str) {
            println!("author: {author}");
        }

        // Example2:
        //
        let msg: &str = "static_msg";
        display_static(msg);

        fn display_static<T>(msg: T)
        where
            T: Display + 'static, // 'static 作为生命周期约束了泛型参数 T
        {
            println!("static: {msg}");
        }
    }
    //
    // 辣么问题来了: &'static 和 T: 'static 的用法到底有何区别?

    // (1) &'static
    // &'static 对于生命周期有着非常强的要求: 一个引用必须要"活得跟剩下的程序一样久", 才能被标注为 &'static
    // 也就是说: 'static 标注的引用, 命跟程序一样长
    // 但是!!! &'static 生命周期"针对的仅仅是引用", "而不是持有该引用的变量", 对于变量来说, 还是要遵循相应的作用域规则
    // 也就是说: 引用命很长, 但是持有引用的变量还是会被销毁
    {
        #[allow(dead_code)]
        fn demo() -> usize {
            let strings = "天空没有极限!";
            strings.as_ptr() as usize
        } // strings 到这里就会被销毁了, 但是引用的数据却不会销毁
    }

    // (2) T: 'static
    // T: 'static 与 &'static 有相同的约束：T 必须活得和程序一样久
    {
        fn demo<T: Display + 'static>(data: &T) {
            println!("data: {}", data);
        }

        // case1:
        // demo 函数期望接收一个 T 类型的参数，其中 T 必须是 'static 的。但是，&i 是一个指向 i 的引用，其生命周期与 i 相同，而 i 是一个局部变量，其生命周期仅限于当前作用域。因此，&i 的生命周期不是 'static 的，这违反了函数的约束条件，导致编译错误

        let i = 10;
        // demo(&i); // 直接传递 &i 是不可以的, 因为 i 的引用并没有 'static 这么久
        demo(&i);
        // 原因在于我们约束的是 T, 但是使用的却是它的引用 &T
        // 换而言之, 我们根本没有直接使用 T, 因此编译器就没有去检查 T 的生命周期约束！
        // 它只要确保 &T 的生命周期符合规则即可
    }

    // (3) static 到底是针对了谁?
    // 到底是 &'static 这个引用还是该引用指向的数据活得跟程序一样久呢？
    // 答案是"引用指向的数据", 而"引用本身是要遵循其作用域范围的"
    {
        {
            let _strings = "de";
        } // 到这里, _strings 已经被销毁了, 但是他引用的数据还是打包在 binary 中
          // 由此可见, &'static 说明其指向的数据是生命周期跟程序一样长
    }

    // 总结:
    // * 如果你需要添加 &'static 来让代码工作, 那很可能是设计上出问题了
    // * 如果你希望满足和取悦编译器, 那就使用 T: 'static,很多时候它都能解决问题
}
