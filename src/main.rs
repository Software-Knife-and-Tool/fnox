use mu::env::env;

fn main() {
    println!("lispox 0.0.3");

    let e = env();

    loop {
        e.print(e.read());
    }
}
