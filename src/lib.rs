/* mu lib.rs */
extern crate num;
#[macro_use]
extern crate num_derive;

mod mu;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub use crate::mu::env;
