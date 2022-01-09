use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

struct Rule {
    input: (char, char),
    output: char,
}

impl Rule {
    fn parse(s: &str) -> Rule {
        let mut parts = s.split(" -> ");
        let input = parts.next().unwrap();
        assert_eq!(2, input.len());
        let mut input = input.chars();
        let input_0 = input.next().unwrap();
        let input_1 = input.next().unwrap();
        let input = (input_0, input_1);

        let output = parts.next().unwrap();
        assert_eq!(1, output.len());
        let mut output = output.chars().next().unwrap();

        assert!(parts.next().is_none());
        Rule { input, output }
    }
}

struct Input {
    template: String,
    rules: Vec<Rule>,
    rules_map: HashMap<(char, char), char>,
}

impl Input {
    fn parse(filename: &str) -> Input {
        let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let mut lines = content.lines();
        let template = lines.next().unwrap().to_owned();
        assert_eq!("", lines.next().unwrap());
        let mut rules = Vec::new();
        for line in lines {
            rules.push(Rule::parse(line));
        }

        let rules_map =
            HashMap::<(_, _), char>::from_iter(rules.iter().map(|r| (r.input, r.output)));
        assert_eq!(rules.len(), rules_map.len(), "Duplicate rules");

        Input {
            template,
            rules,
            rules_map,
        }
    }
}

fn part1(filename: &str) {
    println!("{}", filename);
    let input = Input::parse(filename);
    let mut template = input.template;
    for i in 0..10 {
        let mut new_template = String::new();

        for (a, b) in template.chars().zip(template.chars().skip(1)) {
            new_template.push(a);
            if let Some(c) = input.rules_map.get(&(a, b)) {
                new_template.push(*c);
            }
        }

        new_template.push(template.chars().last().unwrap());

        template = new_template;
        // println!("{} {}", i, template);
    }

    println!("{}", template.len());

    let mut counts = BTreeMap::new();
    for c in template.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    println!("{:?}", counts);

    let min = counts.iter().min_by_key(|(_, c)| *c).unwrap().1;
    let max = counts.iter().max_by_key(|(_, c)| *c).unwrap().1;
    println!("{}", max - min);
}

fn main() {
    part1("day14-input-test.txt");
    part1("day14-input.txt");
}
