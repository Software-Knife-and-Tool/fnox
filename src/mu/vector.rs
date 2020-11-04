/* mu/vector.rs */
use crate::mu::r#type::{SysClass, Tag, Type};

#[derive(Debug)]
enum Vector {
    _String(String),
}

#[derive(Debug)]
pub struct _Vector {
    _type: SysClass,
    _vector: Vector,
}

impl _Vector {
    pub fn _vector_type(&self) -> &SysClass {
        &self._type
    }

    /*
        fn homestar(&self) -> String {
            use PlayerClass::*;
            match self {
                Sol(_) => String::from("sun"),
                ...
            }
        }
    */
}

impl Type {
    pub fn is_vector(&self) -> bool {
        match self.tag() {
            Tag::Vector => true,
            _ => false,
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
