// Import std::io for input/output
use std::io; 

pub fn run(){
    // dec
    let mut input = String::new(); 

    io::stdin().read_line(&mut input).expect("failed to read line"); 
    println!("input = {}", input);

}