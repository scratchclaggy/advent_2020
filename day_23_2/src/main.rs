use std::collections::LinkedList;

fn main() {
    let input = "364297581";
    let mut cups: LinkedList<u32> = input.chars().map(|num| num.to_digit(10).unwrap()).collect();

    for i in 10..1_000_001 {
        cups.push_back(i);
    }

    for _ in 0..10_000_000 {
        crab_cups(&mut cups);
    }

    cups.pop_front();
    let x = cups.pop_front().unwrap();
    let y = cups.pop_front().unwrap();

    println!("Ans: {}", x * y);
}

fn crab_cups(cups: &mut LinkedList<u32>) {
    let current = cups.pop_front().unwrap();
    let mut picked_up = LinkedList::<u32>::new();
    for _ in 0..3 {
        picked_up.push_back(cups.pop_front().unwrap());
    }

    let mut destination = current;
    destination = destination.saturating_sub(1);
    if destination == 0 {
        destination = 1_000_000;
    }

    while picked_up.contains(&destination) {
        destination = destination.saturating_sub(1);
        if destination == 0 {
            destination = 1_000_000;
        }
    }

    let destination = cups.iter().position(|&x| x == destination).unwrap();
    let mut second_half = cups.split_off(destination);
    cups.append(&mut picked_up);
    cups.append(&mut second_half);
    cups.push_back(current);
}
