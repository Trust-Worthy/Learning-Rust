fn main() {
    

    let x:u8 = 5;

    println!("The value of x is: {x}");

    {
        let x: u8 = x * 2;

        println!("The value of x in the inner scope is {x}");
    }

    let x = 6;
    println!("The value of x is: {x} ");    

    // Example of a tuple:
    let tup: (i32,f64,u8) = (500,6.4,1); /// I think the type annotations for a tuple are cool
    let (x,y,z) = tup;

    println!("The value of y is {y}");





}
