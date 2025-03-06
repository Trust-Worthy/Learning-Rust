use std::env;





fn main() {
    

    let args: Vec<String> = env::args().collect();

    let query: &String = &args[1]; // String to find in the file
    let file_path: &String = &args[2]; // path to the file to be searched.

    println!("Searching for {query}");
    println!("In file {file_path}");

    
}
