use std::{fs::File, io::Read, cmp::max};

use common::{aoc_get_file};
use ndarray::{prelude::*, OwnedRepr};

static DAY: u8 = 8;
static INPUT_FILE_NAME: &'static str = "input.txt";

fn get_map(file: &mut File) -> ArrayBase<OwnedRepr<i32>, Dim<[usize; 2]>> {
	let mut buf = String::new();
	file.read_to_string(&mut buf).unwrap_or_else(|e| panic!("Could not read file: {e}"));
	let lines = buf.split('\n').filter(|l| l.len() > 0);
	
    let map = lines
		.clone()
		.map(|l| {
			l
				.chars()
				.map(|n| n.to_digit(10).unwrap_or_else(|| panic!("Could not convert {n} to a number")) as i32)
		})
		.flatten()
		.collect::<Vec<i32>>();
	let num_rows = lines.count();

	let map = Array2::from_shape_vec(
		(num_rows, map.len() / num_rows), 
		map
	)
		.unwrap_or_else(|e| panic!("Could not convert input to 2d array: {e}"));
	
	// println!("{map}");
	map
}

fn task1(file: &mut File) {
	let map = get_map(file);

	// Would need a special case for when m or n is 2, but the given input is way beyond that.
	
	let mut visible = 0;
	let num_rows = map.len_of(Axis(0));
	let num_cols = map.len_of(Axis(1));
	let num_edge_elements = (num_cols * 2) + (num_rows * 2) - 4;
	println!("Num edge elements: {num_edge_elements}");
	for i in 1..num_rows - 1 {
		for j in 1..num_cols - 1 {
			// println!("({i}, {j})");
			let val = map[[i, j]];
			// elevated_map[[i, j]] = -1;
			let row = map.row(i);
			let col = map.column(j);

			let mut left = row.iter().enumerate().take(j);
			let mut right = row.iter().enumerate().skip(j + 1);
			let mut up = col.iter().enumerate().take(i);
			let mut down = col.iter().enumerate().skip(i + 1);

			if left.all(|(_, n)| n < &val)
			|| right.all(|(_, n)| n < &val)
			|| up.all(|(_, n)| n < &val)
			|| down.all(|(_, n)| n < &val)
			 {
				visible += 1;
			}
		}
	}
	
	println!("Number of visible trees: {}", visible + num_edge_elements);
}

fn task2(file: &mut File) {
	let map = get_map(file);

	// Would need a special case for when m or n is 2, but the given input is way beyond that.
	
	let mut max_scenic_score = 0;
	let num_rows = map.len_of(Axis(0));
	let num_cols = map.len_of(Axis(1));
	for i in 1..num_rows - 1 {
		for j in 1..num_cols - 1 {
			// println!("({i}, {j})");
			let val = map[[i, j]];
			// elevated_map[[i, j]] = -1;
			let row = map.row(i);
			let col = map.column(j);

			let mut left = row.iter().enumerate().take(j);
			let mut right = row.iter().enumerate().skip(j + 1);
			let mut up = col.iter().enumerate().take(i);
			let mut down = col.iter().enumerate().skip(i + 1);

			let left_view_distance = match left.rfind(|(_, v)| v >= &&val) {
				Some((x, _)) => j - x,
				None => j,
			};
			let right_view_distance = match right.find(|(_, v)| v >= &&val) {
				Some((x, _)) => x - j,
				None => num_cols - j - 1,
			};
			let up_view_distance = match up.rfind(|(x, v)| v >= &&val) {
				Some((x, _)) => i - x,
				None => i,
			};
			let down_view_distance = match down.find(|(x, v)| v >= &&val) {
				Some((x, _)) => x - i,
				None => num_rows - i - 1,
			};

			let scenic_score = left_view_distance * right_view_distance * up_view_distance * down_view_distance;
			
			max_scenic_score = max(max_scenic_score, scenic_score);
		}
	}

	println!("Best scenic score: {max_scenic_score}");
}

#[tokio::main]
async fn main() {
    let mut file = aoc_get_file(INPUT_FILE_NAME, DAY).await.unwrap();
	// task1(&mut file);
	task2(&mut file);
}