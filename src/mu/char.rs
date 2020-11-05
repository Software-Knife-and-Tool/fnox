/* mu/char.rs */
use std::char;
use std::mem;

use crate::mu::r#type::{ImmediateClass, _immediate};
use crate::mu::r#type::{Tag, Type};

#[derive(Debug)]
pub struct _Char {
    _char: char,
}

pub fn _char(src: char) -> Type {
    _immediate(src as u64, 0, ImmediateClass::Char)
}

impl _Char {
    pub fn _print(&self) {
        println!("{}", self._char);
    }

    pub fn _from_char(_char: char) -> _Char {
        _Char { _char: _char }
    }

    pub fn _from_type(ch: &Type) -> Option<_Char> {
        if Type::typep_char(ch) {
            Some(_Char {
                _char: (ch.immediate_data() as u32),
            })
        } else {
            None
        }
    }
}

impl Type {
    pub fn typep_char(&self) -> bool {
        match self.tag() {
            Tag::Immediate => match Type::immediate_class(self) {
                ImmediateClass::Char => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn u8_from_char(&self) -> Option<u8> {
        if Type::typep_char(self) {
            Some((self.immediate_data() & 0xff) as u8)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type() {
        assert!(_char(32).typep_char());
    }

    /*
    #[test]
    fn test_i64() {
        assert!(match _fixnum(0).i64_from_fixnum() {
            None => false,
            Some(v) => v == 0,
        });

        assert!(match _fixnum(1).i64_from_fixnum() {
            None => false,
            Some(v) => v == 1,
        });
    }

    #[test]
    fn test_eq() {
        assert!(_fixnum(0).eq(_fixnum(0)));
        assert!(!_fixnum(0).eq(_fixnum(1)));
    }

    #[test]
    fn test_minusp() {
        assert!(match _Fixnum::_from_type(&_fixnum(-1)) {
            Some(fx) => fx._minusp().eq(T),
            None => false,
        });
    }

    #[test]
    fn test_add() {
        assert!(match _Fixnum::_from_type(&_fixnum(1)) {
            Some(fx) => match fx._add(&_fixnum(2)) {
                Some(sum) => match sum.i64_from_fixnum() {
                    Some(v) => v == 3,
                    None => false,
                },
                None => false,
            },
            None => false,
        });
    }

    #[test]
    fn test_mul() {
        assert!(match _Fixnum::_from_type(&_fixnum(2)) {
            Some(fx) => match fx._mul(&_fixnum(3)) {
                Some(sum) => match sum.i64_from_fixnum() {
                    Some(v) => v == 6,
                    None => false,
                },
                None => false,
            },
            None => false,
        });
    }

    #[test]
    fn test_trunc() {
        assert!(match _Fixnum::_from_type(&_fixnum(3)) {
            Some(fx) => match fx._trunc(&_fixnum(2)) {
                Some(sum) => match sum.i64_from_fixnum() {
                    Some(v) => v == 1,
                    None => false,
                },
                None => false,
            },
            None => false,
        });
    }

    #[test]
    fn test_logand() {
        assert!(match _Fixnum::_from_type(&_fixnum(1)) {
            Some(fx) => match fx._logand(&_fixnum(2)) {
                Some(sum) => match sum.i64_from_fixnum() {
                    Some(v) => v == 0,
                    None => false,
                },
                None => false,
            },
            None => false,
        });
    }

    #[test]
    fn test_mod() {
        assert!(match _Fixnum::_from_type(&_fixnum(5)) {
            Some(fx) => match fx._mod(&_fixnum(3)) {
                Some(sum) => match sum.i64_from_fixnum() {
                    Some(v) => v == 2,
                    None => false,
                },
                None => false,
            },
            None => false,
        });
    }
     */
}
