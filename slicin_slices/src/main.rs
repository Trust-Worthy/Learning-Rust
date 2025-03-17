





fn main() {

    /*
    Here’s a small programming problem: 
    write a function that takes a string of words separated by spaces and returns the first word it finds in that string. 
    If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.
     */
    let mut s: String = String::from("hello twin");
    
    let word: &str = first_word(&s[..]); // passing in a reference 

    // s.clear();

    println!("The first word is: {word}");



    // word is now invalid because s is an empty string!. Word isn't connected to the STATE of s


    // String slices to the rescue
    let string_to_slice: String = String::from("Hello Twin");

    let hello: &str = &string_to_slice[0..5]; // This is exclusive --> slice comprises index 0 - 4.
    let twin: &str = &string_to_slice[6..10];

    
}


fn first_word(s: &str) -> &str { // This function is now more flexible because I can pass either an &str or a reference to a String! Brilliant

    let bytes: &[u8] = s.as_bytes(); // reference to an array of unsigned 8 bit integers

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]

}

