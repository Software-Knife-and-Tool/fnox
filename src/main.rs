use mu::env::env;

fn main() {
    println!("lispox 0.0.2");

    let e = env();

    loop {
        e.print(e.read());
    }
}
