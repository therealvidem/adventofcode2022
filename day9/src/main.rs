use std::{fs::File, collections::HashSet};

mod pos;
use crate::pos::*;

use common::{aoc_get_file, for_each_line};
use ndarray::Array2;

static DAY: u8 = 9;
static INPUT_FILE_NAME: &'static str = "example_input2.txt";

// This should be refactored into a generic T on struct Pos.
static NUM_DIMENSIONS: usize = 2;

fn get_new_pos<const N: usize>(knots: &mut [Pos; N], i: usize, axis: Axis, unit_movement: Pos) -> Pos {
    let head_pos = knots[i - 1];
    let mut res = knots[i].clone();
    let dt_pos = res - head_pos;
    // To get x of pos, do pos[0]. To get y, do pos[1].
    let dimension_index: usize = axis.into();

    if dt_pos[dimension_index].abs() > 1 {
        // Move diagonally to align other dimensions to head_pos
        for d in (0..NUM_DIMENSIONS).filter(|d| *d != dimension_index) {
            if dt_pos[d].abs() > 0 {
                res[d] -= dt_pos[d];
            }
        }
        
        // For clarification, I expanded out what was originally an AddAssign expression
        res[dimension_index] = res[dimension_index] + (-dt_pos[dimension_index] - unit_movement[dimension_index]);
    }

    res
}

fn get_tail_visits<const N: usize>(file: &mut File) {
    if N < 2 {
        panic!("Number of segments must be >= 2");
    }

    let mut tail_visited: HashSet<Pos> = HashSet::new();
    // let mut head_pos: Pos = Pos { val: (0, 0) };

    let mut knots = [Pos::zero(); N];
    // let mut tail_pos: Pos = Pos { val: (0, 0) };
    tail_visited.insert(knots[N - 1]);
    // println!("{head_pos:?} {tail_pos:?}");
    for_each_line(file, |i, line| {
        let mut split = line.split(" ");

        let Some(dir) = split.next() else {
            panic!("Could not find dir in line '{line}' (line {i})");
        };
        let Some(num_steps) = split.next() else {
            panic!("Could not find number of steps in '{line}' (line {i})");
        };

        let num_steps = num_steps.parse::<i32>().unwrap_or_else(|e| panic!("Could not parse num_steps in line '{line}' (line {i}) into a number: {e}"));
        let dir = dir.parse::<Direction>().unwrap_or_else(|e| panic!("Could not parse line '{line}' (line {i}) into a Direction: {e}"));

        let unit_movement: Pos = dir.into();
        let axis: Axis = dir.into();
        let movement_pos = Pos::from_movement(dir, num_steps);
    
        knots[0] += movement_pos;

        // Update all knots except the tail (very last knot)
        for i in 1..N - 1 {
            knots[i] = get_new_pos(&mut knots, i, axis, unit_movement);
        }

        // Update tail
        let old_pos = knots[N - 1];
        knots[N - 1] = get_new_pos(&mut knots, N - 1, axis, unit_movement);
        let new_pos = knots[N - 1];
        let dimension_index: usize = axis.into();
        for j in 1..(old_pos[dimension_index] - new_pos[dimension_index]).abs() + 1 {
            let mut visited_pos = new_pos;
            visited_pos[dimension_index] = old_pos[dimension_index] + j * unit_movement[dimension_index];
            tail_visited.insert(visited_pos);
            println!("Visited {visited_pos:?}");
        }

        let mut map = Array2::<usize>::zeros((30, 30));

        for i in (0..N).rev() {
            map[((-(knots[i].y() - 1) + 30/2) as usize, ((knots[i].x() - 1) + 30/2) as usize)] = if i == 0 { 10 } else { i };
        }

        println!("{map:#}");
        
        println!("{knots:#?}");
        // println!("{head_pos:?} {tail_pos:?}");
    });

    println!("Number of tail's visited positions: {}", tail_visited.len());
}

fn task1(file: &mut File) {
    get_tail_visits::<2>(file);
}

fn task2(file: &mut File) {
    get_tail_visits::<10>(file);
}

#[tokio::main]
async fn main() {
    let mut file = aoc_get_file(INPUT_FILE_NAME, DAY).await.unwrap();
	// task1(&mut file);
	task2(&mut file);
}