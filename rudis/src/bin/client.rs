use log::{error, info};
use mini_redis::client;
use rudis::Command;
use tokio::sync::{mpsc, oneshot};

/// `#[tokio::main]` 宏将 `async fn main`` 隐式的转换为 `fn main`` 的同时还 **对整个异步运行时进行了初始化**
#[tokio::main]
async fn main() {
    env_logger::init();

    let (tx1, mut rx) = mpsc::channel(32);
    let tx2 = tx1.clone();

    let task_1 = tokio::spawn(async move {
        let (oneshot_tx, oneshot_rx) = oneshot::channel();

        // 发送 SET 命令
        tx1.send(Command::Set {
            key: "name".to_string(),
            val: "bar".into(),
            resp: oneshot_tx,
        })
        .await
        .unwrap();

        // 等待回复
        oneshot_rx.await.unwrap().unwrap();
    });
    let task_2 = tokio::spawn(async move {
        let (oneshot_tx, oneshot_rx) = oneshot::channel();

        // 发送 GET 命令
        tx2.send(Command::Get {
            key: "name".to_string(),
            resp: oneshot_tx,
        })
        .await
        .unwrap();

        // 等待回复
        let val = oneshot_rx.await.unwrap().unwrap();
        info!("Get command Got: {:?}", val);
    });

    let manager = tokio::spawn(async move {
        let mut client = match client::connect("127.0.0.1:6379").await {
            Ok(client) => client,
            Err(err) => {
                error!("failed to connect rudis server by err: {}", err);
                return;
            }
        };

        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Set { key, val, resp } => {
                    let res = client.set(&key, val).await;

                    let _ = resp.send(res);
                }
                Command::Get { key, resp } => {
                    let res = client.get(&key).await;

                    let _ = resp.send(res);
                }
            }
        }
    });

    task_1.await.unwrap();
    task_2.await.unwrap();
    manager.await.unwrap();
}
