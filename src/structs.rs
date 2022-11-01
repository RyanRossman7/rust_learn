// Structs are used to create custom data types
// Structs are object attributes in an object oriented programming language. 
use std::any::type_name;
// In this traditional struct, we are creating a new user with 4 attributes.
// This is similar to key value pairs in Python. With no restriction on type.    
struct User {
    username: String, 
    email: String, 
    pin: u64, 
    active: bool, 
}
// Now we can crate an instance of the User struct. 
pub fn run() { 
    let mut user1 = User {
           username: String::from("BigD224"), 
           email: String::from("BD244@Gmail.com"), 
           pin: 1234, 
           active: true, 
    };
    let name = user1.username; 
    let email = user1.email; 
    let pin = user1.pin; 
    let active = user1.active; 
    
    user1.username = String::from("BiggieSmalls");
    
    let user2 = build_user(
        String::from("KingBiggie@Biggie.org"),
        String::from("KingBiggie")
    );

    let user3 = User {
        email: String::from("2Pac@Pac.com"),
        username: String::from("2PAC"),
        ..user2
    }; 
fn build_user(email: String, username: String) -> User {
    User {
        email, 
        username, 
        active: true,
        pin: 2334,
    }
}
