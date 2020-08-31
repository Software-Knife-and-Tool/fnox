/* mu/r#type.rs */
use crate::mu::cons::_Cons;
use crate::mu::extend::_Extend;
use crate::mu::fixnum::_Fixnum;
use crate::mu::function::_Function;
use crate::mu::symbol::_Symbol;

#[derive(Debug)]
pub struct Type {
    pub bits: u64
}

#[derive(FromPrimitive)]
pub enum Tag {
    Address = 0,   /* machine address */
    Efixnum = 1,   /* even fixnum (62 bits) */
    Symbol = 2,    /* symbol/keyword */
    Function = 3,  /* function */
    Cons = 4,      /* cons */
    Ofixnum = 5,   /* odd fixnum (62 bits) */
    Immediate = 6, /* immediate */
    Extend = 7     /* extended */
}

pub enum TagClass {
    Address(i32),
    Fixnum(_Fixnum),
    Symbol(_Symbol),
    Function(_Function),
    Cons(_Cons),
    Immediate(Type),
    Extend(i32)
}

pub enum SysClass {
    Cons,
    Fixnum,
    Function,
    String,
    Symbol,
    T
}

#[derive(FromPrimitive)]
pub enum ImmediateClass {
    Char = 0,
    String = 1,
    Keyword = 2,
    Float = 3
}

const _IMMEDIATE_STR_MAX: u64 = 7;

pub const _T: Type = Type {
    bits: (('t' as u64) << 8)
        | (1 << 5)
        | ((ImmediateClass::Keyword as u64) << 3)
        | (Tag::Immediate as u64)
};

pub const NIL: Type = Type {
    bits: (((('l' as u64) << 16)
            | (('i' as u64) << 8)
            | (('n' as u64))) << 8)
        | (3 << 5)
        | ((ImmediateClass::Keyword as u64) << 3)
        | (Tag::Immediate as u64)
};

pub fn entag(base: u64, tag: Tag) -> Type {
    Type {
        bits: base | tag as u64
    }
}

pub fn detag(_type: Type) -> u64 {
    _type.bits >> 3
}

pub fn _immediate(data: u64, len: u8, tag: ImmediateClass) -> Type {
    Type {
        bits: (data << 8)
            | ((len as u64) << 5)
            | ((tag as u64) << 3)
            | ((Tag::Immediate as u64))
    }
}

impl Type {
    pub fn tag(&self) -> Tag {
        let tag: std::option::Option<Tag> =
            num::FromPrimitive::from_u64(self.bits & 0x7);
        match tag {
            Some(_) => tag.unwrap(),
            None => panic!("Unknown tag")
        }
    }

    pub fn type_of(&self) -> SysClass {
        match self.tag() {
            Tag::Address => SysClass::T, 
            Tag::Cons => SysClass::Cons,
            Tag::Efixnum | Tag::Ofixnum => SysClass::Fixnum,
            Tag::Extend => SysClass::T,
            Tag::Function => SysClass::Function,
            Tag::Immediate => SysClass::T,
            Tag::Symbol => SysClass::Symbol
        }
    }
    
    pub fn immediate_data(&self) -> u64 {
        self.bits >> 8
    }

    pub fn immediate_size(&self) -> u64 {
        self.bits >> 5
    }
    
    pub fn immediate_class(&self) -> ImmediateClass {
        let tag: std::option::Option<ImmediateClass> =
            num::FromPrimitive::from_u64(self.bits >> 3);
        match tag {
            Some(_) => tag.unwrap(),
            None => panic!("Unknown tag")
        }
    }

    pub fn is_immediate(&self) -> bool {
        match self.tag() {
            Tag::Immediate => true,
            _ => false
        }
    }
  
    pub fn eq(&self, ptr: Type) -> bool {
        self.bits == ptr.bits
    }

    pub fn null(&self) -> bool {
        self.eq(NIL)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_eq() {
        assert!(_T.eq(_T));
    }
}
