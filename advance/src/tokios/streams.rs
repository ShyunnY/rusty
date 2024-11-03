use std::task::Poll;

use tokio_stream::StreamExt;

#[allow(dead_code)]
pub async fn hello() {
    // tokio中的异步迭代器
    println!("tokio 中的异步迭代器");

    // 1. 异步 stream 的基本使用
    // 需要引入 "tokio_stream::StreamExt" 拓展 Iter
    let mut stream = tokio_stream::iter([1, 2, 3, 4, 5]);
    while let Some(v) = stream.next().await {
        println!("异步的 stream: {v}");
    }

    // 2. 异步 stream 的适配器
    /*
       在前面章节中, 我们了解了迭代器有两种适配器:
       + 迭代器适配器: 会将一个迭代器转变成另一个迭代器, 例如 map，filter 等
       + 消费者适配器: 会消费掉一个迭代器, 最终生成一个值. 例如 collect 可以将 "迭代器收集成一个集合"

       与迭代器类似, stream 也有适配器, 例如一个 stream 适配器可以将一个 stream 转变成另一个 stream
       例如 map、take 和 filter

    */
    let stream = tokio_stream::iter([1, 2, 3, 4, 5, 6]);
    let mut events = stream.filter(|x| if x % 2 == 0 { true } else { false });
    while let Some(v) = events.next().await {
        println!("异步 stream 的适配器: {v}");
    }
    // filter_map 同时提供了 filter + map 功能
    // + Some(T): 代表 filter 通过, 返回 T
    // + None: 代表 filter 不通过
    let stream = tokio_stream::iter([1, 2, 3, 4, 5, 6]);
    let res: Vec<Option<i32>> = stream
        .filter_map(|v| if v % 2 == 0 { None } else { Some(Some(v)) })
        .collect()
        .await;
    println!("res: {:#?}", res);

    // 3. 实现异步 stream 特征
    {
        struct Interval {
            count: i32,
            is_ok: bool,
            handle: bool,
        }

        impl tokio_stream::Stream for Interval {
            type Item = ();

            fn poll_next(
                mut self: std::pin::Pin<&mut Self>,
                _cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<Option<Self::Item>> {
                if self.count > 3 {
                    return Poll::Ready(Some(()));
                }
                self.as_mut().count += 1;
                Poll::Pending
            }
        }
    }
}
