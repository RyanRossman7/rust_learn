use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect(); 
    println!("{}", args); 
    let command = args[1].clone(); 
    let name = "Bud"; 

    // print
    if command == "hello" {
        println!("Hi there {}", name ); 
    }
}