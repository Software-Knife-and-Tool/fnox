/* mu/read.rs */
use std::io::{self, BufRead};
use std::str::{from_utf8, FromStr};

use crate::mu::r#type::Type;
use crate::mu::r#type::NIL;
use crate::mu::r#type::{immediate, ImmediateClass};

use crate::mu::fixnum::fixnum;
use crate::mu::string::_string;
use crate::mu::symbol::keyword;
use crate::mu::symbol::symbol;

use nom::{alt, many1, named, opt, eof};
use nom::{tag, take, take_until, take_while, take_while1, tuple};

use nom::character::{is_alphanumeric, is_digit, is_space};

named!(fixnum_<&[u8], &[u8]>,
       take_while1!(is_digit)
);

named!(symbol_<&[u8], &[u8]>,
       take_while1!(is_alphanumeric)
);

named!(keyword_<&[u8], (&[u8], &[u8])>,
       tuple!(
           tag!(":"),
           take_while1!(is_alphanumeric)
       )
);

named!(char_<&[u8], (&[u8], &[u8])>,
       tuple!(
           tag!("#\\"),
           take!(1)
       )
);

named!(string_<&[u8], (&[u8], &[u8], &[u8])>,
       tuple!(
           tag!("\""),
           take_until!("\""),
           tag!("\"")
       )
);

named!(cons_<&[u8], (&[u8], Type, Option<&[u8]>, &[u8])>,
       tuple!(
           tag!("("),
           type_,
           opt!(take_while!(is_space)),
           tag!(")")
       )
);

named!(dotted_<&[u8], (&[u8], Type, Option<&[u8]>, &[u8], Option<&[u8]>, Type, Option<&[u8]>, &[u8])>,
       tuple!(
           tag!("("),
           type_,
           opt!(take_while!(is_space)),
           tag!("."),
           opt!(take_while!(is_space)),
           type_,
           opt!(take_while!(is_space)),
           tag!(")")
       )
);

named!(list_<&[u8], (&[u8], Vec<Type>, Option<&[u8]>, &[u8])>,
       tuple!(
           tag!("("),
           many1!(type_),
           opt!(take_while!(is_space)),
           tag!(")")
       )
);

named!(nil_<&[u8], (&[u8], Option<&[u8]>, &[u8])>,
       tuple!(
           tag!("("),
           opt!(take_while!(is_space)),
           tag!(")")
       )
);

named!(
    type_<Type>,
    alt!(
        char_ => { |cs: (&[u8], &[u8])|
                    immediate(cs.1[0] as u64,
                              1,
                              ImmediateClass::Char)
        } |

        /* distinguish fixnums from symbols */
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

        keyword_ => { |ks: (&[u8], &[u8]) |
                       keyword(_string(ks.1))
        } |

        symbol_ => { |ss: &[u8]|
                      symbol(_string(ss), NIL)
        } |

        string_ => { |ss: (&[u8], &[u8], &[u8])|
                      _string(ss.1)
        } |

        nil_ => { |_fs: (&[u8], Option<&[u8]>, &[u8])|
                    NIL
        } |

        cons_ => { |cs: (&[u8], Type, Option<&[u8]>, &[u8])|
                    cs.1.cons(NIL)
        } |

        dotted_ => { |ds: (&[u8], Type, Option<&[u8]>, &[u8], Option<&[u8]>, Type, Option<&[u8]>, &[u8])|
                       ds.1.cons(ds.5)
        } |

        list_ => { |_ls: (&[u8], Vec<Type>, Option<&[u8]>, &[u8])|
                    NIL
        }
    )
);

named!(read_form<&[u8], (Option<&[u8]>, Type, Option<&[u8]>)>,
       tuple!(
           opt!(take_while!(is_space)),
           type_,
           opt!(eof!())
       )
);

pub fn _read() -> Type {
    let input = io::stdin().lock().lines().next().unwrap().unwrap();
    let instr = input.as_bytes();

    match read_form(instr) {
        Ok((_, (_, type_, _))) => type_,
        Err(err) => {
            println!("undecoded {:?}", err);
            NIL
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fx() {
        assert!(match fixnum_(b"123 ") {
            Ok((_, fx)) => match from_utf8(fx) {
                Ok(str) => match i64::from_str(&str) {
                    Ok(fix) => fix == 123,
                    Err(_) => false,
                },
                Err(_) => false,
            },
            Err(_) => false,
        })
    }

    #[test]
    fn test_fx1() {
        assert!(match fixnum_(b"123 ") {
            Ok((_, fx)) => match from_utf8(fx) {
                Ok(str) => match i64::from_str(&str) {
                    Ok(fix) => {
                        let _fx = fixnum(fix);
                        fix == 123
                    }
                    Err(_) => false,
                },
                Err(_) => false,
            },
            Err(_) => false,
        })
    }

    #[test]
    fn test_symbol() {
        assert!(match symbol_(b"abc123 ") {
            Ok((_, str)) => {
                let _sy = symbol(_string(str), NIL);
                true
            }
            Err(_) => false,
        })
    }

    #[test]
    fn test_keyword() {
        assert!(match keyword_(b":abc123 ") {
            Ok((_, (_, str))) => {
                let _kw = keyword(_string(str));
                true
            }
            Err(_) => false,
        })
    }

    #[test]
    fn test_string() {
        assert!(match string_(b"\"abc123\" ") {
            Ok((_, (_, str, _))) => {
                let _st = _string(str);
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
    fn test_cons() {
        assert!(match cons_(b"( 1234 ) ") {
            Ok((_, (_, type_, _, _))) => type_.typep_fixnum(),
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
}
