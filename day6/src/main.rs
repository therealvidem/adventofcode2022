use std::{io::{BufReader, BufRead}, fs::File, collections::HashMap};

use common::aoc_fetch_input;

static DAY: u8 = 6;
static INPUT_FILE_NAME: &'static str = "input.txt";

fn get_first_marker(line: impl Into<String>, marker_size: usize) -> Option<(usize, usize)> {
    let line: String = line.into();
    let mut counts: HashMap<char, usize> = HashMap::new();
    for end in 0..line.len() {
        counts
            .entry(line.chars().nth(end).unwrap())
            .and_modify(|c| { *c += 1 })
            .or_insert(1);
        
        if end >= marker_size {
            let dropped_char = line.chars().nth(end - marker_size).unwrap();
            // Should never panic
            let dropped_char_count = counts.get_mut(&dropped_char).unwrap();
            if *dropped_char_count > 1 {
                *dropped_char_count -= 1;
            } else {
                counts.remove(&dropped_char);
            }
        }

        if counts.len() == marker_size {
            return Some((end - marker_size + 1, end));
        }
    }
    None
}

fn task1(file: File) {
    let mut reader = BufReader::new(file);

    let mut line: String = String::new();
    reader.read_line(&mut line).unwrap_or_else(|e| panic!("Could not parse line: {e}"));
    // Remove newline
    line.pop();

    println!("{line}");
    
    match get_first_marker(line, 4) {
        Some((start, end)) => {
            println!("First marker at: ({start}, {end})");
        },
        None => println!("No marker found!"),
    }
}

fn task2(file: File) {
    let mut reader = BufReader::new(file);

    let mut line: String = String::new();
    reader.read_line(&mut line).unwrap_or_else(|e| panic!("Could not parse line: {e}"));
    // Remove newline
    line.pop();

    println!("{line}");
    
    match get_first_marker(line, 14) {
        Some((start, end)) => {
            println!("First marker at: ({start}, {end})");
        },
        None => println!("No marker found!"),
    }
}

#[tokio::main]
async fn main() {
    let mut file = std::fs::File::open(INPUT_FILE_NAME);

    if file.is_err() {
        file = Ok(aoc_fetch_input(INPUT_FILE_NAME, DAY).await.expect(&format!("Could not fetch day {DAY}'s input")));
    }

    let file = file.unwrap_or_else(|err| panic!("Could not open file '{INPUT_FILE_NAME}': {err}"));

    task2(file);
}
