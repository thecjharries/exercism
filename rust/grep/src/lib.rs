use anyhow::Error;
use regex::Regex;

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// In the real world, it's common to use crates such as [`clap`] or
/// [`structopt`] to handle argument parsing, and of course doing so is
/// permitted in this exercise as well, though it may be somewhat overkill.
///
/// [`clap`]: https://crates.io/crates/clap
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
/// [`structopt`]: https://crates.io/crates/structopt
#[derive(Debug)]
pub struct Flags {
    line_numbers: bool,
    file_names: bool,
    case_insensitive: bool,
    invert_match: bool,
    match_entire_lines: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut line_numbers = false;
        let mut file_names = false;
        let mut case_insensitive = false;
        let mut invert_match = false;
        let mut match_entire_lines = false;

        for flag in flags {
            match flag {
                &"-n" => line_numbers = true,
                &"-l" => file_names = true,
                &"-i" => case_insensitive = true,
                &"-v" => invert_match = true,
                &"-x" => match_entire_lines = true,
                _ => (),
            }
        }

        Flags {
            line_numbers,
            file_names,
            case_insensitive,
            invert_match,
            match_entire_lines,
        }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut results = Vec::new();
    let mut search_pattern = Regex::new(pattern)?;
    for file in files {
        let contents = std::fs::read_to_string(file)?;
        let mut lines = contents.lines();
        let mut line_number = 1;
        while let Some(line) = lines.next() {
            if search_pattern.is_match(line) {
                results.push(line.to_string());
            }
            line_number += 1;
        }
    }
    Ok(results)
}
