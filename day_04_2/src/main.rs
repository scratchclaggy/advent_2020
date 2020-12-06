use std::collections::HashMap;
use std::fs;

const FILENAME: &str = "ex.txt";

fn main() {
    let input = fs::read_to_string(FILENAME).expect("Error: could not open file");

    let answer: usize = input
        .split("\r\n\r\n")
        .filter(|s| Passport::new(s).is_okay())
        .count();

    println!("The answer is: {}\n", answer);

    println!("END OF Program\n");
}

struct Passport<'a> {
    field: HashMap<&'a str, &'a str>,
}

impl<'a> Passport<'a> {
    fn new(passport_string: &'a str) -> Passport {
        let mut field: HashMap<&str, &str> = HashMap::new();
        for entry in passport_string.split(|c| c == ' ' || c == '\n') {
            let kv_pair: Vec<&str> = entry.split(':').collect();
            field.insert(kv_pair[0], kv_pair[1]);
        }
        Passport { field }
    }

    fn is_okay(&self) -> bool {
        println!("byr: {}", self.byr_okay());
        println!("iyr: {}", self.iyr_okay());
        println!("eyr: {}", self.eyr_okay());
        println!("hgt: {}", self.hgt_okay());
        println!("hcl: {}", self.hcl_okay());
        println!("ecl: {}", self.ecl_okay());
        println!("pid: {}", self.pid_okay());
        println!();

        if self.byr_okay()
            && self.iyr_okay()
            && self.eyr_okay()
            && self.hgt_okay()
            && self.hcl_okay()
            && self.ecl_okay()
            && self.pid_okay()
        {
            return true;
        }
        false
    }

    fn byr_okay(&self) -> bool {
        if let Some(byr) = self.field.get("byr") {
            let byr: u32 = match byr.parse() {
                Ok(byr) => byr,
                Err(_) => 0,
            };

            if byr >= 1920 && byr <= 2002 {
                return true;
            }
        }

        false
    }

    fn iyr_okay(&self) -> bool {
        if let Some(iyr) = self.field.get("iyr") {
            let iyr: u32 = match iyr.parse() {
                Ok(iyr) => iyr,
                Err(_) => 0,
            };

            if iyr >= 1920 && iyr <= 2002 {
                return true;
            }
        }

        false
    }

    fn eyr_okay(&self) -> bool {
        if let Some(eyr) = self.field.get("eyr") {
            let eyr: u32 = match eyr.parse() {
                Ok(eyr) => eyr,
                Err(_) => 0,
            };
            if eyr >= 2020 && eyr <= 2030 {
                return true;
            }
        }

        false
    }

    fn hgt_okay(&self) -> bool {
        if let Some(hgt) = self.field.get("hgt") {
            if hgt.ends_with("cm") {
                let hgt: u32 = match hgt[..3].parse() {
                    Ok(hgt) => hgt,
                    Err(_) => 0,
                };

                if hgt >= 150 && hgt <= 190 {
                    return true;
                }
            }

            if hgt.ends_with("in") {
                let hgt: u32 = match hgt[..2].parse() {
                    Ok(hgt) => hgt,
                    Err(_) => 0,
                };

                if hgt >= 59 && hgt <= 76 {
                    return true;
                }
            }
        }

        false
    }

    fn hcl_okay(&self) -> bool {
        if let Some(hcl) = self.field.get("hcl") {
            return match i64::from_str_radix(&hcl[1..], 16) {
                Ok(_) => true,
                Err(_) => false,
            };
        }

        false
    }

    fn ecl_okay(&self) -> bool {
        if let Some(ecl) = self.field.get("ecl") {
            return "amb blu brn gry grn hzl oth".contains(ecl);
        }
        false
    }

    fn pid_okay(&self) -> bool {
        if let Some(pid) = self.field.get("pid") {
            if pid.len() == 9 {
                return pid.chars().all(char::is_numeric);
            }
        }
        false
    }
}
