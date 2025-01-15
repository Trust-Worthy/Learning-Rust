//
// Project Name: Loops
// File: src/main.rs
// Author: Trust-Worthy
// 
// Description: Learning about the different types of loops in rust


fn main() {
    //loop_loop();
    using_break();
}


fn loop_loop() {
    loop {
        println!("again!");
    }
}

/// Example of using break to break out of the loop keyword 
fn using_break() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}