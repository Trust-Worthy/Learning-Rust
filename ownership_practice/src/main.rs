



fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_int: i32) {
    println!("{some_int}");
}




fn main() {
    // let s: &str = "hello"; // string literal

    // println!("String literal {s}");

    // let mut heap_str: String = String::from("hello bro");

    // heap_str.push_str("what are you up to?");



    // let s1: String = String::from("hiiiii");

    // let s2: String = s1;

    // // println!("s1 is out of scope ! {s1}");
    // println!("s2 is a move of s1! {s2}");


    // // how to create a deep copy!

    // let s3: String = s2.clone();
    // println!("s3 has been officially cloned! clone: {s3}");

    
    // Reviewing & Refreshing on Chapter 4 of the Rust book
    let mut s: String = String::from("hello");

    s.push_str(", world");


    let s1: String = String::from("what up twin!");

    let s2 = s1;

    
    // Passing a variable to a function with either: MOVE or COPY
    takes_ownership(s);
    println!("{}",s);

    let x: i32 = 5;

    makes_copy(x); // i32 implements the Copy Trait so x does NOT move into the function
                            // therefor it's ok to use afterward

    println!("{}",x);




}
