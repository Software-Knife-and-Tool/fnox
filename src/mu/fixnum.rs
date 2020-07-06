/* mu/fixnum.rs */
use crate::mu::r#type::Tag;
use crate::mu::r#type::Type;
use crate::mu::r#type::entag;

pub fn fixnum(src: u64) -> Type {
    return entag(src << 2, Tag::Efixnum); 
}

impl Type {
    pub fn is_fixnum(&self) -> bool {
        match self.tag() {
            Tag::Efixnum => true,
            Tag::Ofixnum => true,
            _ => false
        }
    }
    pub fn u64_of_fixnum(&self) -> u64 {
        return self.bits >> 3;
    }
}
