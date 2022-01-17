use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
};

use regex::Regex;

const WORD_FILE_PATH: &str = "words_alpha.txt";
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let pattern: String = args.get(1).cloned().unwrap_or_else(|| ".....".to_string());
    let included_chars: String = args.get(2).cloned().unwrap_or_else(|| "".to_string());
    let excluded_chars: String = args.get(3).cloned().unwrap_or_else(|| "".to_string());

    let included: HashSet<char> = HashSet::from_iter(included_chars.chars());
    let excluded: HashSet<char> = HashSet::from_iter(excluded_chars.chars());

    let word_list: Vec<_> = {
        let word_file = std::fs::File::open(WORD_FILE_PATH)
            .unwrap_or_else(|_| panic!("can't find word file '{WORD_FILE_PATH}'"));
        BufReader::new(word_file)
            .lines()
            .collect::<Result<Vec<_>, _>>()
            .unwrap_or_else(|e| panic!("error reading lines from '{WORD_FILE_PATH}': {e}"))
    };

    let regex =
        Regex::new(&pattern).unwrap_or_else(|_| panic!("pattern '{pattern}' is not a valid regex"));

    let mut match_count = 0;
    let dictionary_iter = word_list.iter().filter(|w| {
        w.len() == 5
            && {
                let w_chars = HashSet::from_iter(w.chars());
                w_chars.is_superset(&included) && w_chars.is_disjoint(&excluded)
            }
            && regex.is_match(w)
    });

    for w in dictionary_iter {
        match_count += 1;
        println!("- {w}");
    }
    println!("{}", "#".repeat(10));
    println!("found {match_count} matches");
}
