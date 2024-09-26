use std::{backtrace::Backtrace, fmt::Error, fs::File, io::Read, net::IpAddr};

pub fn _entry() {
    // (1) err在什么语言中都是很重要的, 我们需要对此进行处理
    //
    // Rust 中的错误主要分为两类：
    // * 可恢复错误: 通常用于从系统全局角度来看可以接受的错误,例如处理用户的访问、操作等错误,
    //   这些错误只会影响某个用户自身的操作进程，而不会对系统的全局稳定性产生影响
    // * 不可恢复错误:刚好相反，该错误通常是全局性或者系统性的错误，例如数组越界访问，系统启动时发生了影响启动流程的错误等等，
    //   这些错误的影响往往对于系统来说是致命的
    // Rust没有异常, 但是 Rust 也有自己的卧龙凤雏: Result<T, E> "用于可恢复错误", panic! "用于不可恢复错误"
    //_panic();

    // (2) 可恢复的err--Result
    _result();

    // (3) ? 用在Option中的返回
    // 在Option中使用 "?"
    // * 如果是Some, 则返回Some(T)的T
    // * 如果是None, 则直接return None
    // 当然, 在Option中使用 ? 也是可以进行链式调用
    _get_option();

    // (4) 使用 ? 常见错误
    // 初学者在用 ? 时, 老是会犯错写出这样的代码:
    // fn first(arr: &[i32]) -> Option<&i32> {
    //   arr.get(0)?    // 如果是Some, 那么承接的变量类型为 &i32
    // }
    // 这段代码无法通过编译, 切记: ? 操作符"需要一个变量来承载正确的值"(也就是当正确时, 需要绑定变量的)
    // 这个函数只会返回 Some(&i32) 或者 None. 只有错误值能直接return返回, 正确的值是需要变量进行承接的
    // 因此 ? 只能用于以下形式:
    // * let v = xxx()?;    // 使用变量承接Some的T
    // * xxx()?.yyy()?;     // 忽略返回值处理

    // (5) main函数也是可以携带返回值滴!
    // 默认rust的main函数是返回()空元组类型, 实际上 Rust 还支持另外一种形式的 main 函数: 返回Result类型
    // 详情可见main.rs文件 main 函数

    // (6) try
    // try!宏类似于 "?", 但是现在已经寄了, 我们可以不用管了
    // 感兴趣可以看看源码
}

fn _get_option() -> Option<char> {
    "hello,err".lines().next()?.chars().last()
}

fn _result() {
    // Result枚举
    // result是一个伟大的发明(Q一下golang), 当正常时, 返回Ok(T). 当错误时, 返回Err(E).
    // 在rust中使用Result来处理错误和传播错误
    //
    // 传播界的大明星: "?"语法糖, 太有排面了。
    // 其实 ? 就是一个宏, 它的作用跟 match 几乎一模一样, 本质上展开就是以下样子:
    // let mut f = match f{
    //      OK(f) => f,
    //      Err(e) => return Err(e),
    // }
    // * 结果是 Ok(T), 则把 T 赋值给f.
    // * 结果是 Err(E),则 return 该错误.
    // 所以 ? 特别适合用来向上传播错误
    //
    // 想象一下, 一个设计良好的系统中, 肯定有自定义的错误特征.
    // 错误之间很可能会存在上下级关系, 例如标准库中的 std::io::Error 和 std::error::Error.
    // 前者是 IO 相关的错误结构体, "后者是一个最最最通用的标准错误特征", 同时前者实现了后者
    // 因此 std::io::Error 可以转换为 std:error::Error
    // 明白了以上的错误转换, ? 的更胜一筹就很好理解了, "**它可以自动进行类型提升（转换）**"
    //
    // 换句话说: 在设计函数Result时, 我们可以使用高抽象的Error, 让返回的real Err能够自动转换到高抽象Err上

    let config_name = "/Users/shyunn/code/rusty/src/config.yaml";
    let result = _display_file_content(config_name);
    println!("result: {}", result.unwrap());
}

// 我们可以看到一个错误类型通过 ? 返回后, 变成了另一个错误类型, 这就是 ? 的神奇之处
// 本质上这就是依赖了标准库中的 From 特征: 该特征有一个方法 from, 用于把一个类型转成另外一个类型. ? 可以自动调用该方法,然后进行隐式类型转换
// 因此只要函数返回的错误 ReturnError 实现了 From<OtherError> 特征, 那么 ? 就会自动把 OtherError 转换为 ReturnError
//
// 这种转换非常好用, 意味着你可以用一个大而全的 ReturnError 来覆盖所有错误类型, 只需要为各种子错误类型实现这种转换即可
// 所以 "?" 在背地里偷偷的调用了 From 进行了隐式转换成底层err
fn _display_file_content(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut f = File::open(path)?;
    let mut content = String::new();
    let _ = f.read_to_string(&mut content)?;

    // 甚至 "?" 还可以进行链式调用
    // 在编码时, 只要我们心里想着: ?成功就返回Result的T, 失败就return. 那么就能理解链式调用是怎么操作的了
    let mut c = String::new();
    File::open(path)?.read_to_string(&mut c)?;

    Ok(c)
}

fn _panic() {
    // (1) panic! 与不可恢复错误
    // panic用于面对影响应用程序的大错误, 不可恢复错误等
    // 对于这些严重到影响程序运行的错误, 触发 panic 是很好的解决方式.
    // 在 Rust 中触发 panic 有两种方式:
    //  *被动触发(类似于数组溢出, runtime运行时错误等)
    //  *主动调用(当我们认为这个错误影响程序运行并且不可恢复时, 我们手动抛出错误)
    // WARN: 切记!一定是不可恢复的错误, 才调用 panic! 处理.
    // 你总不想系统仅仅因为用户随便传入一个非法参数就panic吧？这很不合理, 所以只有当你不知道该如何处理时, 再去调用 panic!

    // 1. 手动抛出错误(我们可以设置env展示更详细的堆栈信息: RUST_BACKTRACE=1 cargo run)
    // panic!("rust panic!")

    // 2. backtrace 栈展开
    // 在真实场景中, 错误往往涉及到很长的调用链甚至会深入第三方库, 如果没有栈展开技术, 错误将难以跟踪处理
    // 我们可以通过env开启栈展开:
    // 以下代码就是一次栈展开(也称栈回溯), 它包含了函数调用的顺序.
    // 当然按照"逆序"排列: 最近调用的函数排在列表的最上方. 因为咱们的 main 函数基本是最先调用的函数了,所以排在了倒数第二位
    // 还有一个关注点, 排在最顶部最后一个调用的函数是 "rust_begin_unwind", 该函数的目的就是进行栈展开, 呈现这些列表信息给我们
    // 0: rust_begin_unwind
    // 1: core::panicking::panic_fmt
    // 2: core::panicking::panic_bounds_check
    // 3: <usize as core::slice::index::SliceIndex<[T]>>::index
    // 4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
    // 5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
    // 6: rs::err::_panic
    //           at ./src/err.rs:29:27
    // 7: rs::err::_entry
    //           at ./src/err.rs:10:5
    // 8: rs::main
    //           at ./src/main.rs:48:5
    // 9: core::ops::function::FnOnce::call_once
    //           at /rustc/eeb90cda1969383f56a2637cbd3037bdf598841c/library/core/src/ops/function.rs:250:5
    let _vc = vec![1, 2, 3];
    //println!("val: {}", vc[100]);

    // 3. panic时的两种终止方式
    // 当出现 panic! 时, 程序提供了两种方式来处理终止流程："栈展开"和"直接终止"
    //  * 栈展开: 默认的方式就是栈展开,这意味着 Rust 会回溯栈上数据和函数调用,因此也意味着更多的善后工作,好处是可以给出充分的报错信息和栈调用信息
    //  * 直接终止: 顾名思义不清理数据就直接退出程序, 善后工作交与操作系统来负责
    // 我们可以在cargo.toml文件中添加以下字段设置为panic时直接终止策略
    // [profile.release]
    // panic = 'abort'

    // 4. 线程panic之后, 程序还会停止吗?
    //  * 如果是 main 线程, 则程序会终止
    //  * 如果是 子线程, 则该线程会终止, 但不会影响 main线程
    // 这点跟golang还是有点不太一样, golang是只要有panic, 则整个程序都会终止
    // 因此, 尽量不要在 main 线程中做太多任务, 将这些任务交由子线程去做, 就算子线程 panic 也不会导致整个程序的结束

    // 5. 啥时候使用panic比较好?
    // 场景一: 示例、原型、测试. 此时我们需要快速开发, 所以我们直接用unwrap即可
    // 场景二: 你确切的知道你的程序是正确时, 可以使用 panic. 因为清楚不可能panic, 我们可以直接用unwrap/expect
    // 场景三: 可能导致全局有害状态时(非预期的错误/后续代码的运行会受到显著影响/内存安全的问题). 其实就是错误不可处理时, 我们进行panic
    //
    // Result<T,E>枚举专用于rust中进行错误处理, 它用来表示函数的返回结果. 成功时返回 Result::Ok(T), 失败时返回 Err(E)
    // 对于 Result 返回我们有很多处理方法, 最简单粗暴的就是 unwrap 和 expect
    // * unwrap: 如果Result=Ok, 则返回T. 如果Result=Err, 则进行panic
    // * expect: 如果Result=Ok, 则返回T. 如果Result=Err, 则进行panic并且能附加我们自定义的信息

    let home: IpAddr = "127.0.0.1".parse().expect("???");
    println!("ipAddr is loopback? result={}", home.is_loopback());

    // 6. panic原理解析
    // (1). 格式化 panic 信息, 然后使用该信息作为参数, 调用 std::panic::panic_any() 函数
    // (2). std::panic::panic_any() 会检查应用是否使用了panic hook.
    //      如果使用了, 该hook函数就会被调用（hook 是一个钩子函数,是外部代码设置的, 用于在 panic 触发时,执行外部代码所需的功能）
    // (3). 当 hook 函数返回后, 当前的线程就开始进行栈展开: 从 panic_any 开始, 如果寄存器或者栈因为某些原因信息错乱了
    //      那很可能该展开会发生异常, 最终线程会直接停止, 展开也无法继续进行
    // (4). 展开的过程是一帧一帧的去回溯整个栈, 每个帧的数据都会随之被丢弃.
    //      但是在展开过程中, 你可能会遇到被用户标记为 catching 的帧（通过 std::panic::catch_unwind() 函数标记）
    //      此时用户提供的 catch 函数会被调用, 展开也随之停止. 当然, 如果 catch 选择在内部调用 std::panic::resume_unwind() 函数
    //      则展开还会继续
    //
    //
    // 还有一种情况,在展开过程中如果展开本身 panic 了, 那展开线程会终止, 展开也随之停止.
    // 一旦线程展开被终止或者完成, 最终的输出结果是取决于"哪个线程 panic"
    // 对于 main 线程,操作系统提供的终止功能 core::intrinsics::abort() 会被调用,最终结束当前的 panic 进程
    // 如果是其它子线程,那么子线程就会简单的终止, 同时信息会在稍后通过 std::thread::join() 进行收集

    std::panic::set_hook(Box::new(|info| {
        println!("Custom panic hook");
        println!("info: {}", info);
    }));

    panic!("Normal panic");
}
