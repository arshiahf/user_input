use std::io::{stdin, stdout};
use std::io::{BufWriter, Write};
use std::string::String;
use std::process::{exit};

fn flush_out(message: &[u8])
{
    let mut o = BufWriter::new(stdout());
    match o.write(message)
    {
        _ => {}
    }

    match o.flush()
    {
        _ => {}
    }
}

pub fn request_input(message: &[u8], in_str: &mut String)
{
    flush_out(message);
    let i = stdin();
    match i.read_line(in_str)
    {
        Ok(_) => {
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
            exit(1);
        }
    }
}
