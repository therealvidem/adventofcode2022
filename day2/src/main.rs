mod shape;

use std::{io::{BufReader, BufRead}};

use common::aoc_fetch_input;

use crate::shape::{get_outcome_from_my_move, get_shape_from_opponent_move, get_shape_from_my_move, get_outcome_score};

static DAY: u8 = 2;
static INPUT_FILE_NAME: &'static str = "input.txt";

fn part1_get_score_from_line(line_result: Result<String, std::io::Error>, i: usize) -> u32 {
    let line = line_result.expect(&format!("Could not parse line {}", i + 1));
    let mut split = line.split(" ");
    let Some(opponent_move) = split.next() else {
        panic!("Could not parse opponent move on line {i}: '{line}'");
    };
    let Some(my_move) = split.next() else {
        panic!("Could not parse my move on line {i}: '{line}'");
    };
    let opponent_move = get_shape_from_opponent_move(opponent_move);
    let my_move = get_shape_from_my_move(my_move);
    let shape_score = my_move.score();
    let outcome_score = get_outcome_score(my_move, opponent_move);
    shape_score + outcome_score
}

fn part2_get_score_from_line(line_result: Result<String, std::io::Error>, i: usize) -> u32 {
    let line = line_result.expect(&format!("Could not parse line {}", i + 1));
    let mut split = line.split(" ");
    let Some(opponent_move) = split.next() else {
        panic!("Could not parse opponent move on line {i}: '{line}'");
    };
    let Some(my_move) = split.next() else {
        panic!("Could not parse my move on line {i}: '{line}'");
    };
    let opponent_move = get_shape_from_opponent_move(opponent_move);
    let outcome = get_outcome_from_my_move(my_move);
    let my_move = outcome.get_shape_from_opponent_move(opponent_move);
    let shape_score = my_move.score();
    let outcome_score = get_outcome_score(my_move, opponent_move);
    shape_score + outcome_score
}

#[tokio::main]
async fn main() {
    let mut file = std::fs::File::open(INPUT_FILE_NAME);

    if file.is_err() {
        file = Ok(aoc_fetch_input(INPUT_FILE_NAME, DAY).await.expect(&format!("Could not fetch day {DAY}'s input")));
    }

    let Ok(file) = file else {
        panic!("Could not open file '{INPUT_FILE_NAME}'");
    };

    let reader = BufReader::new(file);
    
    let mut total_score = 0u32;

    for (i, line_result) in reader.lines().enumerate() {
        total_score += part2_get_score_from_line(line_result, i);
    }

    println!("My total score: {total_score}");
}