// Enums are types that have definite values
// They allow us to enumerate a list of variants
// Multiple variations on a type
enum IpAddrKind {
    V4(u8, u8, u8, u8), 
    V6(String), 
}
enum Message {
    Quit, 
    Move {x: i32, y: i32}, 
    Write(String), 
    ChangeColor(i32, i32, i32),  
}
impl Message {
    fn some_function() {
        println!("function!", some_function);
    }
}
// A Structure used in Enums
struct IpAddr {
    kind: IpAddrKind, 
    address: String, 
}

//MAIN
pub fn run() {
    // A better way to handle it
    let localhost = IpAddrKind::V4(127, 1, 1, 1);
}
