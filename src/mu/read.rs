/* mu/read.rs */
use std::io::{self, BufRead};

use std::str::FromStr;
use std::str::from_utf8;

use crate::mu::r#type::_immediate;
use crate::mu::r#type::ImmediateClass;
use crate::mu::r#type::Type;
use crate::mu::r#type::NIL;
// use crate::mu::r#type::_T;

use crate::mu::fixnum::_fixnum;
use crate::mu::string::_string;
use crate::mu::symbol::_symbol;

use nom::{alt, many1, named, opt};
use nom::{tag, take, take_until, take_while, take_while1, tuple};

use nom::character::{is_alphanumeric, is_digit, is_space};

named!(fixnum_<&[u8], (Option<&[u8]>, &[u8])>,
       tuple!(
           opt!(take_while!(is_space)),
           take_while1!(is_digit)
       )
);

named!(symbol_<&[u8], (Option<&[u8]>, &[u8])>,
       tuple!(
           opt!(take_while!(is_space)),
           take_while1!(is_alphanumeric)
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

named!(dotted_<&[u8], (Option<&[u8]>, &[u8], Type, Option<&[u8]>, &[u8], Option<&[u8]>, Type, Option<&[u8]>, &[u8])>,
       tuple!(
           opt!(take_while!(is_space)),
           tag!("("),
           read_,
           opt!(take_while!(is_space)),
           tag!("."),
           opt!(take_while!(is_space)),
           read_,
           opt!(take_while!(is_space)),
           tag!(")")
       )
);

named!(list_<&[u8], (Option<&[u8]>, &[u8], Vec<Type>, Option<&[u8]>, &[u8])>,
       tuple!(
           opt!(take_while!(is_space)),
           tag!("("),
           many1!(read_),
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

    char_ => { |cs: (Option<&[u8]>, &[u8], &[u8])|
                _immediate(cs.2[0] as u64,
                           1,
                           ImmediateClass::Char)
    } |

    /* distinguish fixnums from symbols */
    fixnum_ => { |fs: (Option<&[u8]>, &[u8])|
                  match from_utf8(fs.1) {
                      Ok(str) =>
                          match i64::from_str(&str) {
                              Ok(fix) => _fixnum(fix),
                              Err(_) => NIL
                          },
                      Err(_) => NIL
                  }
    } |
    
    symbol_ => { |ss: (Option<&[u8]>, &[u8])|
                  _symbol(_string(ss.1), NIL)
    } |
    
    string_ => { |ss: (Option<&[u8]>, &[u8], &[u8], &[u8])| 
                  _string(ss.2)
    } |
    
    nil_ => { |_fs: (Option<&[u8]>, &[u8], Option<&[u8]>, &[u8])|
                NIL
    } |

    cons_ => { |cs: (Option<&[u8]>, &[u8], Type, Option<&[u8]>, &[u8])|
                cs.2.cons(NIL)
    } |

    dotted_ => { |ds: (Option<&[u8]>, &[u8], Type, Option<&[u8]>, &[u8], Option<&[u8]>, Type, Option<&[u8]>, &[u8])|
                   ds.2.cons(ds.6)
    } |
    
    list_ => { |_ls: (Option<&[u8]>, &[u8], Vec<Type>, Option<&[u8]>, &[u8])|
                NIL
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
            match cons_(b" ( 1234 ) ") {
                Ok((_, (_, _, type_, _, _))) => type_.type_fixnum(),
                Err(_) => false
            })}

    #[test]
    fn test_dotted() {
        assert!(
            match dotted_(b" ( 123 . 456 ) ") {
                Ok((_, (_, _, _car, _, _, _, _cdr, _, _))) => _car.type_fixnum(),
                Err(_) => false
            })}

    #[test]
    fn test_list() {
        assert!(
            match list_(b" ( 1234 5678 ) ") {
                Ok((_, (_, _, _vec, _, _))) => true,
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
