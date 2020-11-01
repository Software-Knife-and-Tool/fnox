/* mu/float.rs */
use crate::mu::r#type::{Type, Tag};
use crate::mu::r#type::{T, NIL};
use crate::mu::r#type::{ImmediateClass, _immediate};

#[derive(Debug)]
pub struct _Float {
    float: f32
}

pub fn _float(src: f32) -> Type {
    _immediate((src as u64) << 24, 0, ImmediateClass::Float)
}

pub fn _float_add(args: Vec<Type>) -> Type {
    match _Float::_from_type(&args[0]) {
        Some(fl) =>
            match fl._add(&args[1]) {
                Some(s) => s,
                None => NIL
            },
        None => NIL
    }
}

impl _Float {
    pub fn _print(&self) {
        println!("{}", self.float);
    }
    
    pub fn _from_f32(_float: f32) -> _Float {
        _Float { float: _float }
    }

    pub fn _from_type(fl: &Type) -> Option<_Float> {
        if Type::type_float(fl) {
            Some(_Float { float: (fl.as_u64() >> 32) as f32 })
        } else {
            None
        }
    }

    pub fn _add(&self, fl: &Type) -> Option<Type> {
        if Type::type_float(fl) {
            Some(_float(self.float + (fl.as_u64() >> 32) as f32))
        } else {
            None
        }
    }

    pub fn _mul(&self, fl: &Type) -> Option<Type> {
        if Type::type_float(fl) {
            Some(_float(self.float * (fl.as_u64() >> 32) as f32))
        } else {
            None
        }
    }

    pub fn _div(&self, fl: &Type) -> Option<Type> {
        if Type::type_float(fl) {
            Some(_float(self.float / (fl.as_u64() >> 32) as f32))
        } else {
            None
        }
    }

    pub fn _minusp(&self) -> Type {
        println!("minusp: {:x?}", self.float);
        if self.float < 0.0 { T } else { NIL }
    }
}

impl Type {
    pub fn type_float(&self) -> bool {
        match self.tag() {
            Tag::Immediate =>
                match self.immediate_class() {
                    ImmediateClass::Float => true,
                    _ => false
                },
            _ => false
        }
    }
    
    pub fn f32_from_float(&self) -> Option<f32> {
        if Type::type_float(self) {
            Some((self.as_u64() >> 32) as f32)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type() {
        assert!(_float(0.0).type_float());
    }

    #[test]
    fn test_float() {
        assert!(
            match _float(0.0).f32_from_float() {
                None => false,
                Some(v) => v == 0.0
            });

        assert!(
            match _float(1.0).f32_from_float() {
                None => false,
                Some(v) => v == 1.0
            });
    }

    #[test]
    fn test_eq() {
        assert!(_float(0.0).eq(_float(0.0)));
        assert!(_float(1.0).eq(_float(1.0)));
        assert!(_float(-1.0).eq(_float(-1.0)));
        assert!(!(_float(1.0).eq(_float(0.0))));
        assert!(!(_float(-1.0).eq(_float(0.0))));
        assert!(!(_float(-10.0).eq(_float(-200.0))));
    }

    #[test]
    fn test_minusp() {
        assert!(
            match _Float::_from_type(&_float(-1.0)) {
                Some(fl) => fl._minusp().eq(T),
                None => false
            });
    }
    
    #[test]
    fn test_add() {
        assert!(
            match _Float::_from_type(&_float(1.0)) {
                Some(fl) =>
                    match fl._add(&_float(2.0)) {
                        Some(sum) =>
                            match sum.f32_from_float() {
                                Some(v) => v == 3.0,
                                None => false
                            },
                        None => false
                    },
                None => false
            });
    }

    #[test]
    fn test_mul() {
        assert!(
            match _Float::_from_type(&_float(2.0)) {
                Some(fl) =>
                    match fl._mul(&_float(3.0)) {
                        Some(sum) =>
                            match sum.f32_from_float() {
                                Some(v) => v == 6.0,
                                None => false
                            },
                        None => false
                    },
                None => false
            });
    }

    #[test]
    fn test_div() {
        assert!(
            match _Float::_from_type(&_float(4.0)) {
                Some(fl) =>
                    match fl._div(&_float(2.0)) {
                        Some(sum) =>
                            match sum.f32_from_float() {
                                Some(v) => v == 2.0,
                                None => false
                            },
                        None => false
                    },
                None => false
            });
    }

}
