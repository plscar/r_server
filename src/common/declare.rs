use std::collections::HashMap;


let myfunctions=HashMap::new();
///此宏用于声明一个函数，并将函数保存到一个
/// declare!("myfn",fn(cc)={println("my funciton {}!",cc);})
pub macro_rules! declare {
    ($mod_name:expr,$fn_name:tt,$fn_body:ff) => (myfunctions.insert($mod_name.to_string()+$fn_name, $fn_body))
}

///得到函数
pub GetFns(key:&str)->String
{
    myfunctions.get(key)
}
