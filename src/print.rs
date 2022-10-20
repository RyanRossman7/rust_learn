pub fn run() {
    // Print to console
    println! ("Hello from the print.rs file");
   
    // Basic Formatting
    println!("{} is from  {}", "Ryan", "Nashville");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
         "Ryan", "Nashville", "Code"
    );

    // Named Arguments
    println!(
        "{name} likes to {verb} {activity}",
        name = "Ryan",
        verb = "play", 
        activity = "Baseball"
    );

    // Placeholder traits
    println!(
        "Binary: {:b} Hex: {:x} Octal {:o}", 
         10, 10, 10);
}
