/*
fn parse_i32(input:String, container:&mut Vec<i32>)
{
    let mut iter = input.split_whitespace();
    let mut spot = iter.next();
    while spot != None
    {
        container.push(spot.unwrap().parse::<i32>().unwrap());
        spot = iter.next();
    }
}

fn parse_i64(input:String, container:&mut Vec<i64>)
{
    let mut iter = input.split_whitespace();
    let mut spot = iter.next();
    while iter.next() != None
    {
        container.push(spot.unwrap().parse::<i64>().unwrap());
        spot = iter.next();
    }
}

fn parse_f32()

fn parse_bool(input:String, container:&mut Vec<bool>)
{
    let mut iter = input.split_whitespace();
    let mut spot = iter.next();
    while iter.next() != None
    {
        container.push(spot.unwrap().parse::<bool>().unwrap());
        spot = iter.next();
    }
}
*/

use std::str::FromStr;

pub fn parse_string_to_vec<T>(input:String, container:&mut Vec<T>) where T: FromStr
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
