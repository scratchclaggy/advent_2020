use std::collections::HashMap;
use std::fs;

const FILENAME: &str = "ex.txt";

fn main() {
    let input = fs::read_to_string(FILENAME).expect("Error: could not open file");

    let new_passport = |input_string: &str| {
        let mut passport: HashMap<&str, &str> = HashMap::new();
        for field in input_string.split(|c| c == ' ' || c == '\n') {
            let kv_pair: Vec<&str> = field.split(':').collect();
            passport.insert(kv_pair[0], kv_pair[1]);
        }

        passport
    };

    let is_okay = |passport: &HashMap<&str, &str>| {
        let mut okay = false;
        if passport.contains_key("byr")
            && passport.contains_key("iyr")
            && passport.contains_key("eyr")
            && passport.contains_key("hgt")
            && passport.contains_key("hcl")
            && passport.contains_key("ecl")
            && passport.contains_key("pid")
        {
            okay = true;
        }

        okay
    };

    let answer: usize = input
        .split("\r\n\r\n")
        .map(|s| new_passport(s))
        .filter(|p| is_okay(p))
        .count();

    println!("The answer is: {}\n", answer);

    println!("END OF Program\n");
}

// type Passport<'a> = HashMap<&'a str, &'a str>;

// impl<'a> Passport<'a> {
//     fn new(passport_string: &str) -> Passport {
//         Passport {}
//     }
// }
