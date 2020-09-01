/* mu/read.rs */
use std::io::{self, BufRead};

use crate::mu::r#type::Type;
use crate::mu::r#type::NIL;

use crate::mu::fixnum::_fixnum;

use nom::IResult;
use nom::number::complete::be_u32;
use nom::number::complete::be_i64;
use nom::bytes::complete::take;
use nom::bytes::complete::take_while;

pub fn length_value(input: &[u8]) -> IResult<&[u8],&[u8]> {
    let (input, length) = be_u32(input)?;
    println!("{:x?},{:x?}", input, length);
    take(length)(input)
}

// pub fn _read(_src: Type) -> Type {
pub fn _read() -> Type {
    let input = io::stdin().lock().lines().next().unwrap().unwrap();

    match length_value(input.as_bytes()) {
        Ok(_) => println!("gotcha"),
        Err(whoops) => println!("{}", whoops)
    }
    NIL
}

#[cfg(test)]
mod tests {
    // use super::*;

    /*
    #[test]
    fn test_type() {
        assert!(fixnum(0).type_fixnum());
    }

    #[test]
    fn test_u64() {
        assert!(fixnum(0).u64_from_fixnum() == 0);
        assert!(fixnum(1).u64_from_fixnum() == 1);
    }

    #[test]
    fn test_eq() {
        assert!(fixnum(0).eq(fixnum(0)));
    }
    
    #[test]
    fn test_add() {
        assert!(fixnum(1).fixnum_add(fixnum(2)).eq(fixnum(3)));
    }
     */
}
