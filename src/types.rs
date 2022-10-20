// Primative types in Rust--
//  Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (nymber of bits they take in memory)
//  Floats: f32, f64
//  Boolean (bool)
//  Characters (char)
//  Tuples
//  Arrays
pub fn run() {
    // Default is "i32"
    let x: i32 = 1; 

    // Defualt is f64
    let y: f64 = 2.5;

    // Add explicit type 
    let z: i64 = 45454545454; 


    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i32::MAX);

    // Boolean 
    let is_active: bool = true;
    // Get boolean from expression 
    let is_greater: bool = 10 < 2;
    // Works great for unicode as well =)
    let face = '\u{1F604}';


    println!("{:?}", (x, y, z, is_active, is_greater, face));

}