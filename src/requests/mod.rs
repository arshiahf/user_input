use std::io::{self};
use std::string::String;
use std::process::{exit};

pub fn request_input(mut in_str:String) -> String
{
    //let mut full_input = Vec::new();

    let i = io::stdin();
    //let mut in_str = String::new();
    match i.read_line(&mut in_str)
    {
        Ok(_n) => {
            println!("{}", in_str);
            return in_str
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
            exit(1);
        }
    }
}
