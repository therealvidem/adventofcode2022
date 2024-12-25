use std::{io::{BufReader, BufRead}, fs::File, ops::Range, str::FromStr};

use anyhow::{Result, anyhow};
use common::aoc_fetch_input;

static DAY: u8 = 4;
static INPUT_FILE_NAME: &'static str = "input.txt";

#[derive(Debug)]
struct Assignment {
    ids: Range<i32>,
}

impl FromStr for Assignment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut numbers = s.split("-");
        let Some(start) = numbers.next() else {
            return Err(anyhow!("Missing numbers to parse: {}", s));
        };
        let Some(end) = numbers.next() else {
            return Err(anyhow!("Missing numbers to parse: {}", s));
        };
        let start: i32 = start.parse()?;
        let end: i32 = end.parse()?;
        Ok(Assignment {
            ids: (start..end + 1),
        })
    }
}

impl Assignment {
    fn is_subset(&self, other: &Assignment) -> bool {
        self.ids.start >= other.ids.start && (self.ids.end - 1) <= (other.ids.end - 1)
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        if self.ids.start < other.ids.start {
            self.ids.end > other.ids.start
        } else {
            other.ids.end > self.ids.start
        }
    }
}

fn task1(file: File) {
    let reader = BufReader::new(file);

    let mut overlapping_assignments: Vec<(Assignment, Assignment)> = vec![];

    for (i, line_result) in reader.lines().enumerate() {
        let line = line_result.unwrap_or_else(|err| panic!("Could not parse line {}: {err}", i + 1));
        let mut assignments = line.split(",");
        let Some(assignment1) = assignments.next() else {
            panic!("Could not parse line {}", i + 1);
        };
        let Some(assignment2) = assignments.next() else {
            panic!("Could not parse line {}", i + 1);
        };
        let assignment1 = assignment1.parse::<Assignment>().expect("Failed to parse assignment1");
        let assignment2 = assignment2.parse::<Assignment>().expect("Failed to parse assignment1");        
        if assignment1.ids.start <= assignment2.ids.start && assignment2.is_subset(&assignment1) {
            overlapping_assignments.push((assignment1, assignment2));
        } else if assignment2.ids.start <= assignment1.ids.start && assignment1.is_subset(&assignment2) {
            overlapping_assignments.push((assignment2, assignment1));
        }
    }

    println!("Total number of assignments in which one fully contains the other: {}", overlapping_assignments.len());
}

fn task2(file: File) {
    let reader = BufReader::new(file);

    let mut overlapping_assignments: Vec<(Assignment, Assignment)> = vec![];

    for (i, line_result) in reader.lines().enumerate() {
        let line = line_result.unwrap_or_else(|err| panic!("Could not parse line {}: {err}", i + 1));
        let mut assignments = line.split(",");
        let Some(assignment1) = assignments.next() else {
            panic!("Could not parse line {}", i + 1);
        };
        let Some(assignment2) = assignments.next() else {
            panic!("Could not parse line {}", i + 1);
        };
        let assignment1 = assignment1.parse::<Assignment>().expect("Failed to parse assignment1");        
        let assignment2 = assignment2.parse::<Assignment>().expect("Failed to parse assignment1");        
        if assignment1.overlaps(&assignment2) {
            overlapping_assignments.push((assignment1, assignment2));
        }
    }

    // println!("{:?}", overlapping_assignments);

    println!("Total number of overlapping assignments: {}", overlapping_assignments.len());
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
