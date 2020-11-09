/* mu/print.rs */
// use std::io;

use std::char::from_u32;

use crate::mu::fixnum::*;
use crate::mu::r#type::{SysClass, Type, NIL};

pub fn _print(src: Type) {
    match src.type_of() {
        SysClass::String => {
            let _str = &Type::str_from_type(&src);
            println!("\"{:?}\"", _str)
        },
        SysClass::Symbol => {
            if src.eq(NIL) {
                println!(":nil");
            } else if src.typep_keyword() {
                println!(":{}", "keyword");
            } else {
                let sym = Type::symbol_from_type(&src);
                let name = sym.name();
                let _str = Type::str_from_type(name);

                // println!("{:?}", &_str._value);
            }
        },
        SysClass::Fixnum => match Fixnum::from_type(&src) {
            Some(fx) => {
                println!("fixnum:");
                fx.print()
            },
            None => println!("isn't a fixnum"),
        },
        SysClass::Char => println!("#\\{}", from_u32(src.immediate_data() as u32).unwrap()),
        SysClass::Cons => println!("is a cons"),
        SysClass::Exception => println!("is an exception"),
        SysClass::Float => println!("is a float"),
        SysClass::Function => println!("is a function"),
        SysClass::Stream => println!("is a stream"),
        SysClass::Vector => println!("is a vector"),
        _ => println!("undecoded, should panic"),
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
