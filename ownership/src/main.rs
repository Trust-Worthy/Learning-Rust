//
// Project Name: Ownership
// File: ownership/main.rs
// Author: Trust-Worthy
// Description: This is a file to learn about how ownership works in rust



/// Looking at the scope of variables in this function
fn ex_uno() {
    // when s comes into scope it is valid
    // when s is out of scope it's not
    let _s:&str = "hello";

    // create a string literal using from function
    let mut s2: String = String::from("hello"); // this is an example of that static method ting!
    // * must put the mut keyword!!!
    s2.push_str(", world!"); //  appends a literal to a String

    // String can be mutated by examples like s on line 13 cannot be mutated becaues of how these types deal with memory.
    // My hypothesis is that string literals are placed on the stack

    println!("{s2}");
}











fn main() {
    ex_uno();
}

