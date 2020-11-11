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
            let mut charf : u64 = 0;
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
            Tag::Immediate =>
                match self.immediate_class() {
                  ImmediateClass::String => true,
                  _ => false,
                }
            _ => false,
        }
    }

    pub fn string_from_type(&self) -> &'static String {
        let str: &String = unsafe { std::mem::transmute(detag(self)) };
        str
    }
    
    pub fn str_from_type(&self) -> &str {
        match self.tag() {
            Tag::Immediate => {
                /*
                let mut chars = self.immediate_data();
                let mut v = &[u8; self.immediate_size()];

                for ch in &v {
                    v[i] = chars & 0xff;
                    chars /= 8;
                }
                
                std::str::from_utf8(v).unwrap()
                 */
                std::str::from_utf8(b"immediate-string").unwrap()
            },
            Tag::Vector => std::str::from_utf8(b"char-vector").unwrap(),
            _ => std::str::from_utf8(b"whoa").unwrap(),
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
