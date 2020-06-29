#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/* mu lib.rs */
pub mod mu {
    pub mod env {
        pub fn env() { println!("env!"); }
    }
}
