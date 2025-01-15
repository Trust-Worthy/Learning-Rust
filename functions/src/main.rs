///
/// @Author Trust-Worthy
/// 
/// Topic: Learning about functions in rust
/// # Examples are below
/// Statements are instructions that perform some action and do not return a value
/// Expressions evaluate to a resultatnt value. Kinda like in a math.
/// In math, you have statements and expressions. A statement becomes an expressions when you include the equals sign.

/*
    let y = 6;

    The statement above does not return a value. This is different from C. In C, the assignment returns the value of the assignment.

    C example:
    int y;
    int x = y = 6; -->  ** you can't do this in rust.**
 */



fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measuremeht(5, 'h');
    expression_ex();
}

/// You have to always put type annotatiosn in function definitions
fn another_function(x:i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measuremeht(value: i32, unit_label:char) {
    println!("The measurement is: {value}{unit_label}");
}

fn expression_ex() {
    // The syntax below reminds me a lot of closures in Swift!
    let y = {
        let x = 3;
        x + 1 // expressions do not include ending semicolons
        // if you add a semicolon to the end of an expression you turn it into a statement and it won't return a value
    };

    println!("The value of y is: {y}");
}
