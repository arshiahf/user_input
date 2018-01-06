// Collection of functions that allow the programmer to request user input

use std::io::{stdin, stdout};
use std::io::{BufWriter, Write};
use std::string::String;
use std::process::{exit};

// Places a desired message into the system buffer then flushes it to print it on screen
fn flush_out(message: String)
{
    let mut o = BufWriter::new(stdout());
    match o.write(&message.into_bytes()){_ => {}}
    match o.flush(){_ => {}}
}

// Requests input and places it into a provided String
pub fn request_input(message: &str, in_str: &mut String)
{
    let flushable = String::from(message);
    flush_out(flushable);
    let i = stdin();
    match i.read_line(in_str)
    {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error: {:?}", err);
            exit(1);
        }
    }
}
