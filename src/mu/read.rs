/* mu/read.rs */
use std::io::{self, BufRead};

use hex::FromHex;

use std::str;
use std::str::FromStr;
use std::str::from_utf8;

use crate::mu::r#type::Type;
use crate::mu::r#type::NIL;

use crate::mu::fixnum::_fixnum;
use crate::mu::string::_string;
use crate::mu::symbol::_symbol;

use nom::IResult;

use nom::alt;
use nom::complete;
use nom::eof;
use nom::map_res;
use nom::named;
use nom::tag;
use nom::take_while;
use nom::ws;

use nom::bytes::complete::take;
use nom::bytes::complete::take_while;

use nom::character::is_alphabetic;
use nom::character::is_alphanumeric;
use nom::character::is_digit;

use nom::character::*;

named!(fixnum<&[u8], &[u8]>,
       alt!(complete!(take_while!(is_digit)) |
            complete!(ws!(take_while!(is_digit)))));

named!(symbol<&[u8], &[u8]>,
       alt!(complete!(take_while!(is_alphanumeric)) |
            complete!(ws!(take_while!(is_alphanumeric)))));

named!(string<&[u8], &[u8]>,
       alt!(complete!(take_while!(is_alphanumeric)) |
            complete!(ws!(take_while!(is_alphanumeric)))));

named!(types<Type>, alt!(
    /*
    symbol => { |ss: &[u8]|
                 match from_utf8(ss) {
                     Ok(str) =>
                     {
                         let sym = _symbol(_string(str), NIL);
                         println!("read symbol: {:?}", sym);
                         sym
                     },
                     Err(_) => NIL
                 }} |
    
    string => { |ss: &[u8]|
                 match from_utf8(ss) {
                     Ok(str) =>
                     {
                         let st = _string(str);
                         println!("read string: {:?}", st);
                         st
                     },
                     Err(_) => NIL
                 }} |
     */
    
    fixnum => { |fs: &[u8]|
                 match from_utf8(fs) {
                     Ok(str) =>
                         match i64::from_str(&str) {
                             Ok(fix) =>
                             {
                                 let fx = _fixnum(fix);
                                 println!("read fixnum: {:?}", fx);
                                 fx
                             },
                             Err(_) => NIL
                         },
                     Err(whoops) => 
                     {
                         println!("Err{:x?}", whoops);
                         NIL
                     }
                 }}
));

// pub fn _read(_src: Type) -> Type {
pub fn _read() -> Type {
    let input = io::stdin().lock().lines().next().unwrap().unwrap();

    match types(input.as_bytes()) {
        Ok((_, _type)) =>
        {
            _type
        }
        Err(_err) =>
        {
            println!("undecoded {}", _err);
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
