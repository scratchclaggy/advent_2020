use std::collections::HashMap;
use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    let input = fs::read_to_string(FILENAME).expect("FILE ERROR");
    let program: Vec<Task> = input
        .lines()
        .map(|line| {
            let mut input_string = line.split(" = ");
            let task_type = input_string.next().unwrap();
            let new_task: Task = match task_type {
                "mask" => Task::BitMask(Mask::new(input_string.next().unwrap())),
                _ => Task::Allocation(
                    task_type
                        .split(|c| c == '[' || c == ']')
                        .skip(1)
                        .next()
                        .unwrap()
                        .parse::<u64>()
                        .unwrap(),
                    input_string.next().unwrap().trim().parse::<u64>().unwrap(),
                ),
            };
            new_task
        })
        .collect();

    let mut current_mask: Mask = Mask::new("0");
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for instruction in program {
        match instruction {
            Task::Allocation(address, value) => {
                memory.insert(address, current_mask.apply_mask(value));
                //println!("{}: {}", address, current_mask.apply_mask(value));
            }
            Task::BitMask(next_mask) => current_mask = next_mask,
        }
    }

    println!("{}", memory.values().sum::<u64>());
}

struct Mask<'a> {
    mask_string: &'a str,
    x_index: Vec<usize>,
    max: u64,
}

impl<'a> Mask<'a> {
    fn new(mask_string: &'a str) -> Mask {
        let x_index: Vec<usize> = Vec::new();
        for (i, c) in mask_string.chars().enumerate() {
            if c == 'X' {
                x_index.push(i);
            }
        }
        let max_string = String::new();
        for i in 0..x_index.len() {
            max_string.push('1');
        }
        let max = max_string.parse().unwrap();

        Mask {
            mask_string,
            x_index,
            max,
        }
    }

    fn decode_address(&self, address: u64) -> Vec<u64> {
        let addresses: Vec<u64> = vec![];
        for bit in 0..self.max {
            for i in self.x_index {}
        }

        addresses
    }
}

enum Task {
    BitMask(Mask),
    Allocation(u64, u64),
}
