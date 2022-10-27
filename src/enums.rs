// Enums are types that have definite values
enum Movement { 
    // Variants 
    Up,
    Down, 
    Left, 
    Right, 
}
fn move_self(m: Movement) {
    // Perform action depending on info 
    match m {
        // IF the Movement ::(==)pitch_up =>(THEN)
        Movement::Up => println!("Up"), 
        Movement::Down => println!("Down"), 
        Movement::Left => println!("Left"), 
        Movement::Right => println!("Right") 
    }
}

pub fn run() {
    let self1 = Movement::Up;
    let self2 = Movement::Down; 
    let self3 = Movement::Left; 
    let self4 = Movement::Right; 
    
    move_self(self1);
    move_self(self2); 
    move_self(self3); 
    move_self(self4);  
}