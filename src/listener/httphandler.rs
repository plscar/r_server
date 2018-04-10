use std::{
    net::{TcpListener,TcpStream},
    io,
    io::prelude::*,
    fs::File
    };
use common::config;
///主方法，开启服务
/// address:监听地址
/// max_length:每次处理的请求最大长度
pub fn open_server(address:&str){
    let myres= mainlistener(address);
    match myres {
        Ok(_v) => println!("成功运行结束"),
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
    println!("request=>\n{}",request_msg);
    let mut my_res=String::from("");               //用于记录所返回的值
    let mut status=String::from("200 OK");      //用于记录返回状态
    //筛选get或post请求
    if request_msg.contains("GET") {
        //对get进行处理
        let r=do_get(&request_msg);
        match r {
            Ok(v) => {
                let v_cl=v.clone();
                let res_head=v_cl.get(0..5);
                match res_head {
                    Some(vcc) => {
                        println!("vcc=>{}",vcc);
                        if vcc.contains("err=>") {
                            status="405 ERR".to_string();
                        }
                        else
                        {
                            my_res=v;
                        }
                    },
                    None => {my_res=v;},
                }
                
            },
            Err(_e) => {
                status="404 NO FOUND".to_string();
            },
        }
    }
    else {
        //对post进行处理
        
        // my_res=do_post(&request_msg);
    }

    //返回请求
    let _resp= response(stre,&status,&my_res)?;
    
    Ok("cc".to_string())
}
/// 处理get请求，get请求只用于请求html文件
/// paras:请求所携带的参数，如request:get /index.html?id=1 http/1.1
fn do_get(paras:&str) -> Result<String,io::Error> {
    //切割
    let idx_get=paras.find("GET ");
    match idx_get {
        Some(v) => {
            let (_first,second)=paras.split_at(v+3);
            let idx_para = second.find("\n");
            match idx_para {
                Some(v) => {
                    let (mut myfirst,mut _mysecond)=second.split_at(v);
                    let mut r=myfirst.split_whitespace();
                    match r.next() {
                        Some(v) => {
                            let request_path:Vec<&str>=v.split("?").collect();
                            let c_rs= config::get_config("defaultWebFiles");
                            let mut f_path=String::new();
                            let mut f_res=String::new();
                            match c_rs {
                                Ok(v_c) => {f_path=v_c},
                                Err(_e) =>{},
                            }
                            let mut f=File::open(f_path.to_string()+request_path[0])?;
                            f.read_to_string(&mut f_res);
                            println!("res=>\n{}",f_res );
                            return Ok(f_res);
                        },
                        //get请求无路径
                        None => {
                            return Ok("err=>get请求无路径".to_string());
                        },
                    }
                },
                None => {return Ok("err=>get请求无换行符".to_string())},
            }

        },
        None => {return Ok("err=>无法识别的请求".to_string())},
    }
}

// /// 处理post请求，post请求用于数据请求
// fn do_post(paras:&str)->Result<String,io::Error>
// {
//     Ok("C")
// }

///用于向客户端返回数据
fn response(mut stre:TcpStream,status:&str,res_msg:&str)->Result<String,io::Error>
{
    let respon=format!("HTTP/1.1 {} \r\n\r\n{}",status,res_msg);
    stre.write(respon.as_bytes())?;
    stre.flush()?;
    Ok("success".to_string())
}