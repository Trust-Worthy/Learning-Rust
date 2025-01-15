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
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x is the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

}
