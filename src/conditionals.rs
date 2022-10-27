// Conditionals are used to check the condition for factors of truth
// <
// >
// <=
// >=
// !=
// ==
// Logical opperators (AND, OR, NOT)
// &&(AND), ||(OR), !(NOT)
pub fn run() {
    let age: i32 = 15; 
    let check_id = true; 
    let knows_person_of_age = true; 

    
    // If/Else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink"); 
    }
    else if age < 21 && check_id{
        println!("Bartender: Sorry you have to leave")
    }
    else {
        println!("Bartender: Ill need to see your ID");
    }

    // Shorthand IF
    let is_of_age = if age >= 21 {true} else {false};
    println!("Is of age: {}", is_of_age)
    
}