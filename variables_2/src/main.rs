fn main() {
    

    let mut x:u8 = 5;

    println!("The value of x is: {x}");

    {
        let x: u8 = x * 2;

        println!("The value of x in the inner scope is {x}");
    }

    x = 6;
    println!("The value of x is: {x} ");

}
