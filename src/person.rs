// Ser the structure of the person to set up the construct
struct Person {
    first_name: String, 
    last_name:  String
}
// Now we need to IMPLY what we will be defining in the folowing functions
impl Person {
    // lets Cronstruct person named Mike Hancho
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(), 
            last_name: last.to_string()
        }
    }
    // Get full name (-> *Returns* a String)
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    // Set New Name
    fn set_new_name(&mut self, last:&str) -> (String, String) {
        // .to_string is similar to -> from above except it SETS it to a string rather than returns it
        self.last_name = last.to_string(); 

    // Name to tuple 
    fn to_tuple( mut self) -> (String, String) {
        (self.first_name, self.last_name)
    } 
        
    }
}
pub fn run() {
    let mut p = Person::new("Mike", "Hancho"); 
    
    println!("Person_New: {}", p.full_name());
    p.set_new_name("Pancho");
    println!("Person {}", p.full_name());
    println!("Tuple Person: {:?}", p.to_tuple()); 

}