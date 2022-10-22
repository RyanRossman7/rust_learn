
// Tuple Struct
struct Color(u8, u8, u8);

pub fn run() {
    let mut c = Color(255, 24, 6); 

    c.2 = 9;
    println!("Color: {}, {}, {}", c.0, c.1, c.2); 
}