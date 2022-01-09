use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

#[derive(Clone, Hash, Eq, PartialEq)]
struct Node {
    name: String,
}

impl Node {
    fn new(name: &str) -> Node {
        assert!(
            name.bytes().all(|b| b.is_ascii_lowercase())
                || name.bytes().all(|b| b.is_ascii_uppercase())
        );
        Node {
            name: name.to_owned(),
        }
    }

    fn is_end(&self) -> bool {
        self.name == "end"
    }

    fn is_small(&self) -> bool {
        self.name.bytes().next().unwrap().is_ascii_lowercase()
    }
}

struct Graph {
    edges: HashMap<Node, HashSet<Node>>,
}

impl Graph {
    fn parse(filename: &str) -> Graph {
        let content = fs::read_to_string(filename).unwrap();
        let mut edges = HashMap::<Node, HashSet<Node>>::new();
        for line in content.lines() {
            let parts: Vec<_> = line.split("-").collect();
            assert_eq!(2, parts.len());
            let a = Node::new(parts[0]);
            let b = Node::new(parts[1]);
            edges.entry(a.clone()).or_default().insert(b.clone());
            edges.entry(b.clone()).or_default().insert(a.clone());
        }
        Graph { edges }
    }
}

fn find_all_paths(graph: &Graph, from: &Vec<&Node>) -> u64 {
    let last = from.last().unwrap();
    if last.is_end() {
        return 1;
    }

    let nexts = graph.edges.get(last).unwrap();
    let mut count = 0;
    for next in nexts {
        if next.is_small() {
            if from.contains(&next) {
                continue;
            }
        }
        let mut new_from = from.clone();
        new_from.push(next);
        count += find_all_paths(graph, &new_from);
    }
    count
}

fn part1(filename: &str) {
    println!("{}", filename);
    let graph = Graph::parse(filename);
    let mut path = Vec::new();
    let start = Node {
        name: "start".to_owned(),
    };
    path.push(&start);
    let count = find_all_paths(&graph, &path);
    println!("{}", count);
    assert!(count == 10 || count == 19 || count == 226 || count == 5576);
}

fn main() {
    part1("day12-input-test1.txt");
    part1("day12-input-test2.txt");
    part1("day12-input-test3.txt");
    part1("day12-input.txt");
}
