use mu::env::env;

fn main() {
    println!("fnox 0.0.5");

    match std::env::args().nth(1) {
        Some(path) => println!(";;; loading {}", path),
        None => (),
    }

    let e = env();

    loop {
        e.print(e.eval(e.read()));
    }
}
