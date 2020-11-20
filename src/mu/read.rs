// mu/read.rs
use std::io::{self, BufRead};

use crate::mu::r#type::Type;
use crate::mu::r#type::NIL;

use crate::mu::char::FnChar;
use crate::mu::fixnum::FnFixnum;
use crate::mu::string::FnString;
use crate::mu::symbol::FnSymbol;

use crate::mu::print::debug_println;

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until, take_while, take_while1},
    combinator::map_res,
    multi::many0,
    sequence::tuple,
    IResult,
};

/*
#          non-terminating macro char
\          single escape
|          multiple escape
 */

// terminating macro
/*
"          terminating macro char
'          terminating macro char
(          terminating macro char
)          terminating macro char
,          terminating macro char
;          terminating macro char
`          terminating macro char
 */
// constituent
fn is_constituent(ch: char) -> bool {
    const CONSTITUENT: &str = "0123456789\
                               !$%&*+-./:<=>?@[]^_{}~\
                               ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                               abcdefghijklmnopqrstuvwxyz";

    // println!("{} {}", ch, CONSTITUENT.contains(ch));
    CONSTITUENT.contains(ch)
}

// whitespace

/*
Linefeed   whitespace[2]
Newline    whitespace[2]
Page       whitespace[2]
Return     whitespace[2]
Space      whitespace[2]
Tab        whitespace[2]
*/

// numbers
fn read_hexadecimal(input: &str) -> IResult<&str, Type> {
    let (input, _) = tag("#x")(input)?;
    let (input, hex) = || -> IResult<&str, i64> {
        map_res(take_while(|c: char| c.is_digit(16)), |input: &str| {
            i64::from_str_radix(input, 16)
        })(input)
    }()?;

    Ok((input, FnFixnum::make_type(hex)))
}

fn read_decimal(input: &str) -> IResult<&str, Type> {
    let (input, dec) = || -> IResult<&str, i64> {
        map_res(take_while(|c: char| c.is_digit(10)), |input: &str| {
            i64::from_str_radix(input, 10)
        })(input)
    }()?;

    // println!("dec: {}", neg);
    Ok((input, FnFixnum::make_type(dec)))
}

fn read_neg_decimal(input: &str) -> IResult<&str, Type> {
    let (input, (_, dec)) = tuple((tag("-"), read_decimal))(input)?;

    let d = -dec.i64_from_fixnum().unwrap();
    
    Ok((input, FnFixnum::make_type(d)))
}

fn read_pos_decimal(input: &str) -> IResult<&str, Type> {
    let (input, (_, dec)) = tuple((tag("+"), read_decimal))(input)?;

    Ok((input, dec))
}

fn read_number(input: &str) -> IResult<&str, Type> {
    alt((
        read_hexadecimal,
        read_decimal,
        read_neg_decimal,
        read_pos_decimal,
    ))(input)
}

// string/char
fn read_string(input: &str) -> IResult<&str, Type> {
    let (input, (_, str, _)) = tuple((tag("\""), take_until("\""), take(1 as usize)))(input)?;

    Ok((input, FnString::make_type(str)))
}

fn read_char(input: &str) -> IResult<&str, Type> {
    let (input, (_, ch)) = tuple((tag("#\\"), take(1 as usize)))(input)?;

    Ok((input, FnChar::make_type(ch.chars().nth(0).unwrap())))
}

// special forms
fn read_quote(input: &str) -> IResult<&str, Type> {
    let (input, _) = tag("'")(input)?;

    let (input, form) = alt((
        read_char,
        read_cons,
        read_list,
        read_number,
        read_quote,
        read_string,
        read_symbol,
        read_vector,
    ))(input)?;

    Ok((
        input,
        FnSymbol::make_keyword(FnString::make_type("quote")).cons(form),
    ))
}

// lists/vectors/dotted pair
fn vec_to_list(list: Type, i: usize, v: &Vec<Type>) -> Type {
    if i == v.len() {
        list
    } else {
        vec_to_list(v[i].cons(list), i + 1, v)
    }
}

fn read_list(input: &str) -> IResult<&str, Type> {
    let (input, (_, v, _, _)) = tuple((
        tag("("),
        many0(read_form),
        take_while(|ch: char| ch.is_ascii_whitespace()),
        tag(")"),
    ))(input)?;

    Ok((input, vec_to_list(NIL, 0, &v)))
}

fn read_vector(input: &str) -> IResult<&str, Type> {
    let (input, (_, _n, _, v, _)) =
        tuple((tag("#"), read_decimal, tag("("), many0(read_form), tag(")")))(input)?;

    Ok((input, vec_to_list(NIL, 0, &v)))
}

fn read_cons(input: &str) -> IResult<&str, Type> {
    let (input, (_, car, _, _, _, cdr, _, _)) = tuple((
        tag("("),
        read_form,
        take_while1(|ch: char| ch.is_ascii_whitespace()),
        tag("."),
        take_while1(|ch: char| ch.is_ascii_whitespace()),
        read_form,
        take_while(|ch: char| ch.is_ascii_whitespace()),
        tag(")"),
    ))(input)?;

    Ok((input, car.cons(cdr)))
}

// symbols
fn read_symbol(input: &str) -> IResult<&str, Type> {
    let (input, str) = take_while1(|ch: char| is_constituent(ch))(input)?;
    let ch = str.chars().nth(0).unwrap();

    if ch == ':' {
        Ok((
            input,
            FnSymbol::make_keyword(FnString::make_type(&str[1..])),
        ))
    } else {
        let sym = FnSymbol::make_type(FnString::make_type(str), NIL);
        debug_println(sym);

        Ok((input, sym))
    }
}

// reader
fn read_form(input: &str) -> IResult<&str, Type> {
    let (input, _) = take_while(|ch: char| ch.is_ascii_whitespace())(input)?;

    alt((
        read_char,
        read_number,
        read_cons,
        read_list,
        read_quote,
        read_string,
        read_vector,
        read_symbol,
    ))(input)
}

pub fn read_from_stdin(_stream: Type) -> Type {
    let input = io::stdin().lock().lines().next().unwrap().unwrap();

    match read_form(&input) {
        Ok((_, t)) => t,
        Err(err) => {
            println!("unread {:?}", err);
            NIL
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex() {
        assert!(match read_hexadecimal("#x2F14DF") {
            Ok(("", fx)) => match fx.i64_from_fixnum() {
                Some(ival) => ival == 0x2f14df,
                _ => false,
            },
            _ => false,
        })
    }

    #[test]
    fn test_dec() {
        assert!(match read_decimal("123456") {
            Ok(("", fx)) => match fx.i64_from_fixnum() {
                Some(ival) => ival == 123456,
                _ => false,
            },
            _ => false,
        })
    }

    #[test]
    fn test_string() {
        assert!(match read_string("\"abc123\"") {
            Ok(("", str)) => str.typep_string(),
            _ => false,
        })
    }

    #[test]
    fn test_char() {
        assert!(match read_char("#\\a") {
            Ok(("", ch)) => ch.typep_char(),
            _ => false,
        })
    }

    #[test]
    fn test_symbol() {
        assert!(match read_symbol("abc123") {
            Ok(("", sym)) => sym.typep_symbol(),
            _ => false,
        })
    }

    #[test]
    fn test_keyword() {
        assert!(match read_symbol(":abc123") {
            Ok(("", kwd)) => kwd.typep_keyword(),
            _ => false,
        })
    }

    /*
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
