/* mu/fixnum.rs */
use crate::mu::r#type::Tag;
use crate::mu::r#type::Type;
use crate::mu::r#type::_entag;

pub fn _fixnum(src: u64) -> Type {
    return _entag(src << 2, Tag::Efixnum); 
}

impl Type {
    pub fn type_(&self) -> bool {
        match self.tag() {
            Tag::Efixnum => true,
            Tag::Ofixnum => true,
            _ => false
        }
    }
    pub fn u64_of(&self) -> u64 {
        self.bits >> 2
    }

    pub fn add(&self, fx: Type) -> Type {
        _fixnum(self.u64_of() + fx.u64_of())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type() {
        assert!(_fixnum(0).type_());
    }

    #[test]
    fn test_u64() {
        assert!(_fixnum(0).u64_of() == 0);
        assert!(_fixnum(1).u64_of() == 1);
    }

    #[test]
    fn test_eq() {
        assert!(_fixnum(0).eq(_fixnum(0)));
    }
    
    #[test]
    fn test_add() {
        assert!(_fixnum(1).add(_fixnum(2)).eq(_fixnum(3)));
    }
}
