use core::time;
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
};
use zero::ThreadPool;

/// `#[async_std::main]` 过程宏在内部对 `fn main()` 函数插入了 `block_on()`。 不需要我们手动编写
#[async_std::main]
async fn main() {}

/// `sync` 模式下通过线程池来并发处理请求
#[allow(dead_code)]
fn sync_server(listener: TcpListener) {
    // incoming 会返回一个迭代器, 它每一次迭代都会返回一个新的连接 stream(客户端发起, web服务器监听接收)
    // tcp_stream_result代表了客户端发起网络连接的尝试, 所以有可能出现 "connect faild" 问题, 所以他使用了Result
    //
    // incoming 在迭代器的 next() 中调用了 accept 阻塞式监听连接请求(会阻塞当前线程)

    println!("zero server is running 127.0.0.1:7878 ...");
    let pool = ThreadPool::new(5);
    for tcp_stream_result in listener.incoming() {
        match tcp_stream_result {
            Ok(tcp_stream) => {
                // 1.为每一个 tcp_stream 链接都创建一个线程处理. 下下之策: 这会导致开销很大(经历了线程的 创建 -> 切换 -> 销毁)
                // thread::spawn(|| {
                //     handler_connection(tcp_stream);
                // });

                pool.execute(move || {
                    handler_connection(tcp_stream);
                });
            }
            Err(err) => panic!("connect faild{}", err),
        }
    }

    println!("zero server was shutdown...");
}

fn handler_connection(mut tcp_stream: TcpStream) {
    // 1. 将tcp_stream中的数据读取到缓冲区中
    let buf_reader = BufReader::new(&mut tcp_stream);

    // 2. 迭代消费 buf_reader 中的数据
    // line 切分每一行数据 => map 将Result中的数据掏出来 => take_while 根据predict断言一直获取元素直到返回false就终止后续迭代 => collect 收集所有元素
    // 我们必须使用 take()/take_while(), 因为tcp连接的数据是不断迭代的(除非断开链接), 所以我们使用 take_while() 当不满足要求时直接终止迭代即可
    // let _result: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    let first_line = buf_reader.lines().next().unwrap().unwrap();

    // 由于 match 不会像方法那样 '自动做引用或者解引用', 因此我们需要显式调用: match &request_line[..] ，来获取所需的 &str 类型
    // 其实就是 &first_line[..] == &first_line as &str , 只是前者获取了其切片
    let (response_status, filename) = match &first_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            // 手动模拟休眠 3s, 在单线程下将会阻塞其余请求
            println!("trigger sleep 3000ms");
            thread::sleep(time::Duration::from_millis(3000));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    // 3. 返回响应
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{response_status}\r\nContent-Length: {length}\r\n\r\n\n{contents}");
    tcp_stream.write_all(response.as_bytes()).unwrap();
}
