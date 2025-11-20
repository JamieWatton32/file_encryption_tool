mod encrypt;
use std::env;
use std::fs;
use std::io;


fn expect_usage(arg: &String)-> String{
    format!("Usage: {} <input> <output>", arg)
}
fn main() ->io::Result<()>{
    let base = std::env::args().nth(0).unwrap();
    
    let pattern = std::env::args().nth(1).expect(&expect_usage(&base));
    let path = std::env::args().nth(2).expect(&expect_usage(&base));
    Ok(())

}
