use std::collections::HashSet;
use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    let input_file = fs::read_to_string(FILENAME).expect("FILE ERROR");
    let mut input_file = input_file.lines().into_iter();
    let mut rulebook: Vec<Rule> = vec![];
    let mut unknown_fields: HashSet<u32> = HashSet::new();
    let mut ticket_vec: Vec<Ticket> = vec![];
    let i = 0;
    while let Some(line) = input_file.next() {
        if line == "" {
            break;
        }

        let ticket = line.split(": ");
        let ticket_field = ticket.next().unwrap();
        let field_range: Vec<u32> = ticket
            .next()
            .unwrap()
            .split(" or ")
            .map(|s: &str| s.split("-"))
            .map(|num| (num).parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        rulebook.push(Rule::new(
            ticket_field,
            field_range[0],
            field_range[1],
            field_range[2],
            field_range[3],
        ));
        unknown_fields.insert(i);
    }

    let mut input_file = input_file.skip(4);

    while let Some(line) = input_file.next() {}
}

struct Rule<'a> {
    ticket_field: &'a str,
    range_1: Range,
    range_2: Range,
}

impl<'a> Rule<'a> {
    fn new(ticket_field: &'a str, min_1: u32, max_1: u32, min_2: u32, max_2: u32) -> Rule<'a> {
        Rule {
            ticket_field,
            range_1: (min_1, max_1),
            range_2: (min_2, max_2),
        }
    }
}

type Range = (u32, u32);

fn in_range(val: u32, range: Range) -> bool {
    val >= range.0 && val <= range.1
}

type Ticket = Vec<u32>;
