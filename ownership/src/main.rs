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


fn example_dos() {
    
    // integer version_
    let _x:u32 = 5;
    let _y:u32 = _x;

    // string version
    let s1: String = String::from("hello");
    let s2: String = s1;

    //println!("{}, world",s1); // rust considers the original variable no longer valid


    println!("{}, is the variable that survived the move!",s2);

}

fn example_tres() {
    let mut _s: String = String::from("Ayyyeeeee");
    _s = String::from("Ohhhhh");

    println!("{_s}, world!");
}

fn example_cuatro() {
    let s1:String = String::from("what up what up");
    let s2:String = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
}

fn takes_ownership(some_string: String) {
    println!("takes_ownership func has ownership of {some_string}");
    
}// here some_string goes out of scope and `drop` is called. The backing
// memory is freed;

fn makes_copy(some_integer: i32) {
    println!("makes_copy function is making a copy of {some_integer}");
} // here, some_integer goes out of scope but nothing special happens because it's an integer

fn gives_ownership() -> String { // will move its return values into the function that calls it
    let some_string: String = String::from("yours");
    return some_string; // can also just do some_string
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    // a_string is returned and moves out to the calling function
    a_string
}

fn calc_lens(_s: String) -> (String, usize) {
    let length: usize = _s.len();

    (_s,length)
}


fn main() {
    // ex_uno();
    // example_dos();
    // example_tres();
    // example_cuatro();

    let stringy:String = String::from("I'm a little string");
    takes_ownership(stringy);

    let x_wing:i32= 5;
    makes_copy(x_wing);

    // println!("{stringy}"); doesn't work because stringy is out of scope now :(

    println!("{x_wing} is still winging because it was an integer");


    let s1: String = gives_ownership();
    let s2: String = String::from("hello");
    let s3: String = takes_and_gives_back(s2);

    println!("var s1 was given ownership --> val: {s1}");
    //println!("var s2 gave it's ownership away :( {s2}");
    println!("var s3:{s3} was given ownership by a series of steps:\ns2 gave it's ownership to a_string in the func `move` . \nThen a_string gave it's ownership to s3 via a `move`!");


    let (s2,len ): (String,usize) = calc_lens("Hiii".to_string()); // that's cool that I can use .to_string so that I don't have to type out a whole variable

    println!("The length of '{s2}' is {len}.");
    
}


