use clap::Parser;
use colored::Colorize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialEq, Debug)]
pub enum SearchResult {
    Matched,
    NotMatchedBecause(char, usize),
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

/// Starts a grep Tool with usage of the Boyer-Moore Algorithm
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

fn create_steps_per_char(searchable: &str, ignore_case: bool) -> HashMap<char, usize> {
    let mut steps_per_char: HashMap<char, usize> = HashMap::new();

    for (index, char_searchable) in searchable.chars().rev().enumerate() {
        let parsed_char_searchable;
        if ignore_case {
            parsed_char_searchable = char_searchable.to_ascii_lowercase();
        } else {
            parsed_char_searchable = char_searchable;
        }
        steps_per_char
            .entry(parsed_char_searchable)
            .or_insert(index);
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
    searchable: &str,
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
            // TODO: 3. `&input` → `&chars_string`
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

    // TODO: 6. `&input` → `&chars_string`
    highlight_result(matched_indicies, searchable, &input)
}

// TODO: 4. change to Vec<char> format
fn highlight_result(matched_indicies: Vec<usize>, searchable: &str, input: &str) -> String {
    let mut result_string = String::new();
    let mut last = 0;

    for matched_at in matched_indicies {
        let end = matched_at + searchable.len();
        // TODO: 5. change to Vec<char> format
        result_string.push_str(&input[last..matched_at]);
        result_string.push_str(&input[matched_at..end].green().to_string());
        last = end;
    }
    // TODO: 5. change to Vec<char> format
    result_string.push_str(&input[last..input.len()]);
    result_string
}

// TODO: 1. &Vec<char>
fn handle_zero_steps(
    matched_indicies: &mut Vec<usize>,
    searchable: &str,
    input: &str,
    pivot: &usize,
    steps: &mut usize,
    steps_per_char: &HashMap<char, usize>,
    ignore_case: bool,
) {
    let start: usize = match pivot.checked_sub(searchable.len() - 1) {
        Some(val) => val,
        None => panic!("pivot is to small"),
    };
    let end: usize = pivot + 1;

    // println!("start: {} end: {}  ", start, end);

    // TODO: 2.
    let result: SearchResult = check_for_match(&input[start..end], searchable, ignore_case);
    match result {
        SearchResult::Matched => {
            matched_indicies.push(pivot + 1 - searchable.len());
            *steps = searchable.len();
        }
        SearchResult::NotMatchedBecause(char_to_proove, pivot_decrease) => {
            *steps = steps_per_char
                .get(&char_to_proove)
                .copied()
                .unwrap_or(searchable.len());

            if *steps < pivot_decrease {
                *steps = 1;
            } else {
                *steps -= pivot_decrease;
            }

            // handle case, if potential that the not matching index is, with zero steps
            if *steps == 0 {
                // *steps = searchable.len();
                *steps = 1;
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
///     SearchResult::NotMatchedBecause('i', 7));
/// // Success
/// assert_eq!(
///     check_for_match("Girlfriend", &searchable, false),
///     SearchResult::Matched
/// );
/// ```
pub fn check_for_match(input: &str, searchable: &str, ignore_case: bool) -> SearchResult {
    let searchable_chars: Vec<char> = searchable.chars().collect();
    for (index, mut char_to_prove) in input.chars().rev().enumerate() {
        let mut is_equal = char_to_prove == searchable_chars[searchable_chars.len() - 1 - index];
        if ignore_case {
            char_to_prove = char_to_prove.to_ascii_lowercase();
            is_equal = char_to_prove
                == searchable_chars[searchable_chars.len() - 1 - index].to_ascii_lowercase()
        }

        if !(is_equal) {
            return SearchResult::NotMatchedBecause(
                char_to_prove,
                searchable_chars.len() - 1 - index,
            );
        }
    }
    SearchResult::Matched
}

#[cfg(test)]
mod test {
    use std::fmt::format;

    use super::*;

    #[test]
    fn test_step_table() {
        let searchable = "Roller";

        let map_ignore_case_true: HashMap<char, usize> =
            HashMap::from([('r', 0), ('e', 1), ('l', 2), ('o', 4)]);
        assert_eq!(
            map_ignore_case_true,
            create_steps_per_char(&searchable, true)
        );

        let map_ignore_case_false: HashMap<char, usize> =
            HashMap::from([('r', 0), ('e', 1), ('l', 2), ('o', 4), ('R', 5)]);
        assert_eq!(
            map_ignore_case_false,
            create_steps_per_char(&searchable, false)
        );
    }

    #[test]
    fn test_step_table_non_ascii() {
        let searchable = "1.FC Köln";

        let map_ignore_case_true: HashMap<char, usize> = HashMap::from([
            ('n', 0),
            ('l', 1),
            ('ö', 2),
            ('k', 3),
            (' ', 4),
            ('c', 5),
            ('f', 6),
            ('.', 7),
            ('1', 8),
        ]);
        assert_eq!(
            map_ignore_case_true,
            create_steps_per_char(&searchable, true)
        );

        let map_ignore_case_false: HashMap<char, usize> = HashMap::from([
            ('n', 0),
            ('l', 1),
            ('ö', 2),
            ('K', 3),
            (' ', 4),
            ('C', 5),
            ('F', 6),
            ('.', 7),
            ('1', 8),
        ]);
        assert_eq!(
            map_ignore_case_false,
            create_steps_per_char(&searchable, false)
        );
    }

    #[test]
    fn test_check_match() {
        let input = "1. FC Koeln";
        let searchable = String::from("1. FC Köln");
        assert_eq!(
            check_for_match(input, &searchable, false),
            SearchResult::NotMatchedBecause('e', 7)
        );
        // Success
        assert_eq!(
            check_for_match("1. FC Köln", &searchable, false),
            SearchResult::Matched
        );
    }

    #[test]
    fn test_highlighting() {
        let input = String::from("This is a String with äöüß!");
        let searchable = String::from("äöüß");

        let highlighted_str: String =
            format!("This is a String with {}!", "äöüß".green().to_string());

        let matched_indicies: Vec<usize> = vec![23];
        let str = highlight_result(matched_indicies, &searchable, &input);

        println!("to test: \n{}", highlighted_str);
        println!("correct: \n{}", str);
    }
}
