use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[args.len()-1].clone();
    
    parse_file(filename);
}

fn parse_file(filename: String) {
    let content = fs::read_to_string(filename).expect("Could not read file");
    
    println!("{}", content);
}
