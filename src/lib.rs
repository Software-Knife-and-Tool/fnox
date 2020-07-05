/* mu lib.rs */
extern crate num;

#[macro_use]
extern crate num_derive;

#[macro_use]
extern crate bitfield;

mod mu;

pub use crate::mu::env;

#[cfg(test)]
mod tests {
    #[test]
    fn eq_t() {
        assert!(true, true);
    }
}

