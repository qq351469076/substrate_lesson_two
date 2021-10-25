use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn handle_connection(mut stream: TcpStream) {
    // 构建缓冲器， 用来缓冲数据
    let mut buffer = [0; 1024];

    // 从Steam流中读取数据， 放到缓冲器里
    stream.read(&mut buffer).unwrap();

    // 将流的信息变成字符串
    let str = String::from_utf8_lossy(&buffer[..]);

    // 遍历split迭代器
    for line in str.split("\r\n") {
        // 继续切割
        let query: Vec<&str> = line.split(" ").collect();

        // 获取param参数的值
        if query[1].contains("param") {
            // 拼接response的字符串
            let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", &query[1][8..]);

            // 将获取的param值写入到response
            stream.write(response.as_bytes()).unwrap();
            // 刷新resp
            stream.flush().unwrap();
        }
        break;
    }
}

fn main() {
    // 在本地12306端口上监听请求，出错抛异常
    let listener = match TcpListener::bind("127.0.0.1:22222") {
        Ok(listener) => listener,
        // 匹配Error的错误类型
        Err(e) => match e.kind() {
            // 当错误类型为AddrInUse时，确定是端口被占用
            std::io::ErrorKind::AddrInUse => {
                panic!("端口已被使用")
            }
            // 为其他任何错误时，输出错误
            _ => {
                panic!("{:?}", e)
            }
        },
    };

    // 监听每一个Stream
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // 处理每一个请求
        handle_connection(stream);
    }
}
