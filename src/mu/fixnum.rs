// mu/fixnum.rs
use crate::mu::r#type::{entag, Tag, Type};
use crate::mu::r#type::{NIL, T};

#[derive(Debug)]
pub struct FnFixnum {
    integer: i64,
}

pub fn fixnum_add(args: Vec<Type>) -> Type {
    match FnFixnum::from_type(&args[0]) {
        Some(fx) => match fx.add(&args[1]) {
            Some(s) => s,
            None => NIL,
        },
        None => NIL,
    }
}

impl FnFixnum {
    pub fn make_type(src: i64) -> Type {
        entag((src as u64) << 3, Tag::Fixnum)
    }

    // think: do we need this?
    pub fn _from_i64(_integer: i64) -> FnFixnum {
        FnFixnum {
            integer: _integer as i64,
        }
    }

    pub fn from_type(fx: &Type) -> Option<FnFixnum> {
        if Type::typep_fixnum(fx) {
            Some(FnFixnum {
                integer: (fx.as_u64() >> 3) as i64,
            })
        } else {
            assert!(false);
            None
        }
    }

    pub fn add(&self, fx: &Type) -> Option<Type> {
        if Type::typep_fixnum(fx) {
            Some(FnFixnum::make_type(
                self.integer + (fx.as_u64() >> 3) as i64,
            ))
        } else {
            assert!(false);
            None
        }
    }

    pub fn mul(&self, fx: &Type) -> Option<Type> {
        if Type::typep_fixnum(fx) {
            Some(FnFixnum::make_type(
                self.integer * (fx.as_u64() >> 3) as i64,
            ))
        } else {
            assert!(false);
            None
        }
    }

    pub fn trunc(&self, fx: &Type) -> Option<Type> {
        if Type::typep_fixnum(fx) {
            Some(FnFixnum::make_type(
                self.integer / (fx.as_u64() >> 3) as i64,
            ))
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
            Some(FnFixnum::make_type(
                self.integer % (fx.as_u64() >> 3) as i64,
            ))
        } else {
            assert!(false);
            None
        }
    }

    pub fn logand(&self, fx: &Type) -> Option<Type> {
        if Type::typep_fixnum(fx) {
            Some(FnFixnum::make_type(
                self.integer & (fx.as_u64() >> 3) as i64,
            ))
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
        assert!(FnFixnum::make_type(0).typep_fixnum());
    }

    #[test]
    fn test_i64() {
        assert!(match FnFixnum::make_type(0).i64_from_fixnum() {
            None => false,
            Some(v) => v == 0,
        });

        assert!(match FnFixnum::make_type(1).i64_from_fixnum() {
            None => false,
            Some(v) => v == 1,
        });
    }

    #[test]
    fn test_eq() {
        assert!(!FnFixnum::make_type(0).eq(FnFixnum::make_type(1)));
    }

    #[test]
    fn test_minusp() {
        assert!(match FnFixnum::from_type(&FnFixnum::make_type(-1)) {
            Some(fx) => fx.minusp().eq(T),
            None => false,
        });
    }

    #[test]
    fn test_add() {
        assert!(match FnFixnum::from_type(&FnFixnum::make_type(1)) {
            Some(fx) => match fx.add(&FnFixnum::make_type(2)) {
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
        assert!(match FnFixnum::from_type(&FnFixnum::make_type(2)) {
            Some(fx) => match fx.mul(&FnFixnum::make_type(3)) {
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
        assert!(match FnFixnum::from_type(&FnFixnum::make_type(3)) {
            Some(fx) => match fx.trunc(&FnFixnum::make_type(2)) {
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
        assert!(match FnFixnum::from_type(&FnFixnum::make_type(1)) {
            Some(fx) => match fx.logand(&FnFixnum::make_type(2)) {
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
        assert!(match FnFixnum::from_type(&FnFixnum::make_type(5)) {
            Some(fx) => match fx.mod_(&FnFixnum::make_type(3)) {
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
