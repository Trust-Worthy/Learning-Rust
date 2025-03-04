fn main() {
    let s: &str = "hello"; // string literal

    println!("String literal {s}");

    let mut heap_str: String = String::from("hello bro");

    heap_str.push_str("what are you up to?");



    let s1: String = String::from("hiiiii");

    let s2: String = s1;

    // println!("s1 is out of scope ! {s1}");
    println!("s2 is a move of s1! {s2}");






}
