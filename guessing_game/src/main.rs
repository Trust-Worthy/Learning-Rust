//
// Project Name: Guessing Game
// File: src/main.rs
// Author: Trust-Worthy
// Description: guessing game program that will ask for user input, process that input, and check to see if the user guessed correctly


use std::io; // this is the std library.
use rand::Rng; // used for the random number generator
use std::cmp::Ordering; // ordering type is another enum 



fn main() {
    println!("Guess the number!");

    let secret_number:u32 = rand::thread_rng().gen_range(1..=100);
    // start..=end this is inclusive 
    println!("The secret number is: {secret_number}");

    println!("Please input your guess: ");

    let mut guess: String = String::new();
    
    
    // equals sign implies binding
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
    // * Rust is statically typed which I love.


    let guess: u32 = guess.trim().parse().expect("Please type a positive integer!");
    // shadowing guess here by reusing guess var name rather than creating another
    // shadowing is often used when trying to convert a value to another type!
    // trim method refers to the string version of guess removing any whitspace
    // parse method converts a string to another type
    // parse returns a Result type which has to be handled with .expect in case the string isn't a number

    println!("You guessed: {}",guess);

    match guess.cmp(&secret_number) { // cmp method compares two values and can be called on anything that can be compared.
        // initially secret_number is an u32 which can't be compared to a string
        
        // cmp takes a reference to whatever I want to compare with
        // then we use match expression to decide what to do next based on which variant of Ordering was returned from the call to cmp
        Ordering::Less => println!("Tool small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!!!\n!!!!"),
    }
}
