use std::collections::HashMap;
use std::fs;

const FILENAME: &str = "input.txt";
const WORDSIZE: usize = 8;

fn main() {
    let mut ruleset: HashMap<usize, RuleMessage> = HashMap::new();
    let mut unverified_messages: Vec<Vec<Vec<bool>>> = vec![];

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
        let mut msg: Vec<bool> = vec![];
        let mut line = line.chars();
        while let Some(c) = line.next() {
            if c == 'a' {
                msg.push(true);
            } else {
                msg.push(false);
            }
        }
        let msg = msg.chunks(WORDSIZE).map(|chunk| chunk.to_vec()).collect();

        unverified_messages.push(msg);
    }

    let mut ans = 0;
    for msg in unverified_messages.iter_mut() {
        if msg
            .iter_mut()
            .all(|substring| check_message(substring, 0, &ruleset))
        {
            ans += 1;
        }
    }

    println!("Ans: {}", ans);
}

enum RuleMessage {
    Rule(Vec<Vec<usize>>),
    Val(bool),
}

fn check_message(msg: &mut Vec<bool>, rule: usize, ruleset: &HashMap<usize, RuleMessage>) -> bool {
    if msg.is_empty() {
        return true;
    }

    match ruleset.get(&rule).unwrap() {
        RuleMessage::Rule(list_of_rule_lists) => {
            let mut list_of_rule_lists = list_of_rule_lists.iter();
            let mut okay = false;
            if let Some(ref_list) = list_of_rule_lists.next() {
                let mut msg_copy = msg.clone();
                for rule_ref in ref_list {
                    okay = check_message(&mut msg_copy, *rule_ref, &ruleset);
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
                    okay = check_message(&mut msg_copy, *rule_ref, &ruleset);
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
            if msg[0] == *actual_value {
                msg.remove(0);
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
