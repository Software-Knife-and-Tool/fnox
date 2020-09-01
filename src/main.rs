use mu::env::env;

fn main() {
    println!("lispox 0.0.2");

    let e = env();

    e.print(e.read());
}
