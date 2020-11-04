/* mu/fixnum.rs */
use crate::mu::r#type::{entag, Tag, Type};
use crate::mu::r#type::{NIL, T};

#[derive(Debug)]
pub struct _Fixnum {
    integer: i64,
}

pub fn _fixnum(src: i64) -> Type {
    entag((src as u64) << 3, Tag::Fixnum)
}

pub fn _fixnum_add(args: Vec<Type>) -> Type {
    match _Fixnum::_from_type(&args[0]) {
        Some(fx) => match fx._add(&args[1]) {
            Some(s) => s,
            None => NIL,
        },
        None => NIL,
    }
}

impl _Fixnum {
    pub fn _print(&self) {
        println!("{}", self.integer);
    }

    pub fn _from_i64(_integer: i64) -> _Fixnum {
        _Fixnum {
            integer: _integer as i64,
        }
    }

    pub fn _from_type(fx: &Type) -> Option<_Fixnum> {
        if Type::typep_fixnum(fx) {
            Some(_Fixnum {
                integer: (fx.as_u64() >> 3) as i64,
            })
        } else {
            None
        }
    }

    pub fn _add(&self, fx: &Type) -> Option<Type> {
        if Type::typep_fixnum(fx) {
            Some(_fixnum(self.integer + (fx.as_u64() >> 3) as i64))
        } else {
            None
        }
    }

    pub fn _mul(&self, fx: &Type) -> Option<Type> {
        if Type::typep_fixnum(fx) {
            Some(_fixnum(self.integer * (fx.as_u64() >> 3) as i64))
        } else {
            None
        }
    }

    pub fn _trunc(&self, fx: &Type) -> Option<Type> {
        if Type::typep_fixnum(fx) {
            Some(_fixnum(self.integer / (fx.as_u64() >> 3) as i64))
        } else {
            None
        }
    }

    pub fn _minusp(&self) -> Type {
        if self.integer < 0 {
            T
        } else {
            NIL
        }
    }

    pub fn _mod(&self, fx: &Type) -> Option<Type> {
        if Type::typep_fixnum(fx) {
            Some(_fixnum(self.integer % (fx.as_u64() >> 3) as i64))
        } else {
            None
        }
    }

    pub fn _logand(&self, fx: &Type) -> Option<Type> {
        if Type::typep_fixnum(fx) {
            Some(_fixnum(self.integer & (fx.as_u64() >> 3) as i64))
        } else {
            None
        }
    }
}

impl Type {
    pub fn typep_fixnum(&self) -> bool {
        match self.tag() {
            Tag::Fixnum => true,
            _ => false,
        }
    }

    pub fn i64_from_fixnum(&self) -> Option<i64> {
        if Type::typep_fixnum(self) {
            // que?
            Some((self.as_u64() >> 3) as i64)
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
        assert!(_fixnum(0).typep_fixnum());
    }

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
}
