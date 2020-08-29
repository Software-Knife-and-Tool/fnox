/* mu/string.rs */
use crate::mu::r#type::Tag;
use crate::mu::r#type::Type;
use crate::mu::r#type::entag;
use crate::mu::r#type::detag;

pub struct _String {
    _value: String
}

pub fn _string(_value: String) -> Type {
    let str = _String { _value };
    
    Type::from_string(&str)
}

impl _String {

}

impl Type {
    pub fn type_string(&self) -> bool {
        match self.tag() {
            Tag::Extend => true,
            _ => false
        }
    }
    
    pub fn from_string(_str: &_String) -> Type {
        unsafe {
            let str_addr: u64 = std::mem::transmute(_str);
            entag(str_addr << 3, Tag::Extend)
        }
    }
    
    pub fn string_from_type(self) -> &'static _String {
        let str: &_String = unsafe { std::mem::transmute(detag(self)) };
        str
    }
}

#[cfg(test)]
mod tests {
    /*
    use super::*;

    #[test]
    fn test_type() {
        assert!(NIL.cons(NIL).type_function());
    }
     */
}
