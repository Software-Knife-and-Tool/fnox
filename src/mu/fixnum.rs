/* mu/fixnum.rs */
use crate::mu::r#type::{entag, Tag, Type};
use crate::mu::r#type::{NIL, T};

#[derive(Debug)]
pub struct Fixnum {
    integer: i64,
}

pub fn fixnum(src: i64) -> Type {
    entag((src as u64) << 3, Tag::Fixnum)
}

pub fn fixnum_add(args: Vec<Type>) -> Type {
    match Fixnum::from_type(&args[0]) {
        Some(fx) => match fx.add(&args[1]) {
            Some(s) => s,
            None => NIL,
        },
        None => NIL,
    }
}

impl Fixnum {
    pub fn print(&self) {
        println!("{}", self.integer);
    }

    pub fn from_i64(_integer: i64) -> Fixnum {
        Fixnum {
            integer: _integer as i64,
        }
    }

    pub fn from_type(fx: &Type) -> Option<Fixnum> {
        if Type::typep_fixnum(fx) {
            Some(Fixnum {
                integer: (fx.as_u64() >> 3) as i64,
            })
        } else {
            assert!(false);
            None
        }
    }

    pub fn add(&self, fx: &Type) -> Option<Type> {
        if Type::typep_fixnum(fx) {
            Some(fixnum(self.integer + (fx.as_u64() >> 3) as i64))
        } else {
            assert!(false);
            None
        }
    }

    pub fn mul(&self, fx: &Type) -> Option<Type> {
        if Type::typep_fixnum(fx) {
            Some(fixnum(self.integer * (fx.as_u64() >> 3) as i64))
        } else {
            assert!(false);
            None
        }
    }

    pub fn trunc(&self, fx: &Type) -> Option<Type> {
        if Type::typep_fixnum(fx) {
            Some(fixnum(self.integer / (fx.as_u64() >> 3) as i64))
        } else {
            assert!(false);
            None
        }
    }

    pub fn minusp(&self) -> Type {
        if self.integer < 0 {
            T
        } else {
            NIL
        }
    }

    pub fn mod_(&self, fx: &Type) -> Option<Type> {
        if Type::typep_fixnum(fx) {
            Some(fixnum(self.integer % (fx.as_u64() >> 3) as i64))
        } else {
            assert!(false);
            None
        }
    }

    pub fn logand(&self, fx: &Type) -> Option<Type> {
        if Type::typep_fixnum(fx) {
            Some(fixnum(self.integer & (fx.as_u64() >> 3) as i64))
        } else {
            assert!(false);
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
            Some((self.as_u64() >> 3) as i64)
        } else {
            assert!(false);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type() {
        assert!(fixnum(0).typep_fixnum());
    }

    #[test]
    fn test_i64() {
        assert!(match fixnum(0).i64_from_fixnum() {
            None => false,
            Some(v) => v == 0,
        });

        assert!(match fixnum(1).i64_from_fixnum() {
            None => false,
            Some(v) => v == 1,
        });
    }

    #[test]
    fn test_eq() {
        assert!(fixnum(0).eq(fixnum(0)));
        assert!(!fixnum(0).eq(fixnum(1)));
    }

    #[test]
    fn test_minusp() {
        assert!(match Fixnum::from_type(&fixnum(-1)) {
            Some(fx) => fx.minusp().eq(T),
            None => false,
        });
    }

    #[test]
    fn test_add() {
        assert!(match Fixnum::from_type(&fixnum(1)) {
            Some(fx) => match fx.add(&fixnum(2)) {
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
        assert!(match Fixnum::from_type(&fixnum(2)) {
            Some(fx) => match fx.mul(&fixnum(3)) {
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
        assert!(match Fixnum::from_type(&fixnum(3)) {
            Some(fx) => match fx.trunc(&fixnum(2)) {
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
        assert!(match Fixnum::from_type(&fixnum(1)) {
            Some(fx) => match fx.logand(&fixnum(2)) {
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
        assert!(match Fixnum::from_type(&fixnum(5)) {
            Some(fx) => match fx.mod_(&fixnum(3)) {
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
