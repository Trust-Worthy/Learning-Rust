use std::env;
use std::fs;





struct  Config {
    query: String,
    file_path: String,
}

impl Config {

    fn new (args: &[String]) -> Config  { // I'm really loving this tutorial. It's teaching me how to write code in a rust-like way!
        
    
        let query:String = args[1].clone(); // String to find in the file
        let file_path:String = args[2].clone(); // path to the file to be searched.
    
        return Config {query, file_path}
    }
}






fn main() {
    

    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args);


    println!("Searching for {}",config.query);
    println!("In file {}",config.file_path);


    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file.");

    println!("With text: \n{contents}");


}
