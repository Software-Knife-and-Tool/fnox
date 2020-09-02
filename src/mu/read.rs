/* mu/read.rs */
use std::io::{self, BufRead};

use std::str;
use std::str::FromStr;
use std::str::from_utf8;

use crate::mu::r#type::Type;
use crate::mu::r#type::NIL;

use crate::mu::fixnum::_fixnum;

use nom::IResult;
use nom::bytes::complete::take;
use nom::bytes::complete::take_while;
use nom::character::is_alphabetic;
use nom::character::is_alphanumeric;
use nom::character::is_digit;

use nom::character::*;

/*
// We parse any expr surrounded by parens, ignoring all whitespaces around those
named!(parens<i64>, ws!(delimited!( tag!("("), expr, tag!(")") )) );

// We transform an integer string into a i64, ignoring surrounding whitespaces
// We look for a digit suite, and try to convert it.
// If either str::from_utf8 or FromStr::from_str fail,
// we fallback to the parens parser defined above
named!(factor<i64>, alt!(
    map_res!(
      map_res!(
        ws!(digit),
        str::from_utf8
      ),
      FromStr::from_str
    )
  | parens
  )
);

// We read an initial factor and for each time we find
// a * or / operator followed by another factor, we do
// the math by folding everything
named!(term <i64>, do_parse!(
    init: factor >>
    res:  fold_many0!(
        pair!(alt!(tag!("*") | tag!("/")), factor),
        init,
        |acc, (op, val): (&[u8], i64)| {
            if (op[0] as char) == '*' { acc * val } else { acc / val }
        }
    ) >>
    (res)
  )
);
*/
fn fixnum(input: &[u8]) -> IResult<&[u8],&[u8]> {
    take_while(is_digit)(input)
}

// pub fn _read(_src: Type) -> Type {
pub fn _read() -> Type {
    let input = io::stdin().lock().lines().next().unwrap().unwrap();

    match fixnum(input.as_bytes()) {
        Ok((rest, token)) =>
            {
                println!("{:x?},{:x?}", rest, token);
                match from_utf8(&token) {
                    Ok(str) =>
                        match i64::from_str(&str) {
                            Ok(fix) => _fixnum(fix),
                            Err(_) => NIL
                        },
                    Err(whoops) => 
                        {
                            println!("{:x?}", whoops);
                            NIL
                        }
                }
            },
        Err(whoops) =>
            {
                println!("{:x?}", whoops);
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
