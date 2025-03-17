



fn calculate_len(s: &String) -> usize {
    s.len()
}

fn main() {
    
    // Example of providing a reference to a String value. The reference is like a pointer to data that OWNED by a different variable
    // Unlike a pointer, the reference is guaranteed to point to a valid value of a particular type

    let s1: String = String::from("hello twin");

    let len: usize = calculate_len(&s1); // The & means BORROW the value at this address instead of moving or transferring ownership

    println!("The length of '{s1}' is {len}."); // I am still able to use s1 because it's value was borrowed instead of ownership being transferred and then it going out of scope.

    
}
