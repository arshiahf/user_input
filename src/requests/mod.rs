use std::io::{self};
use std::string::String;

pub fn request_input() -> String
{
    //let mut full_input = Vec::new();

    let i = io::stdin();
    let mut in_str = String::new();
    match i.read_line(&mut in_str)
    {
        Ok(_n) => {
            //println!("Input: {}", n);
        }
        //Err(error) => println!("Error: {}", error),
        _ => println!("Error"),
    }
    println!("{}", in_str);
    in_str
}
