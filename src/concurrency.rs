use std::{thread, time::Duration}; 

pub fn run() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hello number {} from the spawned thread", i); 
            thread::sleep(Durration::from_millis(1)); 
        }
    }); 

    for i in 1..5 { 
        println!("hello number {} from the spawned thread", i); 
        thread::sleep(Durration::from_millis(1)); 
    }

}