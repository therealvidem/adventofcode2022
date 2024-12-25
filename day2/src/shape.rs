use std::{collections::HashMap, cmp::Ordering};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref WIN_ORDER: HashMap<Shape, Shape> = HashMap::from([
        (Shape::Rock, Shape::Scissors),
        (Shape::Paper, Shape::Rock),
        (Shape::Scissors, Shape::Paper),
    ]);
	
    pub static ref LOSE_ORDER: HashMap<Shape, Shape> = HashMap::from([
        (Shape::Rock, Shape::Paper),
        (Shape::Paper, Shape::Scissors),
        (Shape::Scissors, Shape::Rock),
    ]);

    pub static ref OPPONENT_SHAPES: HashMap<&'static str, Shape> = HashMap::from([
        ("A", Shape::Rock),
        ("B", Shape::Paper),
        ("C", Shape::Scissors),
    ]);

	pub static ref MY_SHAPES: HashMap<&'static str, Shape> = HashMap::from([
		("X", Shape::Rock),
		("Y", Shape::Paper),
		("Z", Shape::Scissors),
	]);

	pub static ref OUTCOME_TO_PLAY: HashMap<&'static str, Outcome> = HashMap::from([
		("X", Outcome::Lose),
		("Y", Outcome::Draw),
		("Z", Outcome::Win),
	]);
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Outcome {
	Win,
	Draw,
	Lose,
}

impl Outcome {
	pub fn get_shape_from_opponent_move(&self, opponent_move: Shape) -> Shape {
		// returns the necessary shape to achieve outcome
		// given opponent_move (i.e., such that returned > opponent_move if outcome is Win, returned < opponent_move if Lose, etc...)
		match self {
			Outcome::Win => *LOSE_ORDER.get(&opponent_move).expect(&format!("Unknown shape {opponent_move:?}")),
			Outcome::Draw => opponent_move,
			Outcome::Lose => *WIN_ORDER.get(&opponent_move).expect(&format!("Unknown shape: {opponent_move:?}")),
		}
	}
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn score(&self) -> u32 {
        match *self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Shape {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if WIN_ORDER.get(other) == Some(self) {
            Ordering::Less
        } else if WIN_ORDER.get(self) == Some(other) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

pub fn get_outcome_score(me: Shape, opponent: Shape) -> u32 {
    match me.cmp(&opponent) {
        Ordering::Less => 0,
        Ordering::Equal => 3,
        Ordering::Greater => 6,
    }
}

pub fn get_outcome_from_my_move(my_move: &str) -> Outcome {
    *OUTCOME_TO_PLAY.get(my_move).expect(&format!("Unknown outcome: {my_move}"))
}

pub fn get_shape_from_my_move(my_move: &str) -> Shape {
    *MY_SHAPES.get(my_move).expect(&format!("Unknown move: {my_move}"))
}

pub fn get_shape_from_opponent_move(opponent_move: &str) -> Shape {
    *OPPONENT_SHAPES.get(opponent_move).expect(&format!("Unknown move: {opponent_move}"))
}
