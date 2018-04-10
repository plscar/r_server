use std::{
    fs::File,
    io,
    io::prelude::*
};

///获取config.cnf中的特定键所对应的值
pub fn get_config(key:&str)->Result<String,io::Error>
{
    let mykey="\"".to_string()+key+&"\"".to_string();
    let mut fil=File::open("config.cnf")?;
    let mut mycontent= String::new();
    fil.read_to_string(&mut mycontent)?;
    let obj:Vec<&str>=mycontent.trim().split("\n").collect();
    for item in &obj {
        let idx=item.find(&mykey);
        println!("myitem___{}\n",item);
        //如果idx中有值，也就是找到了指定的key，则返回对应的value
        if idx.is_some()
        {
            let myobj:Vec<&str>=item.split("=>").collect();
            return Ok(myobj[1].trim_matches('"').to_string());
        }
    }
    Ok("127.0.0.1:1196".to_string())
}   