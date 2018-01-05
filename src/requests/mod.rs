use std::io::{self};
use std::string::String;

pub fn request_input() -> String
{
    //let mut full_input = Vec::new();

    let i = io::stdin();
    let mut in_str = String::new();
    match i.read_line(&mut in_str)
    {
        Ok(n) => {
            println!("Input: {}", n);
            println!("{}", in_str);
        }
        //Err(error) => println!("Error: {}", error),
        _ => println!("Error"),
    }

    in_str
}
