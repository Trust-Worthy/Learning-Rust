



fn calculate_len(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String){ // change the type annotation to a mutable reference

    some_string.push_str(", the world is yours");

 }

fn dangle() -> String {
    let s = String::from("hello");

    return s // returning a refernce to a variable that's going to go out of scope to try and create a dangling pointer!c
}
fn main() {
    
    // Example of providing a reference to a String value. The reference is like a pointer to data that OWNED by a different variable
    // Unlike a pointer, the reference is guaranteed to point to a valid value of a particular type

    let mut s1: String = String::from("hello twin");

    let len: usize = calculate_len(&s1); // The & means BORROW the value at this address instead of moving or transferring ownership

    println!("The length of '{s1}' is {len}."); // I am still able to use s1 because it's value was borrowed instead of ownership being transferred and then it going out of scope.


    // Just as variables are immutable by default, so are references. Not allowed to modify something my code only has a reference to.
    change(&mut s1);

    // HUGE RULE: if I have a mutable reference to a value, I can have NO OTHER references to that value

    let mut s: String = String::from("he he ha");

    {
        let r1: &mut String = &mut s;
    } // brackets allow for multiple mutable references! just not simultaneous ones!!!

    let r2: &mut String = &mut s; // This is a no-no. Have to wait for r1's mutable reference go out of scope.

    // println!("{}, {}", r1, r2); // but now r1 is out of scope :(
    

    // We also can't have a mutable reference while having an immutable reference at the same time!
    let mut ex_2: String = String::from("Please reference me!");
    
    let r1: &String = &ex_2;
    let r2: &String = &ex_2;
    println!("{}, {}", r1, r2);

    let r3: &mut String = &mut ex_2; // Big problemo ! IF line 47 didn't exist.

    let ref_to_nothing = dangle(); // Trying to create a dangling pointer. Or a pointer that is pointing to invalid memory.



    




}
