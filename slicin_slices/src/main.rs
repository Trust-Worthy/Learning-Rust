





fn main() {

    /*
    Here’s a small programming problem: 
    write a function that takes a string of words separated by spaces and returns the first word it finds in that string. 
    If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.
     */


    
}


fn first_word(s: &String) -> usize {

    let bytes: &[u8] = s.as_bytes(); // reference to an array of unsigned 8 bit integers

    for (i, &item) in bytes.iter().enumerate() {

        // i is the index and &item is a reference to that index! Brilliant!!!  
        if item == b' ' {
            return i;
        }
    }

    s.len()

}

