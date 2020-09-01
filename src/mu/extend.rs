/* mu/extend.rs */
use crate::mu::string::_String;
use crate::mu::r#type::Type;
use crate::mu::r#type::SysClass;
use crate::mu::r#type::Tag;
use crate::mu::string::_string;

#[derive(Debug)]
enum Extended {
    string(_String)
}

#[derive(Debug)]
pub struct _Extend {
    _type: SysClass,
    _struct: Extended
}

impl _Extend {
    pub fn _extended_type(&self) -> &SysClass {
        &self._type
    }    
}

impl Type {
    pub fn is_extended(&self) -> bool {
        match self.tag() {
            Tag::Extend => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod tests {
/*
    use super::*;
    
    #[test]
    fn test_immed() {
        assert!(_T.eq(_T));
    }
*/
}
