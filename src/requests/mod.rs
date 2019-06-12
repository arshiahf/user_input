// Collection of functions that allow the programmer to request user input

use interpret::{parse_string_to_vec};
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

// Requests command line input and places it into a provided String
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

// Requests command line input and places it into a provided Vector
/* Uses whitespace splitting for variable creation, making it so no variable may be more than
 one word long */
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

// Requests command line input and places it into a provided tuple
/* Uses whitespace splitting for variable creation, making it so no variable may be more than
 one word long */
 /*
pub fn request_input_to_tuple<T>(message: &str, in_tup: &mut (), size:i32) where T: FromStr
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

    parse_string_to_tuple(in_str, in_tup, size);
}
*/

// Programmer-side function to create a series of files or directories from an input String Vector
// Note: Single directories or files may be made using a one-item Vector
pub fn new_directories_files(root_dir:String, names:Vec<String>, input_type:String)
{
    input_type.to_lowercase();
    match input_type.as_str()
    {
        "file" => {
            for file in names
            {
                let full_path = root_dir.clone() + &file.clone();
                match File::create(full_path.as_str())
                {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("{:?}", err);
                        exit(1);
                    }
                }
            }
        }
        "dir" => {
            for dir in names
            {
                let mut builder = DirBuilder::new();
                let full_path = root_dir.clone() + "/" + &dir.clone();
                match builder.recursive(true).create(full_path.as_str())
                {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("{:?}", err);
                        exit(1);
                    }
                }
            }
        }
        _ => {
            eprintln!("Error: Invalid selection. Please select either dir or file.");
        }
    }
}
