use futures::executor::block_on;

#[allow(dead_code)]
pub fn hello() {
    {
        // async标注的函数返回的是一个 future 而已, 并不会进行执行(惰性执行)
        // 只有上了状态机才能真正的进行执行(也就是说需要一个执行器才能进行执行)
        async fn go() {
            // 想要在 async 函数中调用其他 async 函数
            // 两种解决方法: 使用.await语法或者对Future进行轮询(poll)
            come().await;
            println!("go go go!")
        }

        async fn come() {
            println!("come come come on!")
        }

        block_on(go());
    }
}
