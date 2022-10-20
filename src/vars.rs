// Variables hold primative data or references to data
// Variables are immutable by defualt
// Rust is a block scoped language

pub fn run() {
    let name = "Ryan";
    let mut age = 31;
    println!("My name is {} and I am {}", name, age);
    age = 32;
    println!("My name is {} and I am {}", name, age);
    age = 33;
    println!("My name is {} and I am finally {}", name, age);

    // Define constant
    const ID: i32 = 001; 
    println!("ID: {}", ID);
}