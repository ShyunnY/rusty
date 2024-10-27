use tokio::{
    fs::{File, OpenOptions},
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    // 1. tokio中常见的 IO 函数
    // io().await

    // 2. 一个简单的 echo 服务
    echo().await;
}

async fn echo() {
    let tcp_listener = TcpListener::bind("127.0.0.1:9527").await.unwrap();

    // 为了实现目标功能, 必须将 socket `分离成一个读取器和写入器`
    // 任何一个读写器( reader + writer )都可以使用 io::split 方法进行分离, 最终返回一个读取器和写入器
    // 这两者可以独自的使用, 例如可以放入不同的任务中
    // io::split 可以用于任何同时实现了 AsyncRead 和 AsyncWrite 的值, 它的内部使用了 Arc 和 Mutex 来实现相应的功能
    //
    // 如果大家觉得这种实现有些重, 可以使用 Tokio 提供的 TcpStream, 它提供了两种方式进行分离:
    // + TcpStream::split: 会获取字节流的引用, 然后将其分离成一个读取器和写入器.
    //   但由于使用了引用的方式, 它们俩必须和 split 在同一个任务中. 优点就是这种实现没有性能开销, 因为无需 Arc 和 Mutex
    // + TcpStream::into_split: 还提供了一种分离实现, 分离出来的结果可以在任务间移动, 内部是通过 Arc 实现

    //
    // 一个数据如果想在 .await 调用过程中存在, 那它必须存储在当前任务内
    // 当任务因为调度在线程间移动时, 存储在栈上的数据需要进行 "保存和恢复", 过大的栈上变量会带来不小的数据拷贝开销
    // 因此存储大量数据的变量最好放到 **堆上**
    // 举个例子: 如果我们需要在 .await 调用中使用数组, 此时数组是需要存贮在栈上的, 在任务调度过程中是会有可能 Copy 的
    // 所以我们将其存放在 堆上, 不仅可以减少Copy, 还能减少栈大小
    loop {
        let (mut tcp_stream, _) = tcp_listener.accept().await.unwrap();
        println!("build tcp connect");

        // 创建 子Future 异步处理请求
        tokio::spawn(async move {
            // 分离读和写的socket
            let (mut r, mut w) = tcp_stream.split();
            println!("echo msg");
            if io::copy(&mut r, &mut w).await.is_err() {
                eprintln!("faile to copy in echo server")
            }
        });
    }
}

#[allow(dead_code)]
async fn io() {
    // 1. async read
    // AsyncReadExt::read 是一个异步方法可以将数据读入缓冲区( buffer )中, 然后返回读取的字节数
    // 当 read 返回 Ok(0) 时，意味着字节流( stream )已经关闭，在这之后继续调用 read 会立刻完成，依然获取到返回值 Ok(0)
    {
        let mut f = File::open("src/main.rs").await.unwrap();
        let mut buf = [0; 30];
        let n = f.read(&mut buf[..]).await.unwrap();

        println!("1.async read got: {n}");
    }

    // 2. async read_to_end 会从字节流中读取所有的字节
    {
        let mut f = File::open("src/main.rs").await.unwrap();
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).await.unwrap();

        println!(
            "2. async read_to_end got: {:?}",
            String::from_utf8(buf).unwrap()
        );
    }

    // 3. AsyncWriteExt::write 异步方法会尝试将缓冲区的内容写入到写入器( writer )中，同时返回写入的字节数
    // b"some bytes" 可以将一个 &str 字符串转变成一个 `字节数组：&[u8;10]`
    // 然后 write 方法又会将这个 &[u8;10] 的数组类型隐式强转为数组切片: &[u8]
    {
        let mut f = File::create("target/foo.txt").await.unwrap();
        let n = f.write(b"hellom, G.E.M.").await.unwrap();
        println!("3. async write {} byte to target/foo.txt", n);
    }

    // 4. AsyncWriteExt::write_all 将缓冲区的内容全部写入到写入器中：
    {
        // 通过 OpenOptions 构建一个带有选项的 File
        let mut f = OpenOptions::new()
            .write(true)
            .append(true)
            .open("target/foo.txt")
            .await
            .unwrap();

        let n = f.write(b"\nhellom, G.E.M.\n").await.unwrap();
        println!("3. async write_all {} byte to target/foo.txt", n);
    }
}
