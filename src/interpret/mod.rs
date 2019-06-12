use std::str::FromStr;

// Parses string into vector, throwing away values not of type T
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

// Parses string into tuple
/*
pub(crate) fn parse_string_to_tuple<T>(input:String, container:&mut (), size:i32) where T: FromStr
{
    let mut iter = input.split_whitespace();
    let mut spot = iter.next();
    for index in 0..size
    {
        match spot.unwrap().parse::<T>()
        {
            Ok(result) => {container.index = result},
            Err(_) => {},
        }
        spot = iter.next();
    }
}
*/
