/* mu/env.rs */
pub struct Env {
    stuff: i64
}

pub fn make() -> Env {
    println!("making env, damnit");
    Env {
        stuff: 0
    }
}

impl Env {

}

