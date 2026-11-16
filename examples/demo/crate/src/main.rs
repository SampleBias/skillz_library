fn greet(name: &str) -> String {
    format!("hello {name}")
}

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("--help") => println!("usage: greet NAME"),
        Some(name) => println!("{}", greet(name)),
        None => {
            eprintln!("missing NAME");
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn greeting_contains_name() {
        assert_eq!(super::greet("Ada"), "hello Ada");
    }
}
