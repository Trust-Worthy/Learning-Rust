//
// Project Name: Loops
// File: src/main.rs
// Author: Trust-Worthy
// 
// Description: Learning about the different types of loops in rust


fn main() {
    //loop_loop();
    using_break();
    multiple_loops();
    loop_collection();
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

/// If there are loops within loops, break and continue apply to the innermost loop at that point.
/// You can also specify a loop label on a loop and then use a break or continue to specify that those keywords apply to the labeled loop.
/// Loop labels must begin with a single quote

fn multiple_loops() {

    let mut count: i32 = 0;

    'counting_up: loop {
        println!("count = {count} ");
        let mut remaining:i32 = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up; // ok I freaking love this feature because somethimes I get confused and don't know how to break properly
            }

            remaining -=1;
        }
        count += 1;
    }

    println!("End count = {count}");

}


/// Looping through a collection
fn loop_collection() {
    let a: [i32;5] = [10,20,30,40,50];

    let mut index:usize = 0; // usize is kinda like size_t in C. However, usize is way more memory safe
    // usize is an unsigned integer type 
    // it is used for indexing collections and memory addresses
    // on 64 bit systems usize 64 bit wide
    // can also be used for pointer arithmetic

    while index < 5 {
        println!("the value is: {}", a[index]); // similar to printf format
        
        index +=1;
    }
    // Ok there are some issues here
    // 1. The code above could cause the the program to panic if the index value or test condition is incorrect.
    // 2. If you changed the a array to have four elements but forgot to change the while condition to index < 4, 
    // the code would panic
    // This also makes the code slow because the compiler adds runtime code to perform the conditional check of whether the index is within the bounds of the array on every iteration!
    // Which is very safe but makes it slow.
    // Solution.... 

    for element in a {
        println!("The value is: {element}");
    }

    // using the for loopo means I wouldn't have to remember to change any other code
    // for loops are more safe and concise in rust

    // ex of how to do the countdown example on line 81 with a for loop
    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("LIFTOFF!!!");
}



