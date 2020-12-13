use std::fs;

const FILENAME: &str = "ex.txt";
fn main() {
    // Get input
    let mut joltages = fs::read_to_string(FILENAME)
        .expect("FILE ERROR")
        .lines()
        .map(|num| num.trim().parse().expect("COULD NOT PARSE"))
        .collect::<Vec<u32>>();
    joltages.sort();
    let mut pathways = 1;

    for i in 0..joltages.len() - 3 {
        if joltages[i + 3] - joltages[i] <= 3 {
            pathways += 3;
        } else if joltages[i + 2] - joltages[i] <= 3 {
            pathways += 2;
        }
    }

    println!("Answer: {}", pathways);
}
