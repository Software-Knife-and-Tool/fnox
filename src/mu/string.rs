/* mu/string.rs */
use crate::mu::r#type::NIL;
use crate::mu::r#type::{detag, entag, Tag, Type};
use crate::mu::r#type::{immediate, ImmediateClass, IMMEDIATE_STR_MAX};

#[derive(Debug)]
pub struct String {
    pub _value: Type,
}

impl String {
    pub fn make_type(str_: &str) -> Type {
        if str_.len() <= IMMEDIATE_STR_MAX {
            let mut charf: u64 = 0;
            for ch in str_.as_bytes() {
                charf = (charf << 8) | *ch as u64;
            }
            immediate(charf, str_.len() as u8, ImmediateClass::String)
        } else {
            unsafe {
                let addr: u64 = std::mem::transmute(&str_.as_bytes());
                entag(addr << 3, Tag::Vector)
            }
        }
    }
}

impl Type {
    pub fn typep_string(&self) -> bool {
        match self.tag() {
            Tag::Vector => true,
            Tag::Immediate => match self.immediate_class() {
                ImmediateClass::String => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn str_from_type(&self) -> std::string::String {
        match self.tag() {
            Tag::Immediate => {  // clean this the hell up
                let l = self.immediate_size();
                let v = &self.immediate_data().to_be_bytes();
                let s = std::str::from_utf8(v).unwrap().to_string();
                let c = &s[(8 - l)..];
                
                c.to_string()
            }
            Tag::Vector => std::str::from_utf8(b"char-vector").unwrap().to_string(),
            _ => std::str::from_utf8(b"whoa").unwrap().to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    /*
    use super::*;

    #[test]
    fn test_string() {
        assert!(_string(b"yep").typep_string());
        assert!(
            match Type::str_from_type(_string(b"astring")) {
                Some(_) => true,
                None => false
        });
    }
    */
}
