use std::io;

/**
 * @Author Trust-Worthy
 * Topic: Rust variables and constants
 */

/// constant declaration example
/// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn floats() {
    let x = 2.0; // f64 --> 64 bit float
    let y: f32 = 3.0; //f32 --> 32 bit float

}

fn chars() {
    /*
     * rust's char type is four bytes in size and represents a unicode scalar value
     * this means it can represent a lot more than just ASCII
     * 
     */

    let c = 'z';
    let z:char = 'b'; // with explicit type annotation --> I love type annotations
}

fn compound_type() {
    /*
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
    let one: u8 = x.2;

}

fn array_type() {
    /*
     * Every element of an array must have the same type unlike tuples.
     * Arrays have a fixed length
     * Arrays are allocated on the stack
     */

    // you write type annotations for arrays using brackets, type of each element, and a semicolon, and then the number of elemetns in the array
    let a: [i32;5]= [1,2,3,4,5];
    
    // indexing
    let first: i32 = a[0];
    let second: i32 = a[1];

    
    // you can initialize an array to contain the same value for each ele be specifying the initial value, semicolon and length
    // below is a array of 5 3's
    let same_ele: [i16;5] = [3;5];
}

fn var_type() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    let x = 5;
    let x = x + 1; // example of variable shadowing

    {
        let x = x * 2;
        println!("The value of x is the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // This is why shadowing is useful --> you can change the type of the value but use the same variable name
    let spaces = "   ";
    let spaces = spaces.len();
    println!("There are {spaces}!");
}

fn invalid_arr_access () {

    let a: [i32;5] = [1,2,3,4,5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index);

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    /*
    Pay attention!

    The program will result in a runtime error at the point of using an invalid value in the indexing operation.

    The program will exit with an error message and won't execute the final println! statement.

    When you attempt to access an element using indexing, Rust will CHECK THAT THE INDEX SPECIFIED is less than the array length. If the index is greater than or equal to array len, rust will panic.

    This check has to happen at runtime, especially because compiler can't konw what value a user will enter when they run the code later.

    This is an example of Rusts's memory safety principles in action! In many low-level languages, this kind of check is nont done. Usually when you provide an incorrect index, invalid memory can be accessed.

    ** Rust protects against this kind of error by immediately exiting instead of allowing the memory access and continuing! ** 
     */

    
}

fn main() {
    /*
     * Rust has 4 primary scalar types:
     * 1. integers
     * 2. floats
     * 3. bools
     * 4. chars
     */

    // use u for unsigned integer and i for signed integer
    // tip: each signed variant can store numbers from -(2^n-1) --> 2^n-1 -1 inclusive where n is the number of bits
    // tip: unsigned variants can store numbers from 2^n-1

    // var_type();
    // floats();
    // chars();
    // compound_type();
    invalid_arr_access();
}
