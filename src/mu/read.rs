/* mu/read.rs */
use std::io::{self, BufRead};

use crate::mu::r#type::Type;
use crate::mu::r#type::NIL;

use crate::mu::char::{Char};
use crate::mu::fixnum::{Fixnum};
use crate::mu::string::{String};
use crate::mu::symbol::{Symbol};

use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_while, take, take_until},
    character::{is_space, is_alphanumeric},
    combinator::{map_res, opt},
    many0,
    sequence::{tuple}
};

// numbers
fn parse_hexadecimal(input: &str) -> IResult<&str, Type> {
    let (input, _) = tag("#x")(input)?;
    let (input, hex) =
         || -> IResult<&str, i64> {
             map_res(
                 take_while(|c: char| c.is_digit(16)),
                 |input: &str| i64::from_str_radix(input, 16)
             )
                 (input)
         }()?;

    Ok((input, Fixnum::make_type(hex)))
}

fn parse_decimal(input: &str) -> IResult<&str, Type> {
    let (input, dec) =
         || -> IResult<&str, i64> {
            map_res(
                take_while(|c: char| c.is_digit(10)),
                |input: &str| i64::from_str_radix(input, 10)
            )(input)
         }()?;

    Ok((input, Fixnum::make_type(dec)))
}

// string/char
fn parse_string(input: &str) -> IResult<&str, Type> {
    let (input, _) = tag("\"")(input)?;
    let (input, str) = take_until("\"")(input)?;

    Ok((input, String::make_type(str)))
}

fn parse_char(input: &str) -> IResult<&str, Type> {
    let (input, _) = tag("#\\")(input)?;
    let (input, ch) = take(1 as usize)(input)?;

    Ok((input, Char::make_type(ch.chars().nth(0).unwrap())))
}

// special forms
fn parse_quote(input: &str) -> IResult<&str, Type> {
    let (input, _) = tag("'")(input)?;
    let (input, form) =
        alt((parse_char,
             parse_decimal,
             parse_hexadecimal,
             parse_quote,
             parse_string))(input)?;
    
    Ok((input, NIL))
}

fn parse_form(input: &str) -> IResult<&str, Type> {
    let (input, _) = take_while(|ch: char| ch.is_ascii_whitespace())(input)?;
    
    alt((parse_char,
         parse_decimal,
         parse_hexadecimal,
         parse_quote,
         parse_string))(input)
}

pub fn _read() -> Type {
    let input = io::stdin().lock().lines().next().unwrap().unwrap();

    match parse_form(&input) {
        Ok((_, t)) => t,
        Err(err) => {
            println!("unparsed {:?}", err);
            NIL
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_hex() {
        assert!(
            match parse_hexadecimal("#x2F14DF") {
                Ok(("", fx)) =>
                   match fx.i64_from_fixnum() {
                       Some(ival) => ival == 0x2f14df,
                       _ => false,
                   },
                _ => false,
            })
    }

    fn test_dec() {
        assert!(
            match parse_decimal("123456") {
                Ok(("", fx)) =>
                   match fx.i64_from_fixnum() {
                       Some(ival) => ival == 123456,
                       _ => false,
                   },
                _ => false,
            })
    }

    #[test]
    fn test_string() {
        assert!(
            match parse_string("\"abc123\"") {
                Ok(("", str)) => str.typep_string(),
                _ => false,
            })
    }

    #[test]
    fn test_char() {
        assert!(
            match parse_char("#\\a") {
                Ok(("", ch)) => ch.typep_char(),
                _ => false,
            })
    }

    /*
    #[test]
    fn test_symbol() {
        assert!(match symbol_(b"abc123 ") {
            Ok((_, str)) => {
                let _sy = symbol(string(str), NIL);
                true
            }
            Err(_) => false,
        })
    }

    #[test]
    fn test_keyword() {
        assert!(match keyword_(b":abc123 ") {
            Ok((_, (_, str))) => {
                let _kw = keyword(string(str));
                true
            }
            Err(_) => false,
        })
    }

    #[test]
    fn test_string() {
        assert!(match string_(b"\"abc123\" ") {
            Ok((_, (_, str, _))) => {
                let _st = string(str);
                true
            }
            Err(_) => false,
        })
    }

    #[test]
    fn test_char() {
        assert!(match char_(b"#\\a ") {
            Ok((_, (_, _ch))) => true,
            Err(_) => false,
        })
    }

    #[test]
    fn test_dotted() {
        assert!(match dotted_(b"( 123 . 456 ) ") {
            Ok((_, (_, _car, _, _, _, _cdr, _, _))) => _car.typep_fixnum(),
            Err(_) => false,
        })
    }

    #[test]
    fn test_list() {
        assert!(match list_(b"( 1234 5678 ) ") {
            Ok((_, (_, _vec, _, _))) => true,
            Err(_) => false,
        })
    }

    #[test]
    fn test_nil() {
        assert!(match nil_(b"( ) ") {
            Ok((_, (_, _, _))) => true,
            Err(_) => false,
        })
    }
     */
}

