extern crate r_server;

fn main() {
    //读取配置文件中的ip地址
    let ip_obj=r_server::common::config::get_config("ip");
    let mut ip=String::new();

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
    r_server::listener::httphandler::open_server(&ip);
}


