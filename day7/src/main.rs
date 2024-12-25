use std::{fs::File};

use common::{aoc_get_file, for_each_line};

static DAY: u8 = 7;
static INPUT_FILE_NAME: &'static str = "input.txt";

#[derive(Debug, PartialEq, Default)]
struct FileObject
{
	name: String,
	size: Option<usize>,
}

#[derive(Debug)]
struct Node<T>
{
	idx: usize,
	val: T,
	parent_idx: Option<usize>,
	children: Vec<usize>,
}

#[derive(Debug, Default)]
struct ArenaTree<T>
{
	arena: Vec<Node<T>>,
	root_idx: usize,
}

impl<T> Node<T>
{
	fn new(idx: usize, val: T) -> Self {
		Self {
			idx,
			val,
			parent_idx: None,
			children: vec![],
		}
	}
}

impl<T> ArenaTree<T>
{
	fn get(&self, idx: usize) -> &Node<T> {
		&self.arena[idx]
	}

	fn get_mut(&mut self, idx: usize) -> &mut Node<T> {
		&mut self.arena[idx]
	}

	fn node(&mut self, val: T) -> usize {
		let idx = self.arena.len();
		self.arena.push(Node::new(idx, val));
		idx
	}
}

fn handle_command(tree: &ArenaTree<FileObject>, line: &str, args: &Vec<&str>, root_dir_idx: usize, cur_dir_idx: &mut usize) {
    let command = args[1];
    match command {
		"cd" => {
			if args.len() != 3 {
				panic!("Mismatching number of args for cd: {line}");
			}
			let dir = args[2];
			match dir {
				"/" => {
					*cur_dir_idx = tree.get(root_dir_idx).idx;
				},
				".." => {
					let parent_idx = tree.arena[*cur_dir_idx].parent_idx
						// .unwrap_or_else(|| panic!("{} does not have a parent directory", tree.arena[*cur_dir_idx].val.name));
						.unwrap_or(0);
					*cur_dir_idx = parent_idx;
				},
				_ => {
					*cur_dir_idx = *tree.get(*cur_dir_idx).children
						.iter()
						.find(|&&child_idx| tree.get(child_idx).val.name == dir)
						.unwrap_or_else(|| panic!("Could not find directory with name '{dir}': {line}"));
				},
			}
		},
		"ls" => (),
		_ => panic!("Unknown command: {command}"),
	}
}

fn get_cwd_str(tree: &ArenaTree<FileObject>, cur_dir_idx: usize) -> String {
	let mut path: Vec<&str> = vec![];
	path.push(&tree.get(cur_dir_idx).val.name.as_str());
	let mut cur_idx = tree.get(cur_dir_idx).parent_idx;
	while let Some(idx) = cur_idx  {
		path.push(&tree.get(idx).val.name.as_str());
		cur_idx = tree.get(idx).parent_idx;
	}
	path.join(" ")
}

fn update_file_sizes(file_size: Option<usize>, tree: &mut ArenaTree<FileObject>, cur_dir_idx: usize) {
    if let Some(file_size) = file_size {
		let mut cur_idx = Some(cur_dir_idx);
		while let Some(idx) = cur_idx {
			let node = tree.get_mut(idx);
			let new_size = node.val.size.map_or(file_size, |s| s + file_size);
			node.val.size = Some(new_size);

			cur_idx = node.parent_idx;
		}
	}
}

fn insert_file(tree: &mut ArenaTree<FileObject>, cur_dir_idx: usize, file_name: &str, file_size: Option<usize>) {
    if tree.get(cur_dir_idx).children
		.iter()
		.find(|&&child_idx| tree.get(child_idx).val.name == file_name)
		.is_some() {
		println!("Already found '{file_name}' under {}", get_cwd_str(&tree, cur_dir_idx));
		return;
	}
    let idx = tree.node(FileObject {
		name: file_name.to_string(),
		size: file_size,
	});
    tree.get_mut(idx).parent_idx = Some(cur_dir_idx);
	tree.get_mut(cur_dir_idx).children.push(idx);
	update_file_sizes(file_size, tree, cur_dir_idx);
}

fn build_file_tree(file: &File) -> ArenaTree<FileObject> {
	let mut tree: ArenaTree<FileObject> = ArenaTree::default();

	let root_idx = tree.node(FileObject {
		name: String::from("/"),
		size: None,
	});
	tree.root_idx = root_idx;
	let mut cur_dir_idx = root_idx;

	for_each_line(&file, |i, line| {
		// println!("{tree:#?}");

		let args: Vec<&str> = line.split(" ").collect();
		if line.starts_with("$") {
			if args.len() < 2 {
				panic!("A line executing a command has no command: {line}");
			}

			handle_command(&tree, line, &args, root_idx, &mut cur_dir_idx);
		} else {
			let mut parts = line.split(" ");

			let Some(first) = parts.next() else {
				panic!("Cannot find size of file: {line}");
			};
			
			let Some(second) = parts.next() else {
				panic!("Cannot find size of file: {line}");
			};

			if first == "dir" {
				let dir_name = second;
				insert_file(&mut tree, cur_dir_idx, dir_name, None);
			} else {
				let file_size = first.parse::<usize>().unwrap_or_else(|e| panic!("Invalid size in '{line}': {e}"));
				let file_name = second;

				insert_file(&mut tree, cur_dir_idx, file_name, Some(file_size));
				
				// // This is probably not needed idk
				// if let Some(&idx) = tree.arena[cur_dir_idx].children
				// 	.iter()
				// 	.find(|&&child_idx| tree.arena[child_idx].val.name == file_name) {
				// 	tree.arena[idx].val.size = Some(file_size);
				// 	return;
				// }
				// let idx = tree.node(FileObject {
				// 	name: file_name.to_string(),
				// 	size: Some(file_size),
				// });
				// tree.arena[idx].parent = Some(cur_dir_idx);
			}
		}
	});

	tree
}

fn is_dir<T>(node: &Node<T>) -> bool {
	node.children.len() > 0
}

fn task1(file: &File) {
	let tree = build_file_tree(file);

	let mut sum = 0usize;
	for target in tree.arena
		.iter()
		.filter(|&n| {
			is_dir(n) && n.val.size.expect(&format!("'{}' does not have a file size", get_cwd_str(&tree, n.idx))) <= 100_000
		}) {
		println!("{target:#?}");
		sum += target.val.size.unwrap();
	}
	// println!("{tree:#?}");
	println!("Sum of all directories with at most size 100000: {sum}");
}

fn task2(file: &File) {
	let tree = build_file_tree(file);

	let target_unused_size = 30_000_000usize;
	let unused_size = 70_000_000 - tree.get(tree.root_idx).val.size.unwrap();
	let target_size = target_unused_size - unused_size;
	let mut target_nodes: Vec<&Node<FileObject>> = tree.arena
		.iter()
		.filter(|&n| {
			is_dir(n) && n.val.size.expect(&format!("'{}' does not have a file size", get_cwd_str(&tree, n.idx))) >= target_size
		})
		.collect();
	target_nodes.sort_by(|a, b| {
		a.val.size.unwrap().partial_cmp(
			&b.val.size.unwrap()
		).unwrap()
	});

	println!("Smallest possible directory to free up to target size: {}", target_nodes[0].val.size.unwrap());
}

#[tokio::main]
async fn main() {
    let file = aoc_get_file(INPUT_FILE_NAME, DAY).await.unwrap();
	// task1(&file);
	task2(&file);
}
