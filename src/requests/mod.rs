// Collection of functions that allow the programmer to request user input

use interpret::parse_string_to_vec;
use std::str::FromStr;
use std::io::{stdin, stdout};
use std::io::{BufWriter, Write};
use std::string::String;
use std::process::{exit};
use std::fs::{File, DirBuilder};

// Places a desired message into the system buffer then flushes it to print it on screen
fn flush_out(message: String)
{
    let mut o = BufWriter::new(stdout());
    match o.write(&message.into_bytes()){_ => {}}
    match o.flush(){_ => {}}
}

// Requests input and places it into a provided String
pub fn request_input_string(message: &str, in_str: &mut String)
{
    let flushable = String::from(message);
    flush_out(flushable);
    let i = stdin();
    match i.read_line(in_str)
    {
        Ok(_) => {
            in_str.pop();
            in_str.pop();
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
            exit(1);
        }
    }
}

// Requests input and places it into a provided Vector
pub fn request_input_to_vec<T>(message: &str, in_vec: &mut Vec<T>) where T: FromStr
{
    let flushable = String::from(message);
    flush_out(flushable);
    let i = stdin();
    let mut in_str:String = String::new();
    match i.read_line(&mut in_str)
    {
        Ok(_) => {
            in_str.pop();
            in_str.pop();
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
            exit(1);
        }
    }

    parse_string_to_vec(in_str, in_vec);
}

pub fn new_directory(root_dir:String, target:String)
{
    let mut builder = DirBuilder::new();
    let path = root_dir.clone() + &target;
    match builder.recursive(true).create(path.as_str())
    {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{:?}", err);
            exit(1);
        }
    }
}

pub fn new_files(path:String, mut file_names:Vec<String>)
{
    let mut file_name = file_names.pop();
    let full_path = path.clone() + &file_name.clone().unwrap();
    while file_name != None
    {
        match File::create(full_path.as_str())
        {
            Ok(_) => {
                println!("{}", file_name.clone().unwrap());
                file_name = file_names.pop();
            }
            Err(err) => {
                eprintln!("{:?}", err);
                exit(1);
            }
        }
    }
}
