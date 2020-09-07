/* mu/read.rs */
use std::io::{self, BufRead};

// use hex::FromHex;

// use std::str;
use std::str::FromStr;
use std::str::from_utf8;

use crate::mu::r#type::_immediate;
use crate::mu::r#type::ImmediateClass;
use crate::mu::r#type::Type;
use crate::mu::r#type::NIL;
use crate::mu::r#type::_T;

use crate::mu::fixnum::_fixnum;
use crate::mu::string::_string;
use crate::mu::symbol::_symbol;

use nom::{alt, complete, eof, map_res, named, opt};
use nom::{tag, take,take_until, take_while, tuple};

use nom::bytes::complete::{take, take_while};
use nom::character::{is_alphabetic, is_alphanumeric, is_digit, is_space};

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

named!(char_<&[u8], (Option<&[u8]>, &[u8], &[u8])>,
       tuple!(
           opt!(take_while!(is_space)),
           tag!("#\\"),
           take!(1)
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

named!(cons_<&[u8], (Option<&[u8]>, &[u8], Type, Option<&[u8]>, &[u8])>,
       tuple!(
           opt!(take_while!(is_space)),
           tag!("("),
           read_,
           opt!(take_while!(is_space)),
           tag!(")")
       )
);

named!(nil_<&[u8], (Option<&[u8]>, &[u8], Option<&[u8]>, &[u8])>,
       tuple!(
           opt!(take_while!(is_space)),
           tag!("("),
           opt!(take_while!(is_space)),
           tag!(")")
       )
);

named!(read_<Type>, alt!(

    char_ => { |cs: (Option<&[u8]>, &[u8], &[u8])| {
        println!("char");
        _immediate(cs.2[0] as u64,
                   1,
                   ImmediateClass::Char)
    }
    } |


    /*
    symbol_ => { |ss: (Option<&[u8]>, &[u8])|
                  match from_utf8(ss.1) {
                      Ok(str) =>
                      {
                          let sym = _symbol(_string(str.as_bytes()), NIL);
                          println!("read symbol: {:?}", sym);
                          sym
                      },
                      Err(_) => NIL
                  }
    } |
     */
    
    /*
    string_ => { |ss: (Option<&[u8]>, &[u8], &[u8], &[u8])|
                  match from_utf8(ss.2) {
                      Ok(str) => _string(str.as_bytes()),
                      Err(_) => NIL
                  }
    } |
     */

    nil_ => { |_fs: (Option<&[u8]>, &[u8], Option<&[u8]>, &[u8])| {
        println!("nil");
        NIL
    }
    } |

    cons_ => { |cs: (Option<&[u8]>, &[u8], Type, Option<&[u8]>, &[u8])| {
        println!("cons");
        cs.2.cons(NIL)
    }
    } |

    fixnum_ => { |fs: (Option<&[u8]>, &[u8])| {
        println!("fixnum");
        match from_utf8(fs.1) {
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
            Err(_) => NIL
        }
    }
    }

));

// pub fn _read(_src: Type) -> Type {
pub fn _read() -> Type {
    let input = io::stdin().lock().lines().next().unwrap().unwrap();

    match read_(input.as_bytes()) {
        Ok((_, _type)) => _type,
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
    fn test_symbol() {
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
    fn test_string() {
        assert!(
            match string_(b"\"abc123\" ") {
                Ok((_, (_, _, str, _))) =>
                    {
                        let _st = _string(str);
                        true
                    },
                Err(_) => false
            })}

    #[test]
    fn test_char() {
        assert!(
            match char_(b"#\\a ") {
                Ok((_, (_, _, _ch))) => true,
                Err(_) => false
            })}
 
    #[test]
    fn test_cons() {
        assert!(
            match cons_(b" ( 123 ) ") {
                Ok((_, (_, _, type_, _, _))) => type_.type_fixnum(),
                Err(_) => false
            })}

    #[test]
    fn test_nil() {
        assert!(
            match nil_(b" ( ) ") {
                Ok((_, (_, _, _, _))) => true,
                Err(_) => false
            })}
}
