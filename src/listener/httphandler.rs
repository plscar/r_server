use std::net::{TcpListener,TcpStream};
use std::io;
use std::io::prelude::*;
///主方法，开启服务
/// address:监听地址
/// max_length:每次处理的请求最大长度
pub fn open_server(address:&str){
    let myres= mainlistener(address);
    match myres {
        Ok(v) => println!("成功运行结束"),
        Err(_e) => println!("http服务启动失败！[{}]",_e),
    }
}
///开启对端口的监听，并将请求进行向下传递
fn mainlistener(address:&str)->Result<String,io::Error>
{
    //监听配置的ip地址
    let listener=TcpListener::bind(address)?;
    println!(">>>http服务启动成功<<<\n>>>监听地址:{}<<<",address);
    for stream in listener.incoming() {
        let stream=stream.unwrap();
        let _res= handler(stream);
    }
    Ok("open success".to_string())
}
///处理http请求
fn handler(mut stre:TcpStream)->Result<String,io::Error>{
    let mut buf=[0;1024];
    stre.read(&mut buf)?;
    let request_msg=String::from_utf8_lossy(&buf[..]);

    Ok("cc".to_string())
}