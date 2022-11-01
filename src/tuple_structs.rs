
// Tuple Structs are used to group values or parameters of different or similar types
#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32 
}
// Impementations in Rust are similar to using methods in other programming languages.
// Using impementations you can create cleaner code by using the dot method.
// *** METHODS GET PASSED SELF /// Associated functions do not ***
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
// Creating an associated functions
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { 
            width: size,
            height: size
        }
    }
}
// **MAIN
pub fn run() { 
    // Lets create Rectangle(rect)
    let rect = Rectangle { 
        width:120, 
        height:200,
    }; 
    // Rectangle_1
    let rect1 = Rectangle {
        width:60, 
        height:100, 
    }; 
    // Rectangle_2
    let rect2 = Rectangle {
        width:145,
        height:230, 
    }; 

    // Rectangle_3
    let rect3 = Rectangle::square(25);

    // PRINT LINES 
    println!("rect can hold rect1: {}", rect.can_hold(&rect1)); 
    
    println!("rect can hold rect2: {}", rect.can_hold(&rect2)); 

    println!("Rect: {:?}", rect);
   
    println!("Rect3: {:?}", rect3); 
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
} 
