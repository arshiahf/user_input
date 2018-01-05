use std::io::{self};
use std::string::String;
use std::process::{exit};

pub fn request_input(in_str:&mut String)
{
    let i = io::stdin();
    match i.read_line(in_str)
    {
        Ok(_n) => {
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
            exit(1);
        }
    }
}
