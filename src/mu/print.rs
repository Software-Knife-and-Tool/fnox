/* mu/print.rs */
use std::char::from_u32;

use crate::mu::r#type::{SysClass, Type, NIL};

fn to_string(src: Type) -> String {
    match src.type_of() {
        SysClass::String => {
            let _str = &Type::str_from_type(&src);
            format!("[string] \"{:?}\"", _str)
        }
        SysClass::Symbol => {
            if src.eq(NIL) {
                format!(":nil")
            } else if src.typep_keyword() {
                format!(":{}", "keyword")
            } else {
                let sym = Type::symbol_from_type(&src);
                let name = to_string(*sym.name());

                format!("{:?}", name)
            }
        }
        SysClass::Char => format!("#\\{}", from_u32(src.immediate_data() as u32).unwrap()),
        SysClass::Cons => format!("(...)"),
        SysClass::Exception => format!("#<exception>"),
        SysClass::Fixnum => format!("{:?}", src.i64_from_fixnum().unwrap()),
        SysClass::Float => format!("[float]"),
        SysClass::Function => format!("#<function>"),
        SysClass::Stream => format!("#<stream>"),
        SysClass::Vector => format!("[vector]"),
    }
}

pub fn _print(src: Type) {
    println!("{}", to_string(src))
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
