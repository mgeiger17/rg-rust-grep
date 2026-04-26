use colored::Colorize;
use std::collections::{HashMap, LinkedList};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum SearchResult {
    Matched(),
    NotMatchedBecause(char),
}

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
    let searchable = &args[1];
    let file_path = &args[2];

    println!("\nQuery: {searchable} | File: {file_path}\n");

    let mut map: HashMap<char, usize> = HashMap::new();

    for (i, c) in searchable.chars().rev().enumerate() {
        if map.get(&c) == None {
            map.insert(c, i);
        }
    }

    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for (i, line) in lines.map_while(Result::ok).enumerate() {
            println!("{}. {}", i, line);
            let result = search_searchable_in_string(&searchable, line, &map);
            println!("{}", result)
        }
    } else {
        println!(
            "{} File with path: {} not found or Error reading it",
            "[ERROR]:".red(),
            file_path.red()
        )
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
// "This is an example Text and simulation how the algorithm should work!" Searchable: example =>
// e=0, l=1, p=2, m=3, a=4, x=5, allOther=$LENGTH+1 LENGTH=7
// This is an example Text and simulation how the algorithm should work!
//        ^ pivot=" " => allOther weiter
//               ^ pivot="a" => 3 weiter
//                  ^ pivot="e" => O weiter also ein zurueck pruefen
//                 ^ pivot="l" => O weiter also ein zurueck pruefen
//                ^ pivot="p" => O weiter also ein zurueck pruefen
//               ^ pivot="m" => O weiter also ein zurueck pruefen
//              ^ pivot="a" => O weiter also ein zurueck pruefen
//             ^ pivot="x" => O weiter also ein zurueck pruefen
//            ^ pivot="e" => O weiter also ein zurueck pruefen ::FOUND::
// Liste found erweitern mit index Found && pivot = pivot + LENGTH
//                         ^ pivot="a" => 8 weiter also ein zurueck pruefen

fn search_searchable_in_string(
    searchable: &String,
    string: String,
    steps_per_char: &HashMap<char, usize>,
) -> String {
    let mut pivot = searchable.len();
    let mut matched_indicies: LinkedList<usize> = LinkedList::new();
    loop {
        let char_at_pivot = string.chars().nth(pivot).unwrap();
        let mut steps = steps_per_char
            .get(&char_at_pivot)
            .copied()
            .unwrap_or(searchable.len());

        println!(
            "This is char: {} for pivot: {} and implies steps: {}",
            char_at_pivot, pivot, steps
        );

        if steps == 0 {
            let start: isize = pivot as isize - searchable.len() as isize + 1;
            let end: usize = pivot + 1;

            // If Beginning is directly the matching, but there is not possible to match whole
            if start < 0 {
                steps = searchable.len();
                break;
            }
            let result: SearchResult = check_word(&string[start as usize..end], searchable);
            match result {
                SearchResult::Matched() => {
                    matched_indicies.push_back(pivot - searchable.len());
                    steps = searchable.len();
                }
                SearchResult::NotMatchedBecause(char_to_proove) => {
                    steps = steps_per_char
                        .get(&char_to_proove)
                        .copied()
                        .unwrap_or(searchable.len());
                }
            }
        }
        pivot += steps;

        if pivot >= string.len() {
            break;
        }
    }

    let mut result_string = String::new();
    let mut last = 0;

    for matched_at in matched_indicies {
        let end = matched_at + searchable.len() + 1;
        result_string.push_str(&string[last..matched_at]);
        result_string.push_str(&string[matched_at..end].green().to_string());
        last = end;
    }
    result_string.push_str(&string[last..string.len()]);

    result_string
}

fn check_word(string: &str, searchable: &String) -> SearchResult {
    println!("stringslice: {} <=> searchable: {}", string, searchable);

    for (index, char_to_proove) in string.chars().rev().enumerate() {
        if char_to_proove == searchable.chars().rev().nth(index).unwrap() {
            println!("THis should be matching");
        } else {
            println!(
                "This is not matching because char_to_proove: {} is not {} for index: {}",
                char_to_proove,
                searchable.chars().nth(searchable.len() - index).unwrap(),
                index
            );
            return SearchResult::NotMatchedBecause(char_to_proove);
        }
    }
    SearchResult::Matched()
}

// time spent: 40 min
// lesson learned:
// Rust hat keine Exception und nutzt Typen um zwischen Fehler zu unterscheiden
// offene Frage:
// Was sind Macros
// Was machen die Pipes (|)
//
