
use std::{fmt::format, io};

// String is implemented as a wrapper around a vector of bytes


pub fn string_practice() {


    let mut stringy = String::new();

    let data = "initial contents";

    let stringy = data.to_string();

    let mut stringy = "this method works on literal strings too which is dope".to_string();

    // strings in Rust are UTF-8 encoded so you can include a lot of different types of text.



    // push str method takes a string slice 
    stringy.push_str("addding to the string!");


    let mut push_it: String = String::from("Jesu");

    push_it.push('s'); // adds a single character

    let more = String::from("is King");

    let combined = push_it + &more; // string concatonation requires at least one owned string
    // a MOVE operationg occurred in the above code
    // push_it was moved into the add() that occured and thus isn't in scope anymore



    // the format macro is def easier

    let s1 = String::from("tic"); let s2 = String::from("tac"); let s3 = String::from("toe");

    let game = format!("{s1}-{s2}-{s3}");

    println!("Game: {game}")





    
}