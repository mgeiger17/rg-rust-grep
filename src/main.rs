use clap::Parser;
use colored::Colorize;
use std::collections::{HashMap, LinkedList};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum SearchResult {
    Matched(),
    NotMatchedBecause(char),
}

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about = "Just a Grep alternative", long_about = None)]
struct Args {
    #[arg(short, long)]
    searchable: String,
    #[arg(short, long)]
    filepath: String,
    #[arg(short, long, default_value_t = false)]
    ignore_case: bool,
}

fn main() {
    let args: Args = Args::parse();

    let searchable = &args.searchable;
    let file_path = &args.filepath;
    let ignore_case = args.ignore_case;

    println!("\nQuery: `{searchable}` | File: `{file_path}`\n");

    let mut map: HashMap<char, usize> = HashMap::new();

    for (i, c) in searchable.chars().rev().enumerate() {
        if map.get(&c) == None {
            if ignore_case {
                map.insert(c.to_ascii_lowercase(), i);
            } else {
                map.insert(c, i);
            }
        }
    }

    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for (i, line) in lines.map_while(Result::ok).enumerate() {
            let result = search_searchable_in_string(&searchable, line, &map, ignore_case);
            println!("{}. {}", i + 1, result)
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
    ignore_case: bool,
) -> String {
    let mut pivot = searchable.len() - 1;
    let mut matched_indicies: LinkedList<usize> = LinkedList::new();
    let chars_string: Vec<char> = string.chars().collect();
    loop {
        let mut char_at_pivot = chars_string[pivot];
        if ignore_case {
            char_at_pivot = char_at_pivot.to_ascii_lowercase();
        }
        let mut steps = steps_per_char
            .get(&char_at_pivot)
            .copied()
            .unwrap_or(searchable.len());

        if steps == 0 {
            let start: isize = pivot as isize - searchable.len() as isize + 1;
            let end: usize = pivot + 1;

            let result: SearchResult =
                check_word(&string[start as usize..end], searchable, ignore_case);
            match result {
                SearchResult::Matched() => {
                    matched_indicies.push_back(pivot + 1 - searchable.len());
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
        let end = matched_at + searchable.len();
        result_string.push_str(&string[last..matched_at]);
        result_string.push_str(&string[matched_at..end].green().to_string());
        last = end;
    }
    result_string.push_str(&string[last..string.len()]);

    result_string
}

fn check_word(string: &str, searchable: &String, ignore_case: bool) -> SearchResult {
    let searchable_chars: Vec<char> = searchable.chars().collect();
    for (index, char_to_proove) in string.chars().rev().enumerate() {
        let mut is_equal = char_to_proove == searchable.chars().rev().nth(index).unwrap();
        if ignore_case {
            is_equal = char_to_proove.to_ascii_lowercase()
                == searchable_chars[searchable_chars.len() - 1 - index].to_ascii_lowercase()
        }
        if !(is_equal) {
            return SearchResult::NotMatchedBecause(char_to_proove);
        }
    }
    SearchResult::Matched()
}

// time spent: 40 min
// lesson learned:
// Rust hat keine Exception und nutzt Typen um zwischen Fehler zu unterscheiden
// Rust hat ebenfalls keine Klassen oder Interfaces dafuer werden struct und traits genutzt
// Rust hat das Ownerschip-Model mit 3 Regeln:
//  1. Jeder Wert hat einen einzigen Owner
//  2. Es kann nur einen Owner an einem Zeitpunkt geben
//  3. Geht das Object, dass das Ownership besitzt out of scope, so wird der Speicherbereich
//     geklaert
// offene Frage:
// Was sind Macros
// Was machen die Pipes (|)
//
