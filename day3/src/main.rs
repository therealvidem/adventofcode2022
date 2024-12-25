use std::{io::{BufReader, BufRead}, collections::{HashSet, HashMap}, fs::File};

use common::aoc_fetch_input;

static DAY: u8 = 3;
static INPUT_FILE_NAME: &'static str = "input.txt";

fn get_item_set_from_str(s: &str) -> HashSet<char> {
    let mut set = HashSet::new(); 
    
    for item in s.chars() {
        set.insert(item);
    }

    set
}

fn get_priority_from_strings<T: AsRef<str>>(strings: &[T], item_priorities: &HashMap<char, u32>) -> u32 {
    let mut sets: Vec<HashSet<char>> = vec![];

    for s in strings {
        let set = get_item_set_from_str(s.as_ref());
        sets.push(set);
    }

    let (intersection, others) = sets.split_at_mut(1);
    let intersection = &mut intersection[0];
    for other in others {
        intersection.retain(|e| other.contains(e));
    }

    let intersection: Vec<&char> = intersection.iter().collect();
    let num_common_items = intersection.len();
    if num_common_items != 1 {
        panic!("Expected 1 common item, got {num_common_items}");
    }

    let common_item = intersection[0];
    *item_priorities.get(&common_item).expect(&format!("Could not find priority for item: {common_item}"))
}

fn get_priority_from_line(line_result: Result<String, std::io::Error>, i: usize, item_priorities: &HashMap<char, u32>) -> u32 {
    let line = line_result.expect(&format!("Could not process line {}", i + 1));

    let items_count = line.len();

    let first_compartment = &line[0..items_count / 2];
    let second_compartment = &line[items_count / 2..];

    get_priority_from_strings(&vec![first_compartment, second_compartment], item_priorities)

    // let first_set = get_item_set_from_str(first_compartment);
    // let second_set = get_item_set_from_str(second_compartment);

    // let intersection = first_set.intersection(&second_set);
    // let intersection = intersection.collect::<Vec<_>>();
    // let num_common_items = intersection.len();
    // if num_common_items != 1 {
    //     panic!("Expected 1 common item, got {num_common_items}");
    // }

    // let common_item = intersection[0];
    // // println!("Common item between {first_compartment} and {second_compartment}: {common_item}");
    // *item_priorities.get(common_item).expect(&format!("Could not find priority for item: {common_item}"))
}

fn get_item_priorities() -> HashMap<char, u32> {
    let mut priorities: HashMap<char, u32> = HashMap::new();
    let mut curr_priority = 1u32;
    
    for i in ('a' as u8)..('z' as u8 + 1) {
        priorities.insert(i as char, curr_priority);
        curr_priority += 1;
    }

    for i in ('A' as u8)..('Z' as u8 + 1) {
        priorities.insert(i as char, curr_priority);
        curr_priority += 1;
    }

    priorities
}

fn task1(file: File) {
    let reader = BufReader::new(file);

    let mut total_sum = 0u32;
    let item_priorities = get_item_priorities();

    for (i, line_result) in reader.lines().enumerate() {
        total_sum += get_priority_from_line(line_result, i, &item_priorities);
    }

    println!("Total sum of priorities: {total_sum}");
}

fn task2(file: File) {
    let reader = BufReader::new(file);

    let mut total_sum = 0u32;
    let item_priorities = get_item_priorities();

    for (i, line_result) in reader.lines().collect::<Vec<_>>().chunks(3).enumerate() {
        let lines: Vec<_> = line_result
            .iter()
            .map(|r| r.as_ref().expect(&format!("Could not parse line within the chunk on line {}", (i * 3) + 1)))
            .collect();
        total_sum += get_priority_from_strings(&lines, &item_priorities);
    }

    println!("Total sum of priorities: {total_sum}");
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

    task2(file);
}
