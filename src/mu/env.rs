/* mu/env.rs */

struct Env {
    stuff: i64
}

pub fn env() {
    println!("env, damnit");
}

impl Env {
    pub fn make() -> Env {
        println!("making env, damnit");
        Env {
            stuff: 0
        }
    }
}
