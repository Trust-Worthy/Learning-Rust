mod sha1;

use sha1::Sha1; /// I want to use the Sha1 datatype from the sha1 module

fn main() {
    let mut hasher: Sha1 = Sha1::new(); // new returns an instance of sha
        let res: [u8;20] = hasher.hash("TESTING");
        println!(" byte version {:?}",res);

         // Convert to a hexadecimal string for human readability
    let hex_string = res.iter().map(|b| format!("{:02x}", b)).collect::<String>();
    println!("Hexadecimal String: {}", hex_string);
        



}

