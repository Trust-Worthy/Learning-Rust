use std::env;
use std::fs;
use std::process;



struct  Config {
    query: String,
    file_path: String,
}

impl Config {

    fn build (args: &[String]) -> Result<Config, &'static str> { // I'm really loving this tutorial. It's teaching me how to write code in a rust-like way!
        
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query:String = args[1].clone(); // String to find in the file
        let file_path:String = args[2].clone(); // path to the file to be searched.
    
        Ok(Config { query, file_path })
    }
}

fn run(config: Config) {


    let contents = fs::read_to_string(config.file_path)
    .expect("Should have been able to read the file.");

    println!("With text: \n{contents}");

}

fn main() {
    

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {err}");
            process::exit(1);
        });
    

    run(config);

   


}
