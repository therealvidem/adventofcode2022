use std::{io::{BufReader, BufRead}, fs::File, collections::{VecDeque}};

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use common::aoc_fetch_input;

static DAY: u8 = 5;
static INPUT_FILE_NAME: &'static str = "input.txt";

mod stack {
    #[derive(Parser)]
    #[grammar = "stack.pest"]
    pub struct StackParser;
}

mod step {
    #[derive(Parser)]
    #[grammar = "step.pest"]
    pub struct StepParser;
}

#[derive(PartialEq, Eq)]
enum ParseState {
    ParsingStack,
    ParsingStep,
}

fn task1(file: File) {
    let reader = BufReader::new(file);

    let mut stacks: VecDeque<VecDeque<char>> = VecDeque::new();

    let mut parsing_state = ParseState::ParsingStack;
    let mut lines = reader.lines().enumerate();

    while let Some((mut i, mut line_result)) = lines.next() {
        if i == 8 {
            for _ in 0..2 {
                (i, line_result) = lines.next().unwrap_or_else(|| panic!("Could not get next line"));
            }
            parsing_state = ParseState::ParsingStep;
            println!("Initial stack state: {stacks:?}");
        }

        let line = line_result.unwrap_or_else(|err| panic!("Could not parse line {}: {err}", i + 1));

        println!("{}", line);

        match parsing_state {
            ParseState::ParsingStack => {
                let pairs = stack::StackParser::parse(stack::Rule::row, line.as_str()).unwrap_or_else(|e| panic!("{e}"));

                let num_pairs = pairs.clone().count();
                stacks.resize(num_pairs, VecDeque::new());
            
                for (i, pair) in pairs.enumerate() {
                    if pair.as_rule() == stack::Rule::elf_crate {
                        let mut inner_pairs = pair.into_inner();
                        let letter_rule = inner_pairs.next().unwrap_or_else(|| panic!("Elf crate does not have inner pair"));
                        
                        stacks[i].push_front(letter_rule.as_str().chars().next().unwrap_or_else(|| panic!("Found no letter in elf crate")));
                    }
                }
            },
            ParseState::ParsingStep => {
                let mut pairs = step::StepParser::parse(step::Rule::step, line.as_str()).unwrap_or_else(|e| panic!("{e}"));
            
                let Some(amount_pair) = pairs.next() else {
                    panic!("Did not find amount of crates to move on line {i}");
                };
                let Some(from_pair) = pairs.next() else {
                    panic!("Did not find amount of crates to move on line {i}");
                };
                let Some(to_pair) = pairs.next() else {
                    panic!("Did not find amount of crates to move on line {i}");
                };
                let amount = amount_pair.as_str().parse::<usize>().unwrap_or_else(|e| panic!("{e}"));
                let from = from_pair.as_str().parse::<usize>().unwrap_or_else(|e| panic!("{e}")) - 1;
                let to = to_pair.as_str().parse::<usize>().unwrap_or_else(|e| panic!("{e}")) - 1;
                
                assert!(from < stacks.len());
                assert!(to < stacks.len());

                let mut popped: Vec<char> = vec![];
                for _ in 0..amount {
                    if stacks[from].len() == 0 {
                        panic!("Not enough items to pop off");
                    }
                    popped.push(stacks[from].pop_back().unwrap());
                }
                
                for item in popped {
                    stacks[to].push_back(item);
                }
            },
            _ => unreachable!(),
        }
    }

    println!("Final stack state: {stacks:?}");

    for stack in stacks {
        match stack.back() {
            Some(c) => print!("{c}"),
            None => (),
        }
    }
}

fn task2(file: File) {
    let reader = BufReader::new(file);

    let mut stacks: VecDeque<VecDeque<char>> = VecDeque::new();

    let mut parsing_state = ParseState::ParsingStack;
    let mut lines = reader.lines().enumerate();

    while let Some((mut i, mut line_result)) = lines.next() {
        if i == 8 {
            for _ in 0..2 {
                (i, line_result) = lines.next().unwrap_or_else(|| panic!("Could not get next line"));
            }
            parsing_state = ParseState::ParsingStep;
            println!("Initial stack state: {stacks:?}");
        }

        let line = line_result.unwrap_or_else(|err| panic!("Could not parse line {}: {err}", i + 1));

        println!("{}", line);

        match parsing_state {
            ParseState::ParsingStack => {
                let pairs = stack::StackParser::parse(stack::Rule::row, line.as_str()).unwrap_or_else(|e| panic!("{e}"));

                let num_pairs = pairs.clone().count();
                stacks.resize(num_pairs, VecDeque::new());
            
                for (i, pair) in pairs.enumerate() {
                    if pair.as_rule() == stack::Rule::elf_crate {
                        let mut inner_pairs = pair.into_inner();
                        let letter_rule = inner_pairs.next().unwrap_or_else(|| panic!("Elf crate does not have inner pair"));
                        
                        stacks[i].push_front(letter_rule.as_str().chars().next().unwrap_or_else(|| panic!("Found no letter in elf crate")));
                    }
                }
            },
            ParseState::ParsingStep => {
                let mut pairs = step::StepParser::parse(step::Rule::step, line.as_str()).unwrap_or_else(|e| panic!("{e}"));
            
                let Some(amount_pair) = pairs.next() else {
                    panic!("Did not find amount of crates to move on line {i}");
                };
                let Some(from_pair) = pairs.next() else {
                    panic!("Did not find amount of crates to move on line {i}");
                };
                let Some(to_pair) = pairs.next() else {
                    panic!("Did not find amount of crates to move on line {i}");
                };
                let amount = amount_pair.as_str().parse::<usize>().unwrap_or_else(|e| panic!("{e}"));
                let from = from_pair.as_str().parse::<usize>().unwrap_or_else(|e| panic!("{e}")) - 1;
                let to = to_pair.as_str().parse::<usize>().unwrap_or_else(|e| panic!("{e}")) - 1;
                
                assert!(from < stacks.len());
                assert!(to < stacks.len());

                let mut popped: Vec<char> = vec![];
                for _ in 0..amount {
                    if stacks[from].len() == 0 {
                        panic!("Not enough items to pop off");
                    }
                    popped.push(stacks[from].pop_back().unwrap());
                }
                
                for item in popped.iter().rev() {
                    stacks[to].push_back(*item);
                }
            },
            _ => unreachable!(),
        }
    }

    println!("Final stack state: {stacks:?}");

    for stack in stacks {
        match stack.back() {
            Some(c) => print!("{c}"),
            None => (),
        }
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
