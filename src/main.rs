use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut i = 1;
    for arg in &args {
        println!("{i}.{arg}");
        i = i + 1
    }
    // catch if there is something missing
    if args.len() != 2 + 1 {
        panic!("You need to Enter Query file!");
    }
    let query = &args[1];
    let file_path = &args[2];

    println!("\nQuery: {query} | File: {file_path}\n");

    let content = fs::read_to_string(file_path)
        .unwrap_or_else(|error| panic!("Problem with reading file: '{file_path}' error: {error}"));

    let result = content.find(query).unwrap();
    println!("Result find: {result}");
    println!("With text: \n {content}");
}

fn search_query_in_string(query: String, string: String) -> String {
    return "nothing found".to_string();
}
