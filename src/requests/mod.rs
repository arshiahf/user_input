use std::io::{stdin, stdout};
use std::io::Write;
use std::string::String;
use std::process::{exit};

pub fn request_input(in_str:&mut String)
{
    let mut o = stdout();
    let i = stdin();
    match i.read_line(in_str)
    {
        Ok(_) => {
            match o.flush()
            {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("{:?}", err);
                    exit(1);
                }
            }
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
            exit(1);
        }
    }
}
