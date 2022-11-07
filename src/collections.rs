pub fn run () {
    let a= [1, 2, 3];
    let mut a:Vec<i32> = Vec::new();
    a.push(1);
    a.push(2); 
    a.push(3);
    // Short hand vectors can be made using the vec! exlimation
    // Due to Rusts inference it can determine Vec size
    //vectors will be dropped when out of scope
    {
        let b = vec![4, 5, 6];
        println!("Scoped Vector is {:?}", b);
    }
    // v2 is dropped becuase it is now out of scope
    println!("first Vector is {:?}", a);
    // Now lets access elements inside of a vector by creating the vector
    let c = vec![7, 8, 9, 10, 11];
    // And then assigning a variable to a REFERENCE in our vector we can identify in brackets with the 3rd value using the 0 scale
    let third = &c[3];
    println!("Third Vector is {:?}", c);
    println!("The third element vector 3 is {:?}", third);
    

    // lets handle index out of bounds errors gracefully by using the match function
    match c.get(88 ) {
        Some(third) => println!("The third element is {}", third),
        None => print!("There is no element here "),
    }

    // We can also iterate over elements by using a for loop
    for n in &c {
        println!(r#"The elements in vector 3 are"#, c);
    }



}

