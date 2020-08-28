/* mu/cons.rs */
use crate::mu::r#type::Tag;
use crate::mu::r#type::Type;
use crate::mu::r#type::entag;

pub fn _cons(_car: Type, _cdr: Type) -> Type {
    return entag(1 << 2, Tag::Efixnum); 
}

impl Type {
    pub fn cons_type(&self) -> bool {
        match self.tag() {
            Tag::Efixnum => true,
            Tag::Ofixnum => true,
            _ => false
        }
    }
}
