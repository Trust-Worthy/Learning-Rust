mod sha1;

use sha1::Sha1; /// I want to use the Sha1 datatype from the sha1 module
use std::io;
fn main() {
    let mut hasher: Sha1 = Sha1::new(); // new returns an instance of sha

    let mut user_input: String = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to take user input");

    
    let res: [u8;20] = hasher.hash(&user_input);
    println!(" byte version {:?}",res);

         // Convert to a hexadecimal string for human readability
    let hex_string = res.iter().map(|b| format!("{:02x}", b)).collect::<String>();
    println!("Hexadecimal String: {}", hex_string);
        



}

