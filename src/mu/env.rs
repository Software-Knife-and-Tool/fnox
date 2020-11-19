// mu/env.rs
use std::collections::HashMap;

use crate::mu::heap::{FnHeap, _heap};

use crate::mu::r#type::{Tag, Type};
use crate::mu::r#type::{NIL, T};

use crate::mu::print::{_print, debug_println};
use crate::mu::read::read_from_stdin;

use crate::mu::fixnum::*;
use crate::mu::function::FnFunction;
use crate::mu::string::FnString;

pub struct FnEnv<'e> {
    pub heap: FnHeap,
    pub symtab: HashMap<&'e str, Type>,
}

pub fn env<'e>() -> FnEnv<'e> {
    let mut init: HashMap<&'e str, Type> = HashMap::new();

    init.insert(
        "fixnum-add",
        FnFunction::make_type(FnString::make_type("fixnum-add"), fx_add, 2),
    );

    init.insert(
        "fixnum-sub",
        FnFunction::make_type(FnString::make_type("fixnum-sub"), fx_sub, 2),
    );

    init.insert(
        "fixnum-mul",
        FnFunction::make_type(FnString::make_type("fixnum-mul"), fx_mul, 2),
    );

    init.insert(
        "fixnum-trunc",
        FnFunction::make_type(FnString::make_type("fixnum-trunc"), fx_trunc, 2),
    );

    init.insert(
        "mod",
        FnFunction::make_type(FnString::make_type("mod"), fx_mod, 2),
    );

    init.insert(
        "logand",
        FnFunction::make_type(FnString::make_type("logand"), fx_logand, 2),
    );

    init.insert(
        "minusp",
        FnFunction::make_type(FnString::make_type("minusp"), fx_minusp, 1),
    );

    FnEnv {
        heap: _heap(1024 * 1024),
        symtab: init,
    }
}

impl FnEnv<'_> {
    pub fn read(&self) -> Type {
        read_from_stdin(T)
    }

    pub fn print(&self, src: Type) {
        _print(src);
    }

    pub fn eval(&self, ptr: Type) -> Type {
        print!("eval: ");
        debug_println(ptr);
        match ptr.tag() {
            Tag::Cons => {
                let cons = ptr.cons_from_type();
                let fsym = cons.car();

                debug_println(fsym);
                if !fsym.typep_symbol() {
                    println!("unquoted list form");
                    NIL
                } else {
                    let sym = Type::symbol_from_type(&fsym);
                    let fn_ = *sym.value();

                    if !fn_.typep_function() {
                        println!("not a function");
                        NIL
                    } else {
                        fn_.funcall(self, NIL)
                    }
                }
            }
            Tag::Fixnum => {
                println!("looks like a fixnum");
                ptr
            }
            Tag::Exception => {
                println!("looks like an exception");
                ptr
            }
            Tag::Function => {
                println!("looks like a function");
                ptr
            }
            Tag::Stream => {
                println!("looks like a stream");
                ptr
            }
            Tag::Symbol => {
                println!("looks like a symbol");
                let sym = Type::symbol_from_type(&ptr);
                *sym.value()
            }
            Tag::Vector => {
                println!("looks like a vector");
                ptr
            }
            Tag::Immediate => {
                println!("looks like an immediate");
                ptr
            }
        }
    }

    pub fn lookup(&self, name: &str) -> Type {
        match self.symtab.get(name) {
            Some(_type) => *_type,
            None => NIL,
        }
    }
}

#[cfg(test)]
mod tests {
    /*
        use super::*;

        #[test]
        fn test_symtab() {
            let env = env();
            assert!(!env.lookup(&"fixnum-add").eq(NIL));
            assert!(env.lookup(&"nope").eq(NIL));
        }
    */
}
