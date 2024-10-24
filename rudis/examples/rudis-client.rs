use mini_redis::{client, Result};

/// `#[tokio::main]` 宏将 `async fn main`` 隐式的转换为 `fn main`` 的同时还 **对整个异步运行时进行了初始化**
#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("localhost:6379").await?;

    // set key-val
    client.set("name", "z3".into()).await?;
    // get key-val
    if let Some(res) = client.get("name").await? {
        println!("从服务端获取到结果: {:?}", res);
    }

    Ok(())
}
