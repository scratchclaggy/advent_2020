use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};


#[derive(Debug, Clone)]
enum Rule {
    Composite(Vec<Vec<usize>>),
    Base(String),
}

impl Rule{
    fn matches<'a>(&self, line: &'a str, others: &HashMap<usize, Rule>)->Option<&'a str>{
        match self{
            Rule::Base(arg) => {
                let (c, rest) = line.split_at(1);
                if c == arg{
                    Some(rest)
                }else{
                    None
                }
            },
            Rule::Composite(rules)=> rules.iter().find_map(|rule|{
                let mut line = Some(line);
                let mut rules = rule.iter();
                while let Some(l) = line{
                    if let Some(rule) = rules.next(){
                        line = others.get(rule).unwrap().matches(l, others);
                    }else{
                        break;
                    }
                }
                line
            })
        }


    }
}

fn build_rules(lines: &mut impl Iterator<Item = impl AsRef<str>>) -> HashMap<usize, Rule> {
    let mut ret = HashMap::new();

    for line in lines {
        let mut parts = line.as_ref().split(':');
        let num = parts.next().and_then(|part| part.trim().parse().ok());
        if num.is_none() {
            return ret;
        }
        let num = num.unwrap();
        let rules = parts.next();
        if rules.is_none() {
            return ret;
        }
        let rules = rules.unwrap().trim();
        if let Some(val) = rules
            .strip_suffix('"')
            .and_then(|rules| rules.strip_prefix('"'))
        {
            ret.insert(num, Rule::Base(val.to_string()));
        } else {
            ret.insert(
                num,
                Rule::Composite(
                    rules
                        .split("|")
                        .map(|rule| {
                            rule.split_whitespace()
                                .map(|part| {
                                    part.parse().unwrap()
                                })
                                .collect()
                        })
                        .collect(),
                ),
            );
        }
    }
    ret
}

fn main() {
    let file = File::open("input.txt").map(BufReader::new).unwrap();
    let mut lines = file.lines().map(|line| line.unwrap());
    let rules = build_rules(&mut lines);
    let rule_to_check = rules.get(&0).unwrap();
    let result = lines.filter(|line| 
        rule_to_check.matches(line, &rules)
            .map(|rest| rest.len() == 0).unwrap_or(false)
    ).count();
    println!("answer is {}", result);
}


#[cfg(test)]
mod tests{
    use super::*;

    const INPUT: &[&'static str] = &[
        "0: 4 1 5",
        "1: 2 3 | 3 2",
        "2: 4 4 | 5 5",
        "3: 4 5 | 5 4",
        "4: \"a\"",
        "5: \"b\"",
        "",
        "ababbb",
        "bababa",
        "abbbab",
        "aaabbb",
        "aaaabbb",
    ];

    #[test]
    fn test(){
        let mut lines = INPUT.iter();
        let rules = build_rules(&mut lines);

        let rule_to_check = rules.get(&0).unwrap();
        let result = lines.filter(|&&line| 
            rule_to_check.matches(line, &rules)
                .map(|rest| rest.len() == 0).unwrap_or(false)
        ).count();
        assert_eq!(result, 2);
    }

}