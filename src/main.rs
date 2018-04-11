extern crate r_server;
use r_server::{
    common::config,
    listener::httphandler
    }; //rust 1.25.0新特性，可以如此写引用

fn main() {
    //读取配置文件中的ip地址
    let ip_obj=config::get_config("ip");
    let mut ip;

    match ip_obj{
        Ok(v)=>{
            println!("my_v==>{}",v);
            if v=="null" {
                panic!("未设置ip");
            }else {
                ip=v;
            }
        },
        Err(_e)=>panic!("ip获取失败！"),
    }
    println!("listing at  =>{}",ip );
    httphandler::open_server(&ip);
}


