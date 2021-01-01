use std::collections::HashMap;
use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    let mut ruleset: HashMap<usize, RuleMessage> = HashMap::new();
    let mut unverified_messages: Vec<(Vec<bool>, usize)> = vec![];

    let input_file = fs::read_to_string(FILENAME).expect("File I/O error");
    let mut input_file = input_file.lines();

    // Get ruleset
    while let Some(line) = input_file.next() {
        if line == "" {
            break;
        }

        let mut line = line.split(": ");
        let rule_name: usize = line.next().unwrap().parse().unwrap();
        let rule_contents = line.next().unwrap();
        if rule_contents.starts_with("\"") {
            // Char
            if rule_contents == "\"a\"" {
                ruleset.insert(rule_name, RuleMessage::Val(true));
            } else {
                ruleset.insert(rule_name, RuleMessage::Val(false));
            }
        } else {
            // Reference
            let mut xor_ref_list: Vec<Vec<usize>> = vec![];
            let mut rule_contents = rule_contents.split(" | ");
            while let Some(ref_list) = rule_contents.next() {
                let mut list_vec: Vec<usize> = vec![];
                for rule_ref in ref_list.split(" ") {
                    list_vec.push(rule_ref.parse().unwrap());
                }
                xor_ref_list.push(list_vec);
            }
            ruleset.insert(rule_name, RuleMessage::Rule(xor_ref_list));
        }
    }

    // Get unverified messages
    while let Some(line) = input_file.next() {
        let mut current_message: Vec<bool> = vec![];
        for c in line.chars() {
            if c == 'a' {
                current_message.push(true);
            } else {
                current_message.push(false);
            }
        }
        let current_len = current_message.len();
        unverified_messages.push((current_message, current_len));
    }

    println!(
        "Answer: {}",
        unverified_messages
            .iter_mut()
            // .filter(|msg| msg.1 == 24)
            .filter_map(|mut msg| {
                for c in msg.0.iter() {
                    if *c {
                        print!("a");
                    } else {
                        print!("b");
                    }
                }
                if check_message(&mut msg, 0, &ruleset, 15) {
                    println!(": okay");
                    Some(true)
                } else {
                    println!(": no good");
                    None
                }
            })
            .count()
    );
}

enum RuleMessage {
    Rule(Vec<Vec<usize>>),
    Val(bool),
}

fn check_message(
    msg: &mut (Vec<bool>, usize),
    rule: usize,
    ruleset: &HashMap<usize, RuleMessage>,
    mut search_depth: usize,
) -> bool {
    if search_depth > msg.1 {
        return false;
    } else if msg.0.is_empty() {
        return true;
    }

    match ruleset.get(&rule).unwrap() {
        RuleMessage::Rule(list_of_rule_lists) => {
            let mut list_of_rule_lists = list_of_rule_lists.iter();
            let mut okay = false;
            if let Some(ref_list) = list_of_rule_lists.next() {
                let mut msg_copy = msg.clone();
                for rule_ref in ref_list {
                    okay = check_message(&mut msg_copy, *rule_ref, &ruleset, search_depth);
                    if !okay {
                        break;
                    }
                }
                if okay {
                    *msg = msg_copy.clone();
                    return true;
                }
            }
            if let Some(ref_list) = list_of_rule_lists.next() {
                let mut msg_copy = msg.clone();
                for rule_ref in ref_list {
                    if *rule_ref == 8 || *rule_ref == 11 {
                        search_depth += 5
                    }
                    okay = check_message(&mut msg_copy, *rule_ref, &ruleset, search_depth);
                    if !okay {
                        break;
                    }
                }
                if okay {
                    *msg = msg_copy.clone();
                    return true;
                }
            }
        }
        RuleMessage::Val(actual_value) => {
            if msg.0[0] == *actual_value {
                msg.0.remove(0);
                return true;
            } else {
                return false;
            }
        }
    }
    false
}

// fn debug_print<'a, I>(messages: I)
// where
//     I: Iterator<Item = &'a Vec<bool>>,
// {
//     for (i, message) in messages.into_iter().enumerate() {
//         let message: String = message.iter().map(|c| if *c { 'a' } else { 'b' }).collect();
//         println!("{}: {}", i, message);
//     }
// }
