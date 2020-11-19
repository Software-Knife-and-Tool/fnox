// mu/print.rs
use std::fmt;
use std::char::from_u32;

use crate::mu::r#type::{ImmediateClass};
use crate::mu::r#type::{SysClass, Tag, Type};

pub fn to_string(src: Type) -> String {
    match src.type_of() {
        SysClass::String => {
            let _str = &Type::str_from_type(&src);
            format!("\"{}\"", _str)
        }
        SysClass::Symbol => {
            if src.typep_keyword() {
                format!(":{}", src.str_from_type())
            } else {
                let sym = Type::symbol_from_type(&src);
                let name = to_string(*sym.name());

                format!("{:?}", name)
            }
        }
        SysClass::Char => format!("#\\{}", from_u32(src.immediate_data() as u32).unwrap()),
        SysClass::Cons => format!("[#<cons>]"),
        SysClass::Exception => format!("#<exception>"),
        SysClass::Fixnum => format!("{:?}", src.i64_from_fixnum().unwrap()),
        SysClass::Float => format!("[float]"),
        SysClass::Function => format!("#<function>"),
        SysClass::Stream => format!("#<stream>"),
        SysClass::T => format!("#<T>"),
        SysClass::Vector => format!("[vector]"),
    }
}

pub fn debug_println(obj: Type) {
    let typestr = |obj: Type| match obj.tag() {
        Tag::Cons => "#<cons>",
        Tag::Fixnum => "#<fixnum>",
        Tag::Exception => "#<exception>",
        Tag::Function => "#<function>",
        Tag::Stream => "#<steam>",
        Tag::Symbol => "#<symbol>",
        Tag::Vector => "#<vector>",
        Tag::Immediate => match Type::immediate_class(&obj) {
            ImmediateClass::Char => "#<char>",
            ImmediateClass::String => "#<immediate-string>",
            ImmediateClass::Keyword => "#<keyword>",
            ImmediateClass::Float => "#<float>",
        },
    };

    print!("debug: tag {:x} type {}", obj.as_u64(), typestr(obj));

    /*
    match src.type_of() {
        SysClass::String => {
            let _str = &Type::str_from_type(&src);
        }
        SysClass::Symbol => {
            if src.typep_keyword() {
                format!(":{}", src.str_from_type())
            } else {
                let sym = Type::symbol_from_type(&src);
                let name = to_string(*sym.name());

                format!("{:?}", name)
            }
        }
        SysClass::Char => format!("#\\{}", from_u32(src.immediate_data() as u32).unwrap()),
        SysClass::Cons => format!("[#<cons>]"),
        SysClass::Exception => format!("#<exception>"),
        SysClass::Fixnum => format!("{:?}", src.i64_from_fixnum().unwrap()),
        SysClass::Float => format!("[float]"),
        SysClass::Function => format!("#<function>"),
        SysClass::Stream => format!("#<stream>"),
        SysClass::T => format!("#<T>"),
        SysClass::Vector => format!("[vector]"),
    }
     */
}

pub fn _print(src: Type) {
    println!("{}", to_string(src))
}

impl fmt::Display for Type {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.as_u64())
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    /*
    #[test]
    fn test_type() {
        assert!(fixnum(0).typep_fixnum());
    }

    #[test]
    fn test_u64() {
        assert!(fixnum(0).u64_from_fixnum() == 0);
        assert!(fixnum(1).u64_from_fixnum() == 1);
    }

    #[test]
    fn test_eq() {
        assert!(fixnum(0).eq(fixnum(0)));
    }

    #[test]
    fn test_add() {
        assert!(fixnum(1).fixnum_add(fixnum(2)).eq(fixnum(3)));
    }
     */
}
