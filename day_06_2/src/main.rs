use std::collections::HashSet;
use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    let groups = fs::read_to_string(FILENAME).expect("Error: could not open file");
    let groups: Vec<&str> = groups.split("\r\n\r\n").collect();

    let mut total = 0;

    for group in groups {
        let group = group.lines().map(|s| s.trim());
        let mut leader_said_yes: HashSet<char> = HashSet::new();
        let mut member_said_yes: HashSet<char> = HashSet::new();
        let mut all_members: HashSet<_>;
        if let Some(leader) = group.next() {
            for c in leader.chars() {
                leader_said_yes.insert(c);
            }
        };
        while let Some(member) = group.next() {
            for c in member.chars() {
                member_said_yes.insert(c);
            }

            let all_members: HashSet<char> = all_members.intersection(&member_said_yes).collect();
        }

        total += leader_said_yes.len();
    }

    println!("The answer is: {}\n", total);

    println!("END OF Program\n");
}
