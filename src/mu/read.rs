/* mu/read.rs */
use std::io::{self, BufRead};

// use hex::FromHex;

// use std::str;
use std::str::FromStr;
use std::str::from_utf8;

use crate::mu::r#type::Type;
use crate::mu::r#type::NIL;

use crate::mu::fixnum::_fixnum;
use crate::mu::string::_string;
use crate::mu::symbol::_symbol;

// use nom::IResult;

use nom::alt;
use nom::complete;
use nom::eof;
use nom::map_res;
use nom::named;
use nom::opt;
use nom::tag;
use nom::take;
use nom::take_until;
use nom::take_while;
use nom::tuple;

use nom::bytes::complete::take;
use nom::bytes::complete::take_while;

use nom::character::is_alphabetic;
use nom::character::is_alphanumeric;
use nom::character::is_digit;
use nom::character::is_space;

use nom::character::*;

/*
named!(tpl<&[u8], (Option<&[u8]>, &[u8], &[u8])>,
  tuple!(
      opt!(take_while!(is_space)),
      take!(3),
      tag!("fg")
  )
);
*/

named!(fixnum_<&[u8], (Option<&[u8]>, &[u8])>,
  tuple!(
      opt!(take_while!(is_space)),
      take_while!(is_digit)
  )
);

named!(symbol_<&[u8], (Option<&[u8]>, &[u8])>,
  tuple!(
      opt!(take_while!(is_space)),
      take_while!(is_alphanumeric)
  )
);

named!(string_<&[u8], (Option<&[u8]>, &[u8], &[u8], &[u8])>,
  tuple!(
      opt!(take_while!(is_space)),
      tag!("\""),
      take_until!("\""),
      tag!("\"")
  )
);

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
    
    fixnum_ => { |fs: (Option<&[u8]>, &[u8])|
                  match from_utf8(fs.1) {
                      Ok(str) =>
                          match i64::from_str(&str) {
                              Ok(fix) =>
                              {
                                  let fx = _fixnum(fix);
                                  println!("read fixnum: {:?}", fx);
                                  fx
                              },
                              Err(_) =>
                              {
                                  NIL
                              }
                          },
                      Err(_) =>
                      {
                          NIL
                      }
                  }
    }
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
    use super::*;

    #[test]
    fn test_fx() {
        assert!(
            match fixnum_(b" 123 ") {
                Ok((_, (_, fx))) =>
                    match from_utf8(fx) {
                        Ok(str) =>
                            match i64::from_str(&str) {
                                Ok(fix) => fix == 123,
                                Err(_) => false
                            },
                        Err(_) => false
                    }
                Err(_) => false
            })}

    #[test]
    fn test_fx1() {
        assert!(
            match fixnum_(b"123 ") {
                Ok((_, (_, fx))) =>
                    match from_utf8(fx) {
                        Ok(str) =>
                            match i64::from_str(&str) {
                                Ok(fix) =>
                                {
                                    let _fx = _fixnum(fix);
                                    fix == 123
                                },
                                Err(_) => false
                            },
                        Err(_) => false
                    }
                Err(_) => false
            })}

    #[test]
    fn test_symbol_() {
        assert!(
            match symbol_(b" abc123 ") {
                Ok((_, (_, str))) =>
                    {
                        let _sy = _symbol(_string(str), NIL);
                        true
                    },
                Err(_) => false
            })}

    #[test]
    fn test_string_() {
        assert!(
            match string_(b"\"abc123\" ") {
                Ok((_, (_, _, str, _))) =>
                    {
                        let _st = _string(str);
                        true
                    },
                Err(_) => false
            })}
}
