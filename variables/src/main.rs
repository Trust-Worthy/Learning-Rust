/**
 * @Author Trust-Worthy
 * Topic: Rust variables and constants
 */

/// constant declaration example
/// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


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
    
}
