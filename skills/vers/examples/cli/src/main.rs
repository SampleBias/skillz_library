fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("--help") => println!("usage: greet NAME"),
        Some(name) => println!("hello {name}"),
        None => {
            eprintln!("missing NAME");
            std::process::exit(2);
        }
    }
}
