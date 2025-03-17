





fn main() {

    /*
    Here’s a small programming problem: 
    write a function that takes a string of words separated by spaces and returns the first word it finds in that string. 
    If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.
     */
    let mut s: String = String::from("hello twin");
    
    let word: usize = first_word(&s); // passing in a reference 

    s.clear();

    // word is now invalid because s is an empty string!

    
}


fn first_word(s: &String) -> usize {

    let bytes: &[u8] = s.as_bytes(); // reference to an array of unsigned 8 bit integers

    for (i, &item) in bytes.iter().enumerate() {

        // i is the index and &item is a reference to that index! Brilliant!!! 
        // this is by specifying a pattern for the tuple. I'll learn more about that in future lessons
        if item == b' ' {
            return i;
        }
    }

    s.len()

}

