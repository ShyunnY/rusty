use std::rc::Rc;

use async_std::task::block_on;

#[allow(dead_code)]
pub fn hello() {
    // async 在 Rust 依然比较新, 疑难杂症少不了.
    // 而它们往往还处于活跃开发状态, 短时间内无法被解决, 下面一起来看看这些问题以及相应的临时解决方案

    // 1. 在 async 语句中使用错误处理的 '?'
    // one();

    // 2. async 函数和 Send 特征
    two();

    // 3. 在特征中使用 async
    // three();
}

#[allow(dead_code)]
fn one() {
    // 在 async 语句中使用错误处理的 '?'

    // 'async' 语句块和 'async fn' 最大的区别就是 **前者无法显式的声明返回值**
    // 在大多数时候这都不是问题, 但是当配合 ? 一起使用时, 问题就有所不同
    // 我们需要手动标注类型注释, 让编译器理解返回值

    async fn foo() -> Result<u8, String> {
        Ok(1)
    }
    async fn bar() -> Result<u8, String> {
        Ok(1)
    }

    let _fut = async {
        foo().await?;
        bar().await?;

        // 既然编译器无法推断出类型, 那咱就给它更多提示
        // 可以使用 ::< ... > 的方式来增加类型注释
        // Ok::<(),String> 告诉编译器 Err 类型参数 T = String

        // 也可以使用 Result::<(), String>::Ok(()) , 看起来更好理解
        Ok::<(), String>(())
    };
}

#[allow(dead_code)]
fn two() {
    // async 函数和 Send 特征
    // 在多线程章节我们深入讲过 Send trait(在不同线程中传递所有权) 对于多线程间数据传递的重要性
    // 对于 async fn 也是如此, 它返回的 Future 能否在线程间传递的关键在于: '.await' 运行过程中, 作用域中的变量类型是否是 Send
    //
    //  .await 有可能被执行器调度到另一个线程上运行, 所有要求 async 里面的数据实现 Send trait

    {
        #[derive(Default)]
        struct NotSend(Rc<()>);

        async fn bar() {}

        async fn foo() {
            let _x = NotSend::default();
            bar().await;
        }

        block_on(foo());
    }
}
