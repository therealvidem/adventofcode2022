use anyhow::{Result, anyhow};

use std::str::FromStr;
use std::ops::{AddAssign, Sub, Add, IndexMut, Index};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pos {
    pub val: (i32, i32),
}

impl Pos {
    pub fn from_movement(dir: Direction, num_steps: i32) -> Self {
        match dir {
            Direction::Up => Pos::from((0, num_steps)),
            Direction::Down => Pos::from((0, -num_steps)),
            Direction::Left => Pos::from((-num_steps, 0)),
            Direction::Right => Pos::from((num_steps, 0)),
        }
    }
    
    pub fn zero() -> Self {
        Self {
            val: (0, 0),
        }
    }
}

impl Default for Pos {
    fn default() -> Self {
        Self::zero()
    }
}

impl From<(i32, i32)> for Pos {
    fn from(pos: (i32, i32)) -> Self {
        Self {
            val: pos,
        }
    }
}

impl Pos {
    pub fn x(&self) -> &i32 {
        &self[0]
    }

    pub fn x_mut(&mut self) -> &mut i32 {
        &mut self[0]
    }

    pub fn y(&self) -> &i32 {
        &self[1]
    }

    pub fn y_mut(&mut self) -> &mut i32 {
        &mut self[1]
    }
}

impl Index<usize> for Pos {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.val.0,
            1 => &self.val.1,
            _ => panic!("Invalid index to {self:?}: {index}"),
        }
    }
}

impl IndexMut<usize> for Pos {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.val.0,
            1 => &mut self.val.1,
            _ => panic!("Invalid index to {self:?}: {index}"),
        }
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            val: (self.x() + rhs.x(), self.y() + rhs.y()),
        }
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            val: (self.x() - rhs.x(), self.y() - rhs.y()),
        }
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.val = (self.x() + rhs.x(), self.y() + rhs.y());
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('U') => Ok(Direction::Up),
            Some('D') => Ok(Direction::Down),
            Some('L') => Ok(Direction::Left),
            Some('R') => Ok(Direction::Right),
            _ => Err(anyhow!("Could not parse direction in '{s}'")),
        }
    }
}

impl From<Direction> for Pos {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Up => Self::from((0, 1)),
            Direction::Down => Self::from((0, -1)),
            Direction::Left => Self::from((-1, 0)),
            Direction::Right => Self::from((1, 0)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Axis {
    Horizontal,
    Vertical,
}

impl From<Direction> for Axis {
    fn from(val: Direction) -> Self {
        match val {
            Direction::Up | Direction::Down => Axis::Vertical,
            Direction::Left | Direction::Right => Axis::Horizontal,
        }
    }
}

impl From<Axis> for usize {
    fn from(val: Axis) -> Self {
        match val {
            Axis::Horizontal => 0,
            Axis::Vertical => 1,
        }
    }
}
