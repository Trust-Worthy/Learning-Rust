/**
 * @Author Trust-Worthy
 * Topic: Rust variables and constants
 */

/// constant declaration example
/// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn floats() {
    let x = 2.0; // f64 --> 64 bit float
    let y: f32 = 3.0 //f32 --> 32 bit float

}

fn chars() {
    /**
     * rust's char type is four bytes in size and represents a unicode scalar value
     * this means it can represent a lot more than just ASCII
     * 
     */

    let c = 'z';
    let z:char = 'b'; // with explicit type annotation --> I love type annotations
}

fn compound_type() {
    /**
     * Compount types can group multiple values into one type!
     */

    // tuples are a compount type
    let tup: (i32,f64,u8) = (500,6.4,1);

    // use pattern matching to destructure a tuple
    let (x,y,z) = tup;

    println!("The value of y is: {y}");

    // deconstructing tuple usign period notation and indexing

    let x: (i32,f64,u8) = (500,6.4,1);

    let five_hunned: i32 = x.0;
    let six_point_four: f64 = x.1;
    let one: u8 = x.2



}

fn array_type() {
    
}

fn main() {
    
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    let x = 5;
    let x = x + 1; /// example of variable shadowing

    {
        let x = x * 2;
        println!("The value of x is the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    /// This is why shadowing is useful --> you can change the type of the value but use the same variable name
    let spaces = "   ";
    let spaces = spaces.len();
    println!("There are {spaces}!");

    /**
     * Rust has 4 primary scalar types:
     * 1. integers
     * 2. floats
     * 3. bools
     * 4. chars
     */

    // use u for unsigned integer and i for signed integer
    // tip: each signed variant can store numbers from -(2^n-1) --> 2^n-1 -1 inclusive where n is the number of bits
    // tip: unsigned variants can store numbers from 2^n-1

}
