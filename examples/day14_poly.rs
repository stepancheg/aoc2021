use std::collections::BTreeMap;
use std::collections::HashMap;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Key {
    bounds: (char, char),
    depth: usize,
}

#[derive(Default)]
struct LetterStats {
    counts: HashMap<char, usize>,
}

impl LetterStats {
    fn add(&mut self, c: char) {
        *self.counts.entry(c).or_insert(0) += 1;
    }

    fn add_all(&mut self, s: &LetterStats) {
        for (c, count) in s.counts.iter() {
            *self.counts.entry(*c).or_insert(0) += *count;
        }
    }
}

fn find_between<'a>(
    rule_map: &HashMap<(char, char), char>,
    key: &Key,
    memo: &'a mut HashMap<Key, LetterStats>,
) -> &'a LetterStats {
    // println!("find_between {:?}", key);
    if let Some(_stats) = memo.get(key) {
        return memo.get(key).unwrap();
    }
    if key.depth == 0 {
        memo.insert(key.clone(), LetterStats::default());
        return memo.get(key).unwrap();
    }
    let mut stats = LetterStats::default();
    if let Some(&x) = rule_map.get(&key.bounds) {
        stats.add(x);
        stats.add_all(find_between(
            rule_map,
            &Key {
                bounds: (key.bounds.0, x),
                depth: key.depth - 1,
            },
            memo,
        ));
        stats.add_all(find_between(
            rule_map,
            &Key {
                bounds: (x, key.bounds.1),
                depth: key.depth - 1,
            },
            memo,
        ));
    }
    memo.insert(key.clone(), stats);
    memo.get(key).unwrap()
}

fn part1(filename: &str) {
    println!("{}", filename);
    let input = Input::parse(filename);
    let mut template = input.template;

    let mut memo: HashMap<Key, LetterStats> = HashMap::new();

    for depth in [10, 40] {
        let mut letter_stats = LetterStats::default();
        for c in template.chars() {
            letter_stats.add(c);
        }
        for (a, b) in template.chars().zip(template.chars().skip(1)) {
            let key = Key {
                bounds: (a, b),
                depth,
            };
            let stats = find_between(&input.rules_map, &key, &mut memo);
            letter_stats.add_all(stats);
        }
        println!("{}", depth);
        println!("{:?}", letter_stats.counts);
        let min = letter_stats
            .counts
            .iter()
            .min_by_key(|(_, c)| *c)
            .unwrap()
            .1;
        let max = letter_stats
            .counts
            .iter()
            .max_by_key(|(_, c)| *c)
            .unwrap()
            .1;
        println!("{}", max - min);
    }
}

fn main() {
    part1("day14-input-test.txt");
    part1("day14-input.txt");
}
