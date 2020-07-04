/* mu/fixnum.rs */
use crate::mu::r#type::Tag;
use crate::mu::r#type::Type;
use crate::mu::r#type::entag;

pub fn fixnum(src: u64) -> Type {
    return entag(src << 2, Tag::Efixnum); 
}

