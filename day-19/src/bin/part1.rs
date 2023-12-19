use std::{collections::HashMap, time::Instant};

enum Rule {
    ComparisonRule(ComparisonRule),
    DefaultRule(Destination),
}

impl Rule {
    fn next(&self, part: &Part) -> Option<&Destination> {
        match self {
            Self::ComparisonRule(comparison) => {
                let value = match comparison.field.as_str() {
                    "x" => part.x,
                    "m" => part.m,
                    "a" => part.a,
                    "s" => part.s,
                    _ => panic!("Invalid field: {}", comparison.field),
                };
                if value > comparison.min && value < comparison.max {
                    Some(&comparison.next)
                } else {
                    None
                }
            }
            Self::DefaultRule(dest) => Some(dest),
        }
    }
}

struct ComparisonRule {
    field: String,
    min: i64,
    max: i64,
    next: Destination,
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        if let Some((comparison, next)) = s.split_once(':') {
            if let Some((field, value)) = comparison.split_once('<') {
                Self::ComparisonRule(ComparisonRule {
                    field: field.to_string(),
                    min: i64::MIN,
                    max: value.parse().unwrap(),
                    next: next.into(),
                })
            } else if let Some((field, value)) = comparison.split_once('>') {
                Self::ComparisonRule(ComparisonRule {
                    field: field.to_string(),
                    min: value.parse().unwrap(),
                    max: i64::MAX,
                    next: next.into(),
                })
            } else {
                panic!("Invalid comparison rule: {}", s);
            }
        } else {
            Self::DefaultRule(s.into())
        }
    }
}

enum Destination {
    A,
    R,
    Next(String),
}

impl From<&str> for Destination {
    fn from(s: &str) -> Self {
        match s {
            "A" => Self::A,
            "R" => Self::R,
            _ => Self::Next(s.to_string()),
        }
    }
}

struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Part {
    fn sum(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}

fn solve(file: &str) -> i64 {
    let mut workflow: HashMap<String, Vec<Rule>> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();

    //Parse the input
    let sections: Vec<_> = file.split("\n\n").collect();
    sections[0].lines().for_each(|l| {
        let (name, rules) = l.split_once('{').unwrap();
        let rules = rules.trim_end_matches('}');
        let rules: Vec<_> = rules.split(',').map(Rule::from).collect();
        workflow.insert(name.to_string(), rules);
    });
    sections[1].lines().for_each(|l| {
        let l = l.trim_start_matches('{');
        let l = l.trim_end_matches("}");
        let attr = l.split(',');
        let mut part = Part {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };
        attr.for_each(|p| {
            let (field, value) = p.split_once('=').unwrap();
            match field {
                "x" => part.x = value.parse().unwrap(),
                "m" => part.m = value.parse().unwrap(),
                "a" => part.a = value.parse().unwrap(),
                "s" => part.s = value.parse().unwrap(),
                _ => panic!("Invalid field: {}", field),
            }
        });
        parts.push(part);
    });

    let mut acc = 0;
    for part in parts {
        let mut current = "in";

        'workflowloop: loop {
            'ruleloop: for rule in workflow.get(current).unwrap() {
                match rule.next(&part) {
                    Some(Destination::A) => {
                        acc += part.sum();
                        break 'workflowloop;
                    }
                    Some(Destination::R) => break 'workflowloop,
                    Some(Destination::Next(next)) => {
                        current = next;
                        break 'ruleloop;
                    }
                    None => (),
                }
            }
        }
    }
    acc
}

fn main() {
    let input = include_str!("input.txt");
    println!("Starting solution");
    let t0 = Instant::now();
    let result = solve(input);
    let duration = t0.elapsed();
    println!("Result: {}", result);
    println!("Time: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whole_part() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!(solve(input), 19114);
    }
}
