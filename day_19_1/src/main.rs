use std::collections::HashMap;
use std::fs;

const FILENAME: &str = "input.txt";
fn main() {
    let input_file = fs::read_to_string(FILENAME).expect("File I/O error");
    let mut ruleset: HashMap<usize, RuleMessage> = HashMap::new();
    let mut input_file = input_file.lines();
    while let Some(line) = input_file.next() {
        if line == "/r/n" {
            break;
        }

        let mut line = line.split(": ");
        let rule_index: usize = line.next().unwrap().parse().unwrap();
        let rule_contents = line.next().unwrap();
        if rule_contents.starts_with("\"") {
            // Actual value
            if rule_contents == "\"a\"" {
                ruleset.insert(rule_index, RuleMessage::Val(false));
            } else {
                ruleset.insert(rule_index, RuleMessage::Val(true));
            }
        } else {
            // Sub-rules
            let mut rule_halves: Vec<Vec<usize>> = vec![vec![]];
            let mut rule_contents = rule_contents.split(" | ");
            while let Some(rule_half) = rule_contents.next() {
                let mut half: Vec<usize> = vec![];
                for rule_reference in rule_half.split(" ") {
                    half.push(rule_reference.parse().unwrap());
                }
                rule_halves.push(half);
            }
            ruleset.insert(rule_index, RuleMessage::Rule(rule_halves));
        }
    }

    println!("All done");
}

enum RuleMessage {
    Rule(Vec<Vec<usize>>),
    Val(bool),
}

fn expand_ruleset(
    rule: usize,
    ruleset: &HashMap<usize, RuleMessage>,
    messages: &mut Vec<Vec<bool>>,
) {
    match ruleset.get(&rule).unwrap() {
        RuleMessage::Rule(current_rule) => {
            // Recursive component
            // Expand the messages accoring to the current rule
            // If there is an alternative rule, give a copy and append after expansion
            let mut new_messages: Vec<Vec<bool>> = vec![vec![]];
            while let Some(sub_rule) = current_rule.iter().next() {
                let mut this_branch_messages = messages.clone();
                for recursive_rule in sub_rule {
                    expand_ruleset(*recursive_rule, &ruleset, &mut this_branch_messages);
                }
                new_messages.append(&mut this_branch_messages);
            }
            *messages = new_messages;
        }
        RuleMessage::Val(actual_value) => {
            // Base case
            // Append the value to all messages
            if messages.is_empty() {
                messages.push(Vec::new());
                messages[0].push(*actual_value);
            } else {
                for message in messages {
                    message.push(*actual_value);
                }
            }
        }
    }
}
