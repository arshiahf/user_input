// Module allowing a string to be parsed into a generic vector

use std::str::FromStr;

pub(crate) fn parse_string_to_vec<T>(input:String, container:&mut Vec<T>) where T: FromStr
{
    let mut iter = input.split_whitespace();
    let mut spot = iter.next();
    while spot != None
    {
        match spot.unwrap().parse::<T>()
        {
            Ok(result) => container.push(result),
            Err(_) => {},
        }
        spot = iter.next();
    }
}
