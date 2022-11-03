// Object Oriented Programming: OOP for short has competeing definitions throughout programming languages
// By some definitions, Rust is an OOP Language. 
// Lets see how rust can be object like whilst still maintining its oxidation  
// Objects, Encapsulation & Inherritance 
// Objects are made of data and methods that operate on that data **Use impl**
// Encapsulation (**Hidden within the object**)

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64, 
}
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop(); 
        match result { 
            Some(value) => { 
                self.update_average();
                Some(value)
            }
            None => None, 
        }
    }
    pub fn average(&self) -> f64 { 
        self.average
    }
}
pub fn run() {
    println!("{}", "yep"); 
}

// Inheratance 
// Rust does not have inheratance directly, however, 
// Code sharing and other functional tools such as defualt trait method implementations 
// define definitions and not fields
// polymorphism allows you to substitute multiple objects for eachother if they share similar characteristics



