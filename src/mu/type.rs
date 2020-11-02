/* mu/r#type.rs */
use std::io::{self, Write};

use crate::mu::cons::_Cons;
use crate::mu::exception::_Exception;
use crate::mu::fixnum::_Fixnum;
use crate::mu::function::_Function;
use crate::mu::stream::_Stream;
use crate::mu::symbol::_Symbol;
use crate::mu::vector::_Vector;

#[derive(Debug)]
pub struct Type(u64);

#[derive(FromPrimitive)]
pub enum Tag {
    Fixnum = 0,    /* fixnum 61 bites */
    Cons = 1,      /* cons */
    Symbol = 2,    /* symbol/keyword */
    Function = 3,  /* function */
    Exception = 4, /* exception */
    Stream = 5,    /* stream */
    Vector = 6,    /* vector */
    Immediate = 7, /* immediate (char, keyword, small string, float) */
}

#[derive(Debug)]
pub enum TagClass {
    Cons(_Cons),
    Exception(_Exception),
    Fixnum(_Fixnum),
    Function(_Function),
    Immediate(Type),
    Stream(_Stream),
    Symbol(_Symbol),
    Vector(_Vector)
}

#[derive(Debug)]
pub enum SysClass {
    Char,
    Cons,
    Exception,
    Fixnum,
    Float,
    Function,
    Stream,
    String,
    Symbol,
    Vector
}

#[derive(FromPrimitive)]
pub enum ImmediateClass {
    Char = 0,
    String = 1,
    Keyword = 2,
    Float = 3
}

const _IMMEDIATE_STR_MAX: u64 = 7;

pub const T: Type = Type {
    0: (('t' as u64) << 8)
        | (1 << 5)
        | ((ImmediateClass::Keyword as u64) << 3)
        | (Tag::Immediate as u64)
};

pub const NIL: Type = Type {
    0: (((('l' as u64) << 16)
         | (('i' as u64) << 8)
         | (('n' as u64))) << 8)
        | (3 << 5)
        | ((ImmediateClass::Keyword as u64) << 3)
        | (Tag::Immediate as u64)
};

pub fn entag(base: u64, tag: Tag) -> Type {
    Type { 0: base | tag as u64 }
}

pub fn detag(_type: &Type) -> u64 {
    (_type.0 >> 3) as u64
}

pub fn _immediate(data: u64, len: u8, tag: ImmediateClass) -> Type {
    Type {
        0: (data << 8)
            | ((len as u64) << 5)
            | ((tag as u64) << 3)
            | ((Tag::Immediate as u64))
    }
}

impl Type {

    pub fn as_u64(&self) -> u64 {
        self.0
    }
    
    pub fn tag(&self) -> Tag {
        let tag: std::option::Option<Tag> =
            num::FromPrimitive::from_u64(self.0 & 0x7);
        match tag {
            Some(_) => tag.unwrap(),
            None => panic!("Unknown tag")
        }
    }

    pub fn type_of(&self) -> SysClass {
        match self.tag() {
            Tag::Cons => SysClass::Cons,
            Tag::Fixnum => SysClass::Fixnum,
            Tag::Exception => SysClass::Exception,
            Tag::Function => SysClass::Function,
            Tag::Stream => SysClass::Stream,
            Tag::Symbol => SysClass::Symbol,
            Tag::Vector => SysClass::Vector,
            Tag::Immediate =>
                match Type::immediate_class(self) {
                    ImmediateClass::Char => SysClass::Char,
                    ImmediateClass::String => SysClass::Vector,
                    ImmediateClass::Keyword => SysClass::Symbol,
                    ImmediateClass::Float => SysClass::Float
                }
        }
    }

    pub fn type_char(&self) -> bool {
        match self.tag() {
            Tag::Immediate =>
                match Type::immediate_class(self) {
                    ImmediateClass::Char => true,
                    _ => false
                },
            _ => false
        }
    }
    
    pub fn immediate_data(&self) -> u64 {
        (self.0 >> 8) as u64
    }

    pub fn immediate_size(&self) -> u64 {
        ((self.0 >> 5) & 7) as u64
    }
    
    pub fn immediate_class(&self) -> ImmediateClass {
        let tag: std::option::Option<ImmediateClass> =
            num::FromPrimitive::from_u64((self.0 >> 3) & 3);

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
        // println!("{:x?} v {:x?}", self.as_u64(), ptr.as_u64());
        io::stdout().flush().unwrap();
        self.0 == ptr.0
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
        assert!(T.eq(T));
    }
}
