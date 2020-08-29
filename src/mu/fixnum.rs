/* mu/fixnum.rs */
use crate::mu::r#type::Tag;
use crate::mu::r#type::Type;
use crate::mu::r#type::entag;

struct _Fixnum {
    integer: i64
}

pub fn fixnum(src: u64) -> Type {
    return entag(src << 2, Tag::Efixnum); 
}

impl _Fixnum {
    pub fn _print(_type: Type) {
        println!("{}", Type::u64_from_fixnum(&_type));
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

    pub fn u64_from_fixnum(&self) -> u64 {
        self.bits >> 2
    }

    pub fn fixnum_add(&self, fx: Type) -> Type {
        fixnum(self.u64_from_fixnum() + fx.u64_from_fixnum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
