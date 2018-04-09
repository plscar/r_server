use std::fs::File;
use std::io;
use std::io::prelude::*;

///获取config.cnf中的特定键所对应的值
/// 
pub fn GetConfig(key:&str)->Result<String,io::Error>
{
    let mut mykey="\"".to_string()+key+&"\"".to_string();
    let mut fil=File::open("config.cnf")?;
    let mut mycontent= String::new();
    fil.read_to_string(&mut mycontent)?;
    let obj:Vec<&str>=mycontent.split("\n").collect();
    for item in &obj {
        let idx=item.find(&mykey);
        println!("myitem=>{}",item);
        match idx {
            Some(v) => {
                let MyObj:Vec<&str>=item.split("=>").collect();
                return Ok(MyObj[1].to_string());
            },
            None => continue,
        }
    }
    
    Ok("".to_string())
}