use std::collections::HashMap;
use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    // let input = fs::read_to_string(FILENAME).expect("FILE ERROR");
    // let program: Vec<Task> = input
    //     .lines()
    //     .map(|line| {
    //         let mut input_string = line.split(" = ");
    //         let task_type = input_string.next().unwrap();
    //         let new_task: Task = match task_type {
    //             "mask" => Task::BitMask(Mask::new(input_string.next().unwrap())),
    //             _ => Task::Allocation(
    //                 task_type
    //                     .split(|c| c == '[' || c == ']')
    //                     .skip(1)
    //                     .next()
    //                     .unwrap()
    //                     .parse::<u64>()
    //                     .unwrap(),
    //                 input_string.next().unwrap().trim().parse::<u64>().unwrap(),
    //             ),
    //         };
    //         new_task
    //     })
    //     .collect();

    // let mut current_mask: Mask = Mask::new("0");
    // let mut memory: HashMap<u64, u64> = HashMap::new();
    // for instruction in program {
    //     match instruction {
    //         Task::Allocation(address, value) => {
    //             // memory.insert(address, current_mask.apply_mask(value));
    //             //println!("{}: {}", address, current_mask.apply_mask(value));
    //         }
    //         Task::BitMask(next_mask) => current_mask = next_mask,
    //     }
    // }

    // println!("{}", memory.values().sum::<u64>());
}

struct Mask {
    bit_mask: u64,
    x_index: Vec<usize>,
    max: u64,
}

impl Mask {
    fn new(input_string: &str) -> Mask {
        let mut x_index: Vec<usize> = Vec::new();
        let mut string_no_x: Vec<char> = Vec::new();
        for (i, c) in input_string.chars().enumerate() {
            if c == 'X' {
                string_no_x.push('0');
                x_index.push(i);
            } else {
                string_no_x.push(c);
            }
        }

        let bit_mask = string_no_x.into_iter().collect::<String>().parse().unwrap();
        let max = vec!["1"; x_index.len()]
            .into_iter()
            .collect::<String>()
            .parse()
            .unwrap();

        Mask {
            bit_mask,
            x_index,
            max,
        }
    }

    fn decode_address(&self, mut input_address: u64) -> Vec<u64> {
        // Apply mask to input_address
        input_address = input_address | self.bit_mask;

        // Create vector to store adress_set
        let mut address_set: Vec<u64> = vec![];

        // From 0 - 2(n)-1 go through each permutation
        for permutation in 0..self.max {
            let mut current_permutation = 0;
            // For each bit position in the permutation
            for bit_position in 0..self.x_index.len() {
                // Mask the current position's bit
                let mut floating_bit = permutation & (1 << bit_position);

                if floating_bit == 1 {
                    // Shift bit to the necessary position
                    floating_bit = floating_bit << self.x_index[bit_position];
                    current_permutation = input_address | floating_bit;
                } else {
                    floating_bit = floating_bit << self.x_index[bit_position];
                    current_permutation = input_address & (!floating_bit);
                }
            }

            address_set.push(current_permutation);
        }

        // Return the set
        address_set
    }
}

enum Task {
    BitMask(Mask),
    Allocation(u64, u64),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask_test() {
        let mask = Mask::new("000000000000000000000000000000X1001X");
        let address = 42;
        let address_set = mask.decode_address(address);
        assert_eq!(address_set, [26, 27, 58, 59]);
    }
}
