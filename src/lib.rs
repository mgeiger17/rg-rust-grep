use clap::Parser;
use colored::Colorize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialEq, Debug)]
pub enum SearchResult {
    Matched(),
    NotMatchedBecause(char),
}

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about = "Just a Grep alternative", long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub searchable: String,
    #[arg(short, long)]
    pub filepath: String,
    #[arg(short, long, default_value_t = false)]
    pub ignore_case: bool,
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let searchable = &args.searchable;
    let file_path = &args.filepath;
    let ignore_case = args.ignore_case;

    println!("\nQuery: `{searchable}` | File: `{file_path}`\n");

    let steps_per_char: HashMap<char, usize> = create_steps_per_char(&searchable, ignore_case);

    let lines = read_lines(file_path)?;
    {
        // Consumes the iterator, returns an (Optional) String
        for (i, line) in lines.map_while(Result::ok).enumerate() {
            let result =
                search_searchable_in_string(&searchable, line, &steps_per_char, ignore_case);
            println!("{}. {}", i + 1, result)
        }
    }
    Ok(())
}

fn create_steps_per_char(searchable: &String, ignore_case: bool) -> HashMap<char, usize> {
    let mut steps_per_char: HashMap<char, usize> = HashMap::new();

    for (index, char_searchable) in searchable.chars().rev().enumerate() {
        if steps_per_char.get(&char_searchable) == None {
            if ignore_case {
                steps_per_char.insert(char_searchable.to_ascii_lowercase(), index);
            } else {
                steps_per_char.insert(char_searchable, index);
            }
        }
    }
    steps_per_char
}

/// The output is wrapped in a Result to allow matching on errors.
/// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// "This is an example Text and simulation how the algorithm should work!"
/// Searchable: example =>
///  |-------------------------------|
///  | e | x | a | m | p | l | other |
///  | 0 | 5 | 4 | 3 | 2 | 1 | 7     |
/// This is an example Text and simulation how the algorithm should work!
///        ^ pivot=" " => allOther weiter
///               ^ pivot="a" => 3 weiter
///                  ^ pivot="e" => O weiter also ein zurueck pruefen
///                 ^ pivot="l" => O weiter also ein zurueck pruefen
///                ^ pivot="p" => O weiter also ein zurueck pruefen
///               ^ pivot="m" => O weiter also ein zurueck pruefen
///              ^ pivot="a" => O weiter also ein zurueck pruefen
///             ^ pivot="x" => O weiter also ein zurueck pruefen
///            ^ pivot="e" => O weiter also ein zurueck pruefen ::FOUND::
/// Liste found erweitern mit index Found && pivot = pivot + LENGTH
///                         ^ pivot="a" => 8 weiter also ein zurueck pruefen
fn search_searchable_in_string(
    searchable: &String,
    input: String,
    steps_per_char: &HashMap<char, usize>,
    ignore_case: bool,
) -> String {
    let mut pivot = searchable.len() - 1;
    let mut matched_indicies: Vec<usize> = Vec::new();
    let chars_string: Vec<char> = input.chars().collect();
    loop {
        if pivot >= chars_string.len() {
            break;
        }

        let mut char_at_pivot = chars_string[pivot];
        if ignore_case {
            char_at_pivot = char_at_pivot.to_ascii_lowercase();
        }
        let mut steps = steps_per_char
            .get(&char_at_pivot)
            .copied()
            .unwrap_or(searchable.len());

        if steps == 0 {
            handle_zero_steps(
                &mut matched_indicies,
                searchable,
                &input,
                &pivot,
                &mut steps,
                steps_per_char,
                ignore_case,
            );
        }
        pivot += steps;
    }

    highlight_result(matched_indicies, searchable, &input)
}

fn highlight_result(matched_indicies: Vec<usize>, searchable: &String, input: &String) -> String {
    let mut result_string = String::new();
    let mut last = 0;

    for matched_at in matched_indicies {
        let end = matched_at + searchable.len();
        result_string.push_str(&input[last..matched_at]);
        result_string.push_str(&input[matched_at..end].green().to_string());
        last = end;
    }
    result_string.push_str(&input[last..input.len()]);
    result_string
}

fn handle_zero_steps(
    matched_indicies: &mut Vec<usize>,
    searchable: &String,
    input: &String,
    pivot: &usize,
    steps: &mut usize,
    steps_per_char: &HashMap<char, usize>,
    ignore_case: bool,
) {
    let start: usize = pivot - searchable.len() + 1;
    let end: usize = pivot + 1;

    let result: SearchResult = check_for_match(&input[start..end], searchable, ignore_case);
    match result {
        SearchResult::Matched() => {
            matched_indicies.push(pivot + 1 - searchable.len());
            *steps = searchable.len();
        }
        SearchResult::NotMatchedBecause(char_to_proove) => {
            *steps = steps_per_char
                .get(&char_to_proove)
                .copied()
                .unwrap_or(searchable.len());

            // handle case, if potential that the not matching index is, with zero steps
            if *steps == 0 {
                *steps = searchable.len();
            }
        }
    }
}

/// This function checks if the word is fitting, and if not returning the incorrect char, which was
/// not matching from the given string.
///
///
/// ```
/// use rust_jorney::{check_for_match, SearchResult};
/// let input = "Girlfriind";
/// let searchable = String::from("Girlfriend");
///
/// // No Success
/// assert_eq!(
///     check_for_match(input, &searchable, false),
///     SearchResult::NotMatchedBecause('i'));
/// // Success
/// assert_eq!(
///     check_for_match("Girlfriend", &searchable, false),
///     SearchResult::Matched()
/// );
/// ```
pub fn check_for_match(input: &str, searchable: &String, ignore_case: bool) -> SearchResult {
    let searchable_chars: Vec<char> = searchable.chars().collect();
    for (index, char_to_proove) in input.chars().rev().enumerate() {
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
