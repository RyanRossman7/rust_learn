// Enums are types that have definite values
// Multiple variations on a type
enum Pitch { 
    Up, Down, Left, Right}
enum Roll {
    Up, Down, Left, Right}
enum Yaw {
    Left,Right }
enum Rpm { 
    Max,Med,Min,Hover,R_Min,R_Med,R_Max
}
fn Movement () {
    // Perform action depending on info 
    match P {
        // IF the Movement ::(==)pitch_up =>(THEN)
        Pitch::Up => println!("Pitch Up"), 
        Pitch::Down => println!("Pitch Down"), 
        Pitch::Left => println!("Pitch Left"), 
        Pitch::Right => println!("Pitch Right")}
    
    match R {
        Roll::Up => println!("Roll UP"), 
        Roll::Down => println!("Roll Down"), 
        Roll::Left => println!("Roll Left"), 
        Roll::Right => println!("Roll Right")}
    
    match Y {
        Yaw::Left => println!("Yaw Left"),
        Yaw::Right => println!("Yaw Right")}

    match R {
        Rpm::Max => println!("Throttle Minimum"),
        Rpm::Med => println!("Throttle Minimum"),
        Rpm::Min => println!("Throttle Minimum"),
        Rpm::Hover => println!("Throttle Minimum"),
        Rpm::Reverse_Min => println!("Throttle Minimum"), 
        Rpm::Reverse_Med => println!("Throttle Minimum"),  
        Rpm::Reverse_Max => println!("Throttle Minimum")}
}


pub fn run() {
    let p_up = Pitch::Up;
    let p_down = Pitch::Down; 
    let p_left = Pitch::Left;
    let p_right = Pitch::Right; 
}
