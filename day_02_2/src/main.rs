use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    let count = fs::read_to_string(FILENAME)
        .expect("Some error with file")
        .lines()
        .filter(|line| PwdTest::new(line).is_okay())
        .count();

    println!("{} passwords meet the requirements", count);
}

struct PwdTest<'a> {
    pass: &'a str,
    c: char,
    index_1: usize,
    index_2: usize,
}

impl<'a> PwdTest<'a> {
    fn new(line: &str) -> PwdTest {
        let v: Vec<&str> = line
            .split(|delim| delim == '-' || delim == ':' || delim == ' ')
            .filter(|s| !s.is_empty())
            .collect();

        let pass = v[3];
        let c = v[2].parse().expect("could not parse to char");
        let index_1: usize = v[0].parse().expect("could not parse to usize");
        let index_2: usize = v[1].parse().expect("could not parse to usize");

        let index_1 = index_1 - 1;
        let index_2 = index_2 - 1;

        PwdTest {
            pass,
            c,
            index_1,
            index_2,
        }
    }

    fn is_okay(&self) -> bool {
        let char_indices = self.pass.as_bytes();
        if (char_indices[self.index_1] == self.c as u8)
            ^ (char_indices[self.index_2] == self.c as u8)
        {
            return true;
        }

        false
    }
}
