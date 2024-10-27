use std::io::Cursor;

use bytes::{Buf, Bytes, BytesMut};
use mini_redis::{Frame, Result};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::TcpStream,
    sync::oneshot::Sender,
};

#[derive(Debug)]
pub enum Command {
    Set {
        key: String,
        val: Bytes,
        resp: Response<()>,
    },
    Get {
        key: String,
        resp: Response<Option<Bytes>>,
    },
}

type Response<T> = Sender<Result<T>>;

pub struct Connection {
    /// 该结构体实现了 AsyncWrite 特征
    /// 当 write 方法被调用时, 不会直接写入到 socket 中, 而是先写入到缓冲区中
    ///
    /// **当缓冲区被填满时,其中的内容会自动刷到(写入到)内部的 socket 中, 然后再将缓冲区清空**
    stream: BufWriter<TcpStream>,

    /// 由于读取 stream 只会返回任意多的数据, 它可能返回帧的一部分、一个帧、多个帧，总之这种读取行为是不确定的
    /// 所以我们需要一个 buffer 将数据缓存下来, 然后进行解析 Frame
    /// 解析完毕之后在缓冲区中移除对应的 Frame 数据
    buffer: BytesMut,
}

impl Connection {
    fn new(tcp_stream: TcpStream) -> Self {
        Connection {
            stream: BufWriter::new(tcp_stream),
            buffer: BytesMut::with_capacity(4096),
        }
    }

    async fn read_frame(&mut self) -> Result<Option<Frame>> {
        loop {
            // 首先在缓冲区中尝试解析一个 Frame
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            // 走到这里说明缓冲区的数据不足以解析成一个完整的 Frame
            // 此时我们需要从 tcp_stream 读取数据到 buffer 中
            if self.stream.read_buf(&mut self.buffer).await? == 0 {
                // n == 0 说明对端关闭了连接, 我们需要判断缓冲区内是否还有数据
                // + 缓冲区为空: 代表解析了完整的Frame
                // + 缓冲区不为空: 代表数据发送了一半
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }

    fn parse_frame(&mut self) -> Result<Option<Frame>> {
        // 在 buffer 中读取一个 Frame
        // 解析 Frame
        let mut buf = Cursor::new(&self.buffer[..]);

        // Frame::check 实际上会移动内部的 curson 位置
        match Frame::check(&mut buf) {
            Ok(_) => {
                // 获取当前 position 位置
                let position = buf.position() as usize;

                // 将 position 设置为0
                buf.set_position(0);

                // 解析 frame
                let frame = Frame::parse(&mut buf).unwrap();

                // 将解析的 frame 数据从 buffer 中移除
                self.buffer.advance(position);

                Ok(Some(frame))
            }
            Err(mini_redis::frame::Error::Incomplete) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    async fn write_fram(&mut self, frame: Frame) -> Result<()> {
        // match frame {
        //     Frame::Simple(val) => {
        //         self.stream.write_u8(b'+').await?;
        //         self.stream.write_all(val.as_bytes()).await?;
        //         self.stream.write_all(b"\r\n").await?;
        //     }
        //     Frame::Error(val) => {
        //         self.stream.write_u8(b'-').await?;
        //         self.stream.write_all(val.as_bytes()).await?;
        //         self.stream.write_all(b"\r\n").await?;
        //     }
        //     Frame::Integer(val) => {
        //         self.stream.write_u8(b':').await?;
        //         self.write_decimal(*val).await?;
        //     }
        //     Frame::Null => {
        //         self.stream.write_all(b"$-1\r\n").await?;
        //     }
        //     Frame::Bulk(val) => {
        //         let len = val.len();

        //         self.stream.write_u8(b'$').await?;
        //         self.write_decimal(len as u64).await?;
        //         self.stream.write_all(&val).await?;
        //         self.stream.write_all(b"\r\n").await?;
        //     }
        //     Frame::Array(_val) => unimplemented!(),
        // }

        // self.stream.flush().await;

        Ok(())
    }
}
