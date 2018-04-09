use std::net::{TcpListener,TcpStream};
use std::io;
use std::io::prelude::*;
extern crate r_server;

fn main() {
    //读取配置文件中的ip地址
    let ip_obj=r_server::common::config::GetConfig("ip");
    let mut ip=String::new();
    match ip_obj{
        Ok(v)=>{
            if v=="" {
                panic!("为设置ip");
            }else {
                ip=v;
            }
        },
        Err(e)=>panic!("ip获取失败！"),
    }
    println!("listing at  =>{}",ip );
    //监听配置的ip地址
    let listener=TcpListener::bind(&ip).unwrap();
    println!("stoped here");
    for stream in listener.incoming() {
        let stream=stream.unwrap();
        let res= handing(stream);
    }


}

fn handing(mut stre:TcpStream)->Result<String,io::Error>{
    let mut buf=[0;1024];
    stre.read(&mut buf)?;
    println!("request:{}",String::from_utf8_lossy(&buf[..]));
    Ok("cc".to_string())
}
