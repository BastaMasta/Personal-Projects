use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem passing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Insufficient arguments");
        }
        Ok(
            Self{
                query: args[1].clone(),
                file_path: args[2].clone(),
            }
        )
    }
}
