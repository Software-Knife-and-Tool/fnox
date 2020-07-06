/* mu lib.rs */
extern crate num;

#[macro_use]
extern crate num_derive;

mod mu;

pub use crate::mu::env;
pub use crate::mu::fixnum;
pub use crate::mu::r#type;

#[cfg(test)]
mod tests {
    #[test]
    fn eq_t() {
        assert!(true, true);
    }
    #[test]
    fn immed() {
        let im: Type = immediate(0, 0, ImmediateClass::Keyword);
        assert!(is_immediate(im), true);
    }
}

