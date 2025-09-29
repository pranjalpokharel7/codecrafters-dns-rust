use std::env;

pub struct Args {
    pub resolver: Option<String>,
}

impl Args {
    pub fn init() -> Self {
        let args: Vec<String> = env::args().collect();

        if args.len() == 1 {
            return Self { resolver: None };
        }

        if args.len() != 3 {
            eprintln!("Usage: {} --resolver <address>", args[0]);
            std::process::exit(1);
        }

        if args[1] != "--resolver" {
            eprintln!("Unknown option: {}", args[1]);
            eprintln!("Usage: {} --resolver <address>", args[0]);
            std::process::exit(1);
        }

        if args[2].is_empty() {
            eprintln!("Error: resolver address cannot be empty");
            std::process::exit(1);
        }

        Self {
            resolver: Some(args[2].clone()),
        }
    }
}
