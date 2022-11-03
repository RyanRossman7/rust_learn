// Command Line Arguments 
use std::env;

pub fn run() {
    // We create an args function that is a vector of strings.
    // Call the args method on the env module & then collect it. 
    // ** Collection needs to know what type we want (Vec<String>).
    let args: Vec<String> = env::args().collect(); 
    println!("{:?}", args); 

    let query = &args[1];
    let filename = &args[2];
    
  
}