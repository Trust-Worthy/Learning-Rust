use std::env;
use std::fs;






fn parse_configs(args: &[String]) -> (&str,&str) {

    let query: &String = &args[1]; // String to find in the file
    let file_path: &String = &args[2]; // path to the file to be searched.

    return (query,file_path)
}






fn main() {
    

    let args: Vec<String> = env::args().collect();

    let (query,file_path) = parse_configs(&args);


    println!("Searching for {query}");
    println!("In file {file_path}");


    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file.");

    println!("With text: \n{contents}");


}
