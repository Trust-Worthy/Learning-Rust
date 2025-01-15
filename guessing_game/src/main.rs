//
// Project Name: Guessing Game
// File: src/main.rs
// Author: Trust-Worthy
// Description: guessing game program that will ask for user input, process that input, and check to see if the user guessed correctly


use std::io; // this is the std library.
use rand::Rng;



fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // start..=end this is inclusive 
    println!("The secret number is: {secret_number}");

    println!("Please input your guess: ");

    let mut guess = String::new(); // equals sign implies binding
    // String::new is a function that returns a new instance of a String.
    // String is a string type provided by the standard library that is growable

    //::new indicates that new is an associated function of the String type.
    // an associated functin is a function that's implemented on a type.
    // the new function is found on many types because it's a common name

    // ** Got it ** Rust associated functions are very similar to Swift static methods
    // Swift --> TypeName.staticMethod()
    // Rust --> TypeName::associated_function()
    // Cruciall both static methods and associated functions are defeined at 
    // THE TYPE LEVEL not the instance level. 
    // You call static methods nad associated functions on THE TYPE ITSELF, not on the instance.

    io::stdin() // --> type that represents a handle to the std input for my terminal
        .read_line(&mut guess)
        // ^^^ like variables, references are immutable by default
        // so you have to write &mut guess rather than &guess to make it mutable 
        .expect("Failed to read line");
        // read_line returns a Result value used for error handling
        // read_line returs either OK or Err (enum vals)
        // Result has a few methods you can call aka .expect so you can display the error message you want to display


    println!("You guessed: {}",guess);
}
