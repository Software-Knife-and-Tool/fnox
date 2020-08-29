/* mu/fixnum.rs */
use crate::mu::r#type::Tag;
use crate::mu::r#type::Type;
use crate::mu::r#type::NIL;
use crate::mu::r#type::entag;

struct _Fixnum {
    integer: i64
}

pub fn _fixnum(src: i64) -> Type {
    entag((src as u64) << 2, Tag::Efixnum)
}

pub fn _fixnum_add(args: Vec<Type>) -> Type {
    match _Fixnum::_from_type(&args[0]) {
        Some(fx) =>
            match fx._add(&args[1]) {
                Some(s) => s,
                None => NIL
            },
        None => NIL
    }
}

impl _Fixnum {
    pub fn _print(&self) {
        println!("{}", self.integer);
    }
    
    pub fn _from_i64(_integer: i64) -> _Fixnum {
        _Fixnum { integer: _integer as i64 }
    }

    pub fn _from_type(_type: &Type) -> Option<_Fixnum> {
        if Type::type_fixnum(_type) {
            Some(_Fixnum { integer: (_type.bits >> 2) as i64 })
        } else {
            None
        }
    }

    pub fn _add(&self, fx: &Type) -> Option<Type> {
        if Type::type_fixnum(fx) {
            Some(_fixnum(self.integer + (fx.bits >> 2) as i64))
        } else {
            None
        }
    }
}

impl Type {
    pub fn type_fixnum(&self) -> bool {
        match self.tag() {
            Tag::Efixnum => true,
            Tag::Ofixnum => true,
            _ => false
        }
    }
    
    pub fn i64_from_fixnum(&self) -> Option<i64> {
        if Type::type_fixnum(self) {
            Some((self.bits >> 2) as i64)
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
        assert!(_fixnum(0).type_fixnum());
    }

    #[test]
    fn test_i64() {
        assert!(
            match _fixnum(0).i64_from_fixnum() {
                None => false,
                Some(v) => v == 0
            });

        assert!(
            match _fixnum(1).i64_from_fixnum() {
                None => false,
                Some(v) => v == 1
            });
    }

    #[test]
    fn test_eq() {
        assert!(_fixnum(0).eq(_fixnum(0)));
        assert!(!_fixnum(0).eq(_fixnum(1)));
    }

    #[test]
    fn test_add() {
        assert!(
            match _Fixnum::_from_type(&_fixnum(1)) {
                Some(fx) =>
                    match fx._add(&_fixnum(2)) {
                        Some(sum) =>
                            match sum.i64_from_fixnum() {
                                Some(v) => v == 3,
                                None => false
                            },
                        None => false
                    },
                None => false
            });
    }
}
