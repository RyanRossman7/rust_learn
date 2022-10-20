// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run(){
    let mut hello = String::from("Hello "); 

    // Get length
    println!("length: {}", hello.len());

    // Push Char
    hello.push('W');

    // Push String (_str)
    hello.push_str("orld"); 

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check if empty
    println!("Is Empty: {}", hello.is_empty());

    // contains
    println!("Contains 'World' {}", hello.contains("World"));

    // Replace 
    println!("Replace: {}", hello.replace("World", "Rust")); 

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word)
    }

    // Create a string with capcity 
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b'); 

    // Asserion testing
    assert_eq!(2, s.len());
    assert_eq!(11, s.capacity());

    println!("{}", s); 



}