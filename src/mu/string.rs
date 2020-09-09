/* mu/string.rs */
use crate::mu::r#type::Tag;
use crate::mu::r#type::Type;
use crate::mu::r#type::entag;
use crate::mu::r#type::detag;
use crate::mu::r#type::NIL;

#[derive(Debug)]
pub struct _String {
    _value: &'static str
}

pub fn _string(_value: &[u8]) -> Type {
    match std::str::from_utf8(_value) {
        Ok(str) => Type::from_string(str),
        Err(_) => NIL
    }
}

impl _String {
    pub fn _string_value(self) -> &'static str {
        self._value
    }
}

impl Type {
    pub fn type_string(&self) -> bool {
        match self.tag() {
            Tag::Extend => true,
            _ => false
        }
    }
    
    pub fn from_string(_str: &str) -> Type {
        unsafe {
            let str_addr: u64 = std::mem::transmute(&_str);
            entag(str_addr << 3, Tag::Extend)
        }
    }

    pub fn string_from_type(self) -> Option<&'static _String> {
        if Type::type_string(&self) {
            let str: &'static _String = unsafe { std::mem::transmute(detag(self)) };
            Some(str)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        assert!(_string(b"yep").type_string());
        assert!(
            match Type::string_from_type(_string(b"astring")) {
                Some(_) => true,
                None => false
            });
    }
}
