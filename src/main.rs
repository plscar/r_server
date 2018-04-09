use std::net::{TcpListener,TcpStream};
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
                res=handing(v);
                },
            Err(e) =>println!("请求错误") ,
        }
        
    }


}

fn handing(stream:TcpStream)->String{
    println!("{}",v.read_to_string());    
    "ok".to_string()
}
