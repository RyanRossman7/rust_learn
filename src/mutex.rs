// Shared state concurrency
// Mutex = mutual exclusion
// only one thread can access this data 
// A lock is a data structure that keeps track of which
//       thread has exclusive access to a piece of data
// RULE: 1 (Must acquire lock before gaining access to data)
// RULE: 2 (Must release lock after so other processes can use the thread)

use std::sync::{Arc, Mutex}; 
use std::thread; 

pub fn run() {
  let counter = Arc::new(Mutex::new(0)); 
  let mut handles = vec![]; 

  for _ in 0..10 { 
    let counter = Arc::clone(&counter); 
    let handle = thread::spawn(move || { 
        let mut num = counter.lock().unwrap(); 

        *num += 1; 
    }); 

    handles.push(handle); 
  }

  for handle in handles {
    handle.join().unwrap(); 
  }

  println!("Result: {}", counter.lock().unwrap()); 

} 