struct Person {
    first_name: String, 
    last_name:  String
}
impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(), 
            last_name: last.to_string()
        }
    }
    // Get full name 
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    // Set New Name
    fn set_new_name(&mut self, last:&str) {
        self.last_name = last.to_string(); 

    }
}
pub fn run() {
    let mut p = Person::new("Ryan", "Rossman"); 
    println!("Person_New: {}", p.full_name());
    p.set_new_name("RoSsMAn");
    println!("Person {}", p.full_name());

}