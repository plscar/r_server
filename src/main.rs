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

    for stream in listener.incoming() {
        let res=String::new();
        match stream {
            Ok(v) => {
                let myres=handing(v);
                match myres {
                    Ok(myv) => {
                        res=myv;
                    },
                    Err(mye) => println!("发生错误！"),
                }
                },
            Err(e) =>println!("请求错误") ,
        }
        
    }


}

fn handing(stre:TcpStream)->Result<String,io::Error>{
    let mut res=String::new();
    stre.read_to_string(&mut res)?;
    println!("{}",res);    
    Ok("ok".to_string())
}
