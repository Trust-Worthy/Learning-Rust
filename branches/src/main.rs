//
// Project Name: branches 
// File: src/main.rs
// Author: Trust-Worthy
// Description: Learning about control flow in rust



fn if_expressions() {
    
    let number:i64 = 3;

    if number < 5 { // condition for if must eval to a bool
        // rust will not try to convert non-bools to bools like in javascript or Ruby
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition:bool = true;
    let num2:i32 = if condition {5} else {6};
    // if the value inside the else block were a string, I would get an error because variables must have a single type.
    // the rust compiler needs to know at compile time what type the num2 variable is

    println!("The value of num2 is: {num2}");
}

fn main() {
    if_expressions();
}
