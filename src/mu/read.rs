/* mu/read.rs */
use std::io::{self, BufRead};
use std::str::{from_utf8, FromStr};

use crate::mu::r#type::Type;
use crate::mu::r#type::NIL;

use crate::mu::char::{Char};
use crate::mu::fixnum::{Fixnum};
use crate::mu::string::{String};
use crate::mu::symbol::{Symbol};

use nom::{
    IResult,
    take_while,
    bytes::complete::{tag, take_while, take, take_until},
    combinator::map_res,
    sequence::tuple};

// numbers
fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn is_dec_digit(c: char) -> bool {
    c.is_digit(10)
}

fn from_hex64(input: &str) -> Result<i64, std::num::ParseIntError> {
    i64::from_str_radix(input, 16)
}

fn from_dec64(input: &str) -> Result<i64, std::num::ParseIntError> {
    i64::from_str_radix(input, 10)
}

fn hex_digits(input: &str) -> IResult<&str, i64> {
    map_res(
        take_while(is_hex_digit),
        from_hex64
    )(input)
}

fn dec_digits(input: &str) -> IResult<&str, i64> {
    map_res(
        take_while(is_dec_digit),
        from_dec64
    )(input)
}

fn hexadecimal_(input: &str) -> IResult<&str, Type> {
    let (input, _) = tag("#x")(input)?;
    let (input, hex) = hex_digits(input)?;

    Ok((input, Fixnum::make_type(hex)))
}

fn decimal_(input: &str) -> IResult<&str, Type> {
    let (input, dec) = dec_digits(input)?;

    Ok((input, Fixnum::make_type(dec)))
}

// string/char
fn string_(input: &str) -> IResult<&str, Type> {
    let (input, _) = tag("\"")(input)?;
    let (input, str) = take_until("\"")(input)?;

    Ok((input, String::make_type(str)))
}

fn char_(input: &str) -> IResult<&str, Type> {
    let (input, _) = tag("#\\")(input)?;
    let (input, ch) = take(1 as usize)(input)?;

    Ok((input, Char::make_type(ch.chars().nth(0).unwrap())))
}

// list; nil as a special case


/*
named!(
    atom<Type>,
    alt!(
        fixnum_ => { |fs: &[u8] |
                      match from_utf8(fs) {
                          Ok(str) =>
                              match i64::from_str(&str) {
                                  Ok(fix) => fixnum(fix),
                                  Err(_) => NIL
                              },
                          Err(_) => NIL
                      }
        } |

        char_ => { |cs: (_, &[u8])|
                    immediate(cs.1[0] as u64,
                              1,
                              ImmediateClass::Char)
        } |

        keyword_ => { |ks: (_, &[u8])| keyword(string(ks.1)) } |

        symbol_ => { |ss: &[u8]| symbol(string(ss), NIL) } |

        string_ => { |ss: (_, &[u8], _)| string(ss.1) } |

        nil_ => { |_fs: (_, _, _)| NIL }
    )
);

named!(read_form<&[u8], Type>, ws!(atom));

*/


pub fn _read() -> Type {
    let _input = io::stdin().lock().lines().next().unwrap().unwrap();

    /*
    match read_form(input.as_bytes()) {
        Ok((_, t)) => t,
        Err(err) => {
            println!("unparsed {:?}", err);
            NIL
        }
    }
     */
    NIL
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_hex() {
        assert!(
            match hexadecimal_("#x2F14DF") {
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
            match decimal_("123456") {
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
            match string_("\"abc123\"") {
                Ok(("", str)) => str.typep_string(),
                _ => false,
            })
    }

    #[test]
    fn test_char() {
        assert!(
            match char_("#\\a") {
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

