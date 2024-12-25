use std::{fs::File, collections::{HashMap, BinaryHeap}, str::FromStr, cmp::Reverse};

use common::{aoc_get_file, for_each_line};
use strum_macros::{EnumString, EnumIter};

static DAY: u8 = 10;
static INPUT_FILE_NAME: &'static str = "input.txt";

#[derive(Debug, EnumIter, Hash, PartialEq, Eq)]
enum Register {
    X,
}

struct Registers {
    _registers: HashMap<Register, i32>,
}

impl Registers {
    fn new() -> Self {
        Self {
            _registers: HashMap::new(),
        }
    }

    fn get(&self, register: Register) -> Option<&i32> {
        self._registers.get(&register)
    }

    fn add(&mut self, register: Register, by: i32) {
        // Registers have a default value of 1
        self._registers.entry(register).and_modify(|v| *v += by).or_insert(by + 1);
    }
}

trait Executable {
    fn execute<'a, I>(&self, registers: &mut Registers, args: &mut I)
    where
        I: Iterator<Item = &'a str>;
}

// struct Instruction {
//     num_cycles: u32,
// }

#[derive(Debug, PartialEq, EnumString)]
enum Instruction {
    #[strum(serialize = "noop")]
    Noop,
    #[strum(serialize = "addx")]
    AddX,
}

impl Instruction {
    fn num_cycles(&self) -> i32 {
        match *self {
            Instruction::Noop => 1,
            Instruction::AddX => 2,
        }
    }
}

impl Executable for Instruction {
    fn execute<'a, I>(&self, registers: &mut Registers, args: &mut I)
    where
        I: Iterator<Item = &'a str>
    {
        match *self {
            Instruction::Noop => (),
            Instruction::AddX => {
                let Some(arg) = args.next() else {
                    panic!("Expected 1 argument to addx, found none");
                };
                let arg = arg.parse::<i32>().unwrap_or_else(|e| panic!("Could not parse argument '{arg}' into a i32: {e}"));
                registers.add(Register::X, arg);
            }
        }
    }
}

fn task1(file: &mut File, cycles_to_sum: &mut BinaryHeap<Reverse<i32>>) {
    let mut registers = Registers::new();

    let mut current_cycle = 1i32;
    let mut total_signal_strength = 0i32;

    for_each_line(file, |i, line| {
        let mut split = line.split(" ");
        
        let Some(instruction) = split.next() else {
            panic!("Instruction not found on line {}", i + 1);
        };
        
        let instruction = Instruction::from_str(instruction).unwrap_or_else(|e| panic!("Unknown instruction '{instruction}': {e}"));
        
        // Registers have a default value of 1
        let old_register_x = *registers.get(Register::X).unwrap_or(&1);
        instruction.execute(&mut registers, &mut split.into_iter());
        let num_cycles = instruction.num_cycles();
        current_cycle += num_cycles;

        if let Some(Reverse(next_cycle_to_sum)) = cycles_to_sum.peek() {
            let register_x = registers.get(Register::X).unwrap();
            if current_cycle == *next_cycle_to_sum {
                total_signal_strength += (*register_x) * next_cycle_to_sum;
                cycles_to_sum.pop();
            } else if current_cycle > *next_cycle_to_sum {
                total_signal_strength += old_register_x * next_cycle_to_sum;
                cycles_to_sum.pop();
            }
        }
    });

    println!("Total signal strength: {total_signal_strength}");
}

fn task2(file: &mut File) {

}

#[tokio::main]
async fn main() {
    let mut file = aoc_get_file(INPUT_FILE_NAME, DAY).await.unwrap();

    let mut task1_cycles = BinaryHeap::from([20, 60, 100, 140, 180, 220].map(|v| Reverse(v)));

	task1(&mut file, &mut task1_cycles);
	// task2(&mut file);
}
