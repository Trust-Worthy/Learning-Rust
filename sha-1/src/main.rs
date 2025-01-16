mod sha1;

use sha1::Sha1; /// I want to use the Sha1 datatype from the sha1 module

fn main() {
    let hasher: Sha1 = Sha1::new(); // new returns an instance of sha
    hasher.hash("TESTING");
}

