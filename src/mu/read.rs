/* mu/read.rs */
use std::io::{self, BufRead};
use std::str::FromStr;

use crate::mu::r#type::Type;
use crate::mu::r#type::NIL;

use crate::mu::fixnum::_fixnum;

use nom::IResult;
use nom::number::complete::be_u32;
use nom::number::complete::be_i64;
use nom::bytes::complete::take;
use nom::bytes::complete::take_while;
use nom::character::is_alphabetic;
use nom::character::is_alphanumeric;
use nom::character::is_digit;

use nom::character::*;

fn fixnum(input: &[u8]) -> IResult<&[u8],&[u8]> {
    let (input, _) = be_u32(input)?;

    take_while(is_alphanumeric)(input)
}

// pub fn _read(_src: Type) -> Type {
pub fn _read() -> Type {
    let input = io::stdin().lock().lines().next().unwrap().unwrap();

    match fixnum(input.as_bytes()) {
        Ok((_,_)) =>
            {
                // println!("{:?},{:?}", i, j);
                _fixnum(i64::from_str(&input).unwrap())
            },
        Err(whoops) =>
            {
                println!("{}", whoops);
                NIL
            }
    }
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
