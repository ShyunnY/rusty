use core::time;
use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

#[allow(dead_code)]
pub fn hello() {
    // 线程间的消息传递
    /*
    在多线程间有多种方式可以共享、传递数据, 最常用的方式就是通过"消息传递"或者将"锁和Arc联合使用"
    Golang的并发设计精髓: "不要通过共享内存来通信, 而是通过通信来共享内存"
    */

    // (1) 消息通道
    // Rust在标准库里提供了消息通道(channel): 一个通道应该支持多个发送者和接收者

    // (2) 多发送者, 单接受者
    // 标准库提供了通道`std::sync::mpsc`, 其中mpsc是 "multiple producer, single consumer" 的缩写
    // 代表了该通道支持"多个发送者,但是只支持唯一的接收者"
    // 当然支持多个发送者也意味着"支持单个发送者"
    /*
        有几个点需要注意:
        1.tx,rx对应发送者和接收者, 它们的类型由编译器自动推导(或者手动标注类型注释). 如果确定了类型, 则只能发送/接受同一类型
        2.接收消息的操作 `rx.recv()` 会"阻塞当前线程", 直到读取到值或者通道被关闭(类似于Golang的阻塞channel)
        3.需要使用move将tx的所有权转移到子线程的闭包中(或者使用Arc咯)
        4.rece读取信息是阻塞的!!!

        send和rece都会返回一个Result, 原因如下:
        1.reve被Drop了: 例如接收者被drop导致了发送的值不会被任何人接收, 此时继续发送毫无意义
        2.send被Drop了: 当发送者关闭时它也会接收到一个错误, 用于说明不会再有任何值被发送过来
    */
    {
        // 创建一个消息通道(message Channel), 返回一个元组: (发送者, 接收者)
        let (send, rece): (Sender<i32>, Receiver<i32>) = mpsc::channel();

        thread::spawn(move || {
            // drop(send);
            send.send(10).unwrap();
            println!("(2) 发送者发送 result = {}", 10);
        });

        let result = rece.recv().unwrap();
        println!("(2) 接受者接收到 result = {result}");
    }

    // (3) 不阻塞的 try_recv 方法
    // 可以使用 `try_recv` 尝试接收一次消息, 该方法并不会阻塞线程
    // 当通道中没有消息时, 它会立刻返回一个错误:
    // * Empty: 当前发送者还活跃, 只是没有消息发送过来
    // * Disconnect: 当前发送者已经断开连接, 永远不会有新的消息了
    {
        // 创建一个消息通道(message Channel), 返回一个元组: (发送者, 接收者)
        let (send, rece): (Sender<i32>, Receiver<i32>) = mpsc::channel();

        thread::spawn(move || {
            send.send(10).unwrap();
            println!("(3) 发送者发送 result = {}", 10);
        });

        thread::sleep(time::Duration::from_millis(30));
        println!("(3) 接受者接收到 result = {:?}", rece.try_recv());
        println!("(3) 接受者接收到 result = {:?}", rece.try_recv());
    }

    // (4) 传输具有所有权的数据
    /*
       使用通道来传输数据, 一样要遵循 Rust 的所有权规则:
       1.若值的类型实现了 Copy 特征, 则直接"复制一份该值", 然后传输过去. e.g. i32,u32这些
       2.若值没有实现 Copy, 则它的所有权会被转移给接收端(所有权发生了Move转移), 在发送端继续使用该值将报错

       Rust还是安全！假如没有所有权的保护, String字符串将被两个线程同时持有
       任何一个线程对字符串内容的修改都会导致另外一个线程持有的字符串"被改变"
    */
    {
        // 创建一个消息通道(message Channel), 返回一个元组: (发送者, 接收者)
        let (send, rece): (Sender<String>, Receiver<String>) = mpsc::channel();

        thread::spawn(move || {
            let msg = String::from("我是密文");
            send.send(msg).unwrap();
            // println!("(4) 发送者发送 result = {}", msg); // 此时不能使用该类型了, 因为"所有权被转移到channel中了"
        });

        println!("(4) 接受者接收到了 result = {:?}", rece.recv().unwrap());
    }

    // (5) 使用 for 进行循环接收
    {
        // 创建一个消息通道(message Channel), 返回一个元组: (发送者, 接收者)
        let (send, rece): (Sender<String>, Receiver<String>) = mpsc::channel();

        thread::spawn(move || {
            let msgs = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for ele in msgs.into_iter() {
                thread::sleep(time::Duration::from_millis(100));
                send.send(ele).unwrap();
            }
        });

        // rece实现了Interator, 它会一直调用 "self.rx.recv().ok()" 接受数据
        // (注意: 如果send被Drop了, recv将无法正常的接收到数据)
        for ele in rece.into_iter() {
            println!("(5) Got: {}", ele);
        }
    }

    // (6) 使用多发送者, 以及单接受者
    /*
       有几点需要注意:
       1.需要 "所有的发送者都被drop掉后" , 接收者 "rx才会收到错误" , 进而跳出for循环最终结束主线程
       2.这里虽然用了clone但是并不会影响性能, 因为它并不在热点代码路径中, 仅仅会被执行一次
       3.由于两个子线程谁先创建完成是未知的, 因此哪条消息先发送也是未知的, 最终主线程的输出顺序也不确定
    */
    {
        let (send, rece): (Sender<String>, Receiver<String>) = mpsc::channel();
        // 我们没有使用Arc进行拷贝, 而是直接进行Clone
        // 如果使用Arc我们还需要手动Drop掉原始副本, 这并不方便和合理(还不如直接进行Clone呢!)
        let send_clone = send.clone();

        thread::spawn(move || {
            send.send(String::from("origin send message")).unwrap();
        });

        thread::spawn(move || {
            send_clone.send(String::from("clone send message")).unwrap();
        });

        for ele in rece.into_iter() {
            println!("(6)接受到多发送者的信息 Got: {}", ele);
        }
    }

    // (7) 消息顺序
    // 对于通道而言, 消息的发送顺序和接收顺序是一致的, 满足FIFO原则(先进先出)
    // 这点和Golang一样, 我们总是能知道消息的传递都是按序的

    // (8) 同步和异步的通道
    // Rust 标准库的mpsc通道其实分为两种类型: "同步" 和 "异步"
    //
    // 1. 异步通道
    // 默认情况下使用mpsc::channel创建的都是异步的通道( 也就是 "发送端发送信息是不会被阻塞的" )
    // 这看起来就像是Golang中的 "有缓冲通道", 发送者往里发完数据就走, 根本不会被阻塞!
    {
        let (send, rece): (Sender<String>, Receiver<String>) = mpsc::channel();

        thread::spawn(move || {
            println!("(8.1) 子线程发送前");
            send.send(String::from("delay message")).unwrap();
            println!("(8.1) 子线程发送后(我的发送没有被阻塞!)");
        });

        // main线程睡眠模拟rece阻塞
        println!("(8.1) main线程睡眠500ms前");
        thread::sleep(time::Duration::from_millis(500));
        println!("(8.1) main线程睡眠500ms后");
        println!("(8.1) main线程 Got: {}", rece.recv().unwrap());
    }
    //
    // 2. 同步通道
    // 我们也可以将channel声明为同步的(发送消息是阻塞的)
    // 同步通道发送消息是阻塞的, 只有在消息被接收后才解除阻塞
    // 这看起来就像是Golang中的 "无缓冲通道", 发送者往里发完数据后会阻塞到接受者读取完数据才能走
    //
    // 主线程由于睡眠被阻塞导致无法接收消息, 因此子线程的发送也一直被阻塞, 直到主线程结束睡眠并成功接收消息后发送才成功:
    // 说明"只有接收消息彻底成功后, 发送消息才算完成"
    {
        let (send, rece) = mpsc::sync_channel(0);

        thread::spawn(move || {
            println!("(8.2) 子线程发送前");
            send.send(String::from("delay message")).unwrap();
            println!("(8.2) 子线程发送后(我的发送被阻塞了!)");
        });

        // main线程睡眠模拟rece阻塞
        println!("(8.2) main线程睡眠500ms前");
        thread::sleep(time::Duration::from_millis(500));
        println!("(8.2) main线程睡眠500ms后");
        println!("(8.2) main线程 Got: {}", rece.recv().unwrap());
    }
    //
    // 3. 消息缓存
    // 在创建同步通道时, 我们传递了一个参数0: mpsc::sync_channel(0), 这是什么意思呢？
    // 该值可以用来指定同步通道的消息缓存条数, 当你设定为 "N" 时,发送者就可以无阻塞的往通道中发送N条消息
    // 当消息缓冲队列满了后, 新的消息发送将被阻塞(如果没有接收者消费缓冲队列中的消息, 那么第N+1条消息"就将触发发送阻塞")
    // 这其实就类似于Golang的: ch := chan(int,2) // "创建一个指定大小的通道, 当消息堆积超出 N 时, 发送会被阻塞"
    //
    // 问题又来了: 异步通道创建时完全没有这个缓冲值参数 mpsc::channel(), 它的缓冲值怎么设置呢？
    // 事实上异步通道的缓冲上限取决于你的内存大小, 不要撑爆就行
    // 通道存放数据也会占用内存的, 所以我们尽量不要使用异步通道, 即使使用也要及时消费
    {
        let (send, rece) = mpsc::sync_channel(1);

        thread::spawn(move || {
            println!("(8.3) 子线程发送前");
            send.send(String::from("delay message1")).unwrap();
            println!("(8.3) 第一条信息的发送没有被阻塞");
            send.send(String::from("delay message2")).unwrap();
            println!("(8.3) 第二条信息的发送被阻塞了...");
        });

        // main线程睡眠模拟rece阻塞
        println!("(8.3) main线程睡眠500ms前");
        thread::sleep(time::Duration::from_millis(500));
        println!("(8.3) main线程睡眠500ms后");
        for ele in rece.into_iter() {
            println!("(8.3) main线程 Got: {}", ele);
            thread::sleep(time::Duration::from_millis(100));
        }
    }

    // (9) 通道关闭（channel closed）
    /*
        之前我们数次提到了通道关闭, 并且提到了当通道关闭后, 发送消息或接收消息将会报错. 那么如何关闭通道呢？
        很简单: 所有发送者被drop或者所有接收者被drop后, 通道会自动关闭

        神奇的是: 这件事是在编译期实现的, 完全没有运行期性能损耗！只能说Rust的 Drop 特征 YYDS
    */

    // (10) 传输多种类型的数据
    // 一个消息通道只能传输一种类型的数据, 如果你想要传输多种类型的数据, 可以为每个类型创建一个通道
    // 你也可以使用枚举类型来实现, 或者使用特征对象来实现
    // 有一点需要注意: Rust会"按照枚举中占用内存 "最大" 的那个成员进行内存对齐", 这意味着就算你传输的是枚举中占用内存最小的成员
    // 它占用的内存依然和最大的成员相同, 因此会造成内存上的浪费
    {
        // example: 用枚举来发送不同类型信息
        enum Message {
            U8(u8),
            STRING(String),
        }

        let (send, rece) = mpsc::channel();
        thread::spawn(move || {
            println!("(9) 发送U8信息");
            send.send(Message::U8(10)).unwrap();

            println!("(9) 发送STRING信息");
            send.send(Message::STRING(String::from("enum msg")))
                .unwrap();
        });

        // 读取是阻塞式的, 除非我们使用 try_rece()
        // 或者等到send被 Drop 掉了
        for ele in rece {
            match ele {
                Message::U8(v) => println!("(9) 接受到 Message::U8 数据 Got {}", v),
                Message::STRING(v) => println!("(9) 接受到 Message::STRING 数据 Got {}", v),
            }
        }
    }

    // (10) 一个小坑
    // 通道关闭的两个条件: 发送者全部drop或接收者被drop, 要结束for循环显然是要求"发送者全部drop"
    // 如果我们不对原始的send进行 Drop, 则主线程会一直阻塞
    // 在该作用域中, 对于rece来说存在4个'send', 线程中的三个send会在线程结束时自动Drop
    // 那么在rece循环前, 主send如果不drop的话会导致永远有一个send存在, 那么for循环将会永远卡死
    //
    // 所以我们需要确保: 发送者全部 Drop 或接收者被 Drop
    {
        let (send, rece) = mpsc::channel();

        for ele in 1..=3 {
            let s_clone = send.clone();

            thread::spawn(move || {
                println!("(10) {} thread send msg", ele);
                s_clone.send(ele).unwrap();
            });
        }

        // 这个Drop很关键
        drop(send);
        for ele in rece {
            println!("(10) rece msg: {}", ele);
        }
        println!("(10) Done!");
    }
}
