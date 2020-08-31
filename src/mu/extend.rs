/* mu/extend.rs */
use crate::mu::string::_String;
use crate::mu::r#type::Type;
use crate::mu::r#type::Tag;

#[derive(Debug)]
pub struct _Extend {
    pub bits: u64
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
