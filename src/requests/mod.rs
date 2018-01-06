use std::io::{stdin, stdout};
use std::io::{BufWriter, Write};
use std::string::String;
use std::process::{exit};

fn flush_out(message: String)
{
    let mut o = BufWriter::new(stdout());
    match o.write(&message.into_bytes()){_ => {}}
    match o.flush(){_ => {}}
}

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
