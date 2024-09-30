use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().into_iter().collect();
    if args.len() != 2 {
        panic!("params must equal 2!");
    }

    let (query, file_path) = (&args[1], &args[2]);

    let file_content = fs::read_to_string(file_path).unwrap();
    println!("query: {}", query);
    println!("{}", file_content);
}
