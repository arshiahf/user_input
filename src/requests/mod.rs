use std::io::{self};
use std::string::String;
use std::process::{exit};

pub fn request_input(mut in_str:String)
{
    let i = io::stdin();
    match i.read_line(&mut in_str)
    {
        Ok(_n) => {
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
            exit(1);
        }
    }
}
