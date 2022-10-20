// Loops are used to iterate until a conditional is met

pub fn run() {
    let mut count = 0; 

// Infinite Loop
    loop {
        count += 1;
        println!("Number: {}", count); 

        if count == 20 {
            break;
        }
    }
    
    // While loop (FizzBuzz)
    while count <= 100{ 
        if count % 15 == 0{ 
            println!("FizzBuzz");
        } else if count % 10 == 0{
            println!("Buzz")
        } else if count % 5 == 0{
            println!("Fizz")
        } else {
            println!("{}", count)
        }
        // increment 
        count += 1;
    }      

    // For Range 
    for x in 0..100 {
        if x % 15 == 0{ 
            println!("FizzBuzz");
        } else if x % 10 == 0{
            println!("Buzz")
        } else if x % 5 == 0{
            println!("Fizz")
        } else {
            println!("{}", x)
        }
    }
}

