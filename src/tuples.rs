// Tuples group together values of different types
// MAX 12 Elements

pub fn run() {
    let person: (&str, &str, i8) = ("Ryan", "Florida", 31);

    println!("{} is from {} and is {}", person.0, person.1, person.2);

}