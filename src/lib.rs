/* mu lib.rs */
extern crate num;

#[macro_use]
extern crate num_derive;

mod mu;

pub use crate::mu::env;

#[cfg(test)]
mod tests {
    #[test]
    fn eq_t() {
        assert!(true, true);
    }
}

