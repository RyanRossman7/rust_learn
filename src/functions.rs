// Function is used to store blocks of code for future re-use


//Basic example of a function called greeting
// Functions are called in the pfr
pub fn run(){
    greeting("Sup", "Ry");

    // We can bind function values to variables
    let get_sum= add(4, 3, 6);
    println!("Sum: {}", get_sum);

    // Closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;

    print!("C Sum: {}", add_nums( 3, 2));
}







// Define the new function "greeting" with variable types and names
fn greeting(greet: &str, name: &str){
    println!("{} {}, nice to meet you", greet, name)
}

// Create a function that adds three values
fn add(n1: i32, n2: i32, n3:i32) -> i32 {
    n1 + n2 + n3
}