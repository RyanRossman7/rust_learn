// Command Line Arguments 
// Lets make a GREP like CLI tool  
use std::env;
use std::fs; //fs is file system
use std::process;
use std::error::Error; 
// We create an args function that is a vector of strings.
// Call the args method on the env module & then collect it. 
// ** Collection needs to know what type we want (Vec<String>).
pub fn run() {
    let args: Vec<String> = env::args().collect(); 

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguements: {}" , err);
        process::exit(1); 
    }); 

    println!("Searching for {}", config.input);
    println!("In file {}", config.file);

    run(config);
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file)
        .expect("error");

    println!("With txt:\n{}", contents); 

    Ok(())
}
struct Config {
    input: String, 
    file: String, 
}
impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        // handle errors gracefully young grasshopper
        if args.len() < 3 {
            return Err("Not enough arguments")
        }
        let input = args[1].clone();
        let file = args[2].clone();
        // Intitializing the structre Config
        Ok(Config { input, file})
    }
}
// cargo run the cli.txt