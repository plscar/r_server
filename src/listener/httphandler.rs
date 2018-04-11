use std::{
    net::{TcpListener,TcpStream},
    io,
    io::prelude::*,
    fs::File,
    collections::HashMap,
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
    let mut f_path=String::from("");  //记录config中的配置默认文件路径
    //获取config中配置的，默认文件存放目录,如果没有配置，就是本程序的根目录
    let c_rs= config::get_config("defaultWebFiles");
    match c_rs {
        Ok(v_c) => {f_path=v_c},
        Err(_e) =>{},
    }
    
    
    //监听配置的ip地址
    let listener=TcpListener::bind(address)?;
    println!(">>>http服务启动成功<<<\n>>>监听地址:{}<<<",address);
    for mut stream in listener.incoming() {
        let mut stream=stream.unwrap();
        let _res= handler(&mut stream,&mut f_path);
        
    }
    Ok("open success".to_string())
}
///处理http请求
fn handler(stre:&mut TcpStream,defaultpath:&mut str)->Result<String,io::Error>{
    let mut buf=[0;1024];
    stre.read(&mut buf)?;
    let request_msg=String::from_utf8_lossy(&buf[..]);
    println!("request=>\n{}",request_msg);
    let mut my_res=String::from("");               //用于记录所返回的值
    let mut status=String::from("200 OK");      //用于记录返回状态
    
    //筛选get或post请求
    if request_msg.contains("GET ") {
        let para_split=splitRequires(&request_msg,"GET ",defaultpath);
        //对get进行处理
        let r=do_get(para_split.ok().expect("err=>参数解析失败"));
        match r {
            Ok(v) => {
                let v_cl=v.clone();
                let res_head=v_cl.get(0..5);
                let vcc=res_head.expect("none");
                if vcc.contains("err=>") 
                {
                    status="405 ERR".to_string();
                }
                else
                {
                    my_res=v;
                }
            },
            Err(_e) => {
                status="404 NO FOUND".to_string();
            },
        }
    }
    else {
        //对post进行处理
        let para_split=splitRequires(&request_msg,"POST ",defaultpath);
        // my_res=do_post(&request_msg);
    }

    //返回请求
    let _resp= response(stre,&status,&my_res)?;
    
    Ok("cc".to_string())
}
/// 处理get请求，get请求只用于请求html文件
/// paras:请求所携带的参数，如request:get /index.html?id=1 http/1.1
fn do_get(paras:request_para) -> Result<String,io::Error> {

    let mut f=File::open(paras.path)?;
    println!("paras==>\n{}",paras.path_para );
    if paras.path_para=="stop" {
        panic!("手动停止！");
    }
    let mut f_res=String::new();
    f.read_to_string(&mut f_res);
    Ok(f_res)
}

#[derive(Clone)]
struct request_para {
    path:String,
    path_para:String,
    body_para:String,
}

/// 处理post请求，post请求用于数据请求
fn do_post(paras:&str)->Result<String,io::Error>
{
    println!("post=>\n{}",paras );

    Ok("C".to_string())
}
///拆分请求，拆分成路径/路径后附带的参数/request body中的参数
fn splitRequires(paras:&str,method:&str,defaultPath:&mut str)->Result<request_para,io::Error>
{
    
    //切割get请求，分割get及其后面的内容
    let idx_get=paras.find(method);
    
    let idx_get_value=idx_get.expect("err=>请求解析失败！");
    let (_,second)=paras.split_at(idx_get_value+3);
    //在分割后的后端内容中，截取第一行，其中含有了要访问的文件地址
    let idx_para = second.find("\r\n");
    let idx_para_value=idx_para.expect("err=>请求解析失败！");
    let (mut myfirst,mut mysecond)=second.split_at(idx_para_value);
    //在分割出的包含有文件地址的行中，根据空白进行分割，空白后面是http版本，前面是文件地址及参数
    let mut r=myfirst.split_whitespace();
    let request_split=r.next().expect("err=>请求无文件地址");
    //拆解出路径和路径后的参数
    let request_path:Vec<&str>=request_split.split("?").collect();    
    let mut f_res=String::new();      //记录请求文件的请求结果
    let body_idx=mysecond.find("\r\n\r\n").expect("err=>报文错误！");
    let (_,body)=mysecond.split_at(body_idx);
    //参数解析一：路径
    let myparas=request_para{
        path:defaultPath.to_string()+&request_path[0],
        path_para:if request_path.len()>1{request_path[1].to_string()}else {"".to_string()},
        body_para:body.to_string(),
        };
    Ok(myparas)
}
///用于向客户端返回数据
fn response(stre:&mut TcpStream,status:&str,res_msg:&str)->Result<String,io::Error>
{
    let respon=format!("HTTP/1.1 {} \r\n\r\n{}",status,res_msg);
    stre.write(respon.as_bytes())?;
    stre.flush()?;
    Ok("success".to_string())
}