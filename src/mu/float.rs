/* mu/float.rs */
use crate::mu::r#type::{immediate, ImmediateClass};
use crate::mu::r#type::{Tag, Type};
use crate::mu::r#type::{NIL, T};

#[derive(Debug)]
pub struct Float {
    float: f32,
}

pub fn float(src: f32) -> Type {
    immediate((src as u64) << 24, 0, ImmediateClass::Float)
}

pub fn float_add(args: Vec<Type>) -> Type {
    match Float::from_type(&args[0]) {
        Some(fl) => match fl.add(&args[1]) {
            Some(s) => s,
            None => NIL,
        },
        None => NIL,
    }
}

impl Float {
    pub fn print(&self) {
        println!("{}", self.float);
    }

    pub fn from_f32(float: f32) -> Float {
        Float { float: float }
    }

    pub fn from_type(fl: &Type) -> Option<Float> {
        if Type::typep_float(fl) {
            Some(Float {
                float: (fl.as_u64() >> 32) as f32,
            })
        } else {
            None
        }
    }

    pub fn add(&self, fl: &Type) -> Option<Type> {
        if Type::typep_float(fl) {
            Some(float(self.float + (fl.as_u64() >> 32) as f32))
        } else {
            None
        }
    }

    pub fn mul(&self, fl: &Type) -> Option<Type> {
        if Type::typep_float(fl) {
            Some(float(self.float * (fl.as_u64() >> 32) as f32))
        } else {
            None
        }
    }

    pub fn div(&self, fl: &Type) -> Option<Type> {
        if Type::typep_float(fl) {
            Some(float(self.float / (fl.as_u64() >> 32) as f32))
        } else {
            None
        }
    }

    pub fn minusp(&self) -> Type {
        if self.float < 0.0 {
            T
        } else {
            NIL
        }
    }
}

impl Type {
    pub fn typep_float(&self) -> bool {
        match self.tag() {
            Tag::Immediate => match self.immediate_class() {
                ImmediateClass::Float => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn f32_from_float(&self) -> Option<f32> {
        if Type::typep_float(self) {
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
        assert!(float(0.0).typep_float());
    }

    #[test]
    fn test_float() {
        assert!(match float(0.0).f32_from_float() {
            None => false,
            Some(v) => v == 0.0,
        });

        assert!(match float(1.0).f32_from_float() {
            None => false,
            Some(v) => v == 1.0,
        });
    }

    #[test]
    fn test_eq() {
        assert!(float(0.0).eq(float(0.0)));
        assert!(float(1.0).eq(float(1.0)));
        assert!(float(-1.0).eq(float(-1.0)));
        assert!(!(float(1.0).eq(float(0.0))));
        assert!(!(float(-1.0).eq(float(0.0))));
        assert!(!(float(-10.0).eq(float(-200.0))));
    }

    #[test]
    fn test_minusp() {
        assert!(match Float::from_type(&float(-1.0)) {
            Some(fl) => fl.minusp().eq(T),
            None => false,
        });
    }

    #[test]
    fn test_add() {
        assert!(match Float::from_type(&float(1.0)) {
            Some(fl) => match fl.add(&float(2.0)) {
                Some(sum) => match sum.f32_from_float() {
                    Some(v) => v == 3.0,
                    None => false,
                },
                None => false,
            },
            None => false,
        });
    }

    #[test]
    fn test_mul() {
        assert!(match Float::from_type(&float(2.0)) {
            Some(fl) => match fl.mul(&float(3.0)) {
                Some(sum) => match sum.f32_from_float() {
                    Some(v) => v == 6.0,
                    None => false,
                },
                None => false,
            },
            None => false,
        });
    }

    #[test]
    fn test_div() {
        assert!(match Float::from_type(&float(4.0)) {
            Some(fl) => match fl.div(&float(2.0)) {
                Some(sum) => match sum.f32_from_float() {
                    Some(v) => v == 2.0,
                    None => false,
                },
                None => false,
            },
            None => false,
        });
    }
}
