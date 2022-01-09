use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
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

    fn is_start(&self) -> bool {
        self.name == "start"
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

#[derive(Clone, Default)]
struct Path<'a> {
    nodes: Vec<&'a Node>,
    has_2_small: Option<&'a Node>,
}

impl<'a> Path<'a> {
    fn last(&self) -> &'a Node {
        self.nodes.last().unwrap()
    }

    fn push(&mut self, node: &'a Node) {
        if self.nodes.len() == 20 {
            println!("{:?}", self.nodes);
        }
        if node.is_small() {
            if self.nodes.contains(&node) {
                assert!(self.has_2_small.is_none());
                self.has_2_small = Some(node);
            }
        }
        self.nodes.push(node);
    }
}

fn find_all_paths_1(graph: &Graph, from: &Path) -> u64 {
    if from.last().is_end() {
        return 1;
    }

    let nexts = graph.edges.get(from.last()).unwrap();
    let mut count = 0;
    for next in nexts {
        if next.is_start() {
            continue;
        }
        if next.is_small() {
            if from.nodes.contains(&next) {
                continue;
            }
        }
        let mut new_from = from.clone();
        new_from.push(next);
        count += find_all_paths_1(graph, &new_from);
    }
    count
}

fn part1(filename: &str) {
    println!("{}", filename);
    let graph = Graph::parse(filename);
    let mut path = Path::default();
    let start = Node {
        name: "start".to_owned(),
    };
    path.push(&start);
    let count = find_all_paths_1(&graph, &path);
    println!("{}", count);
    assert!(count == 10 || count == 19 || count == 226 || count == 5576);
}

fn find_all_paths_2(graph: &Graph, from: &Path) -> u64 {
    if from.last().is_end() {
        return 1;
    }

    let nexts = graph.edges.get(from.last()).unwrap();
    let mut count = 0;
    for next in nexts {
        if next.is_start() {
            continue;
        }
        if next.is_small() {
            if let Some(node) = from.has_2_small {
                if node == next || from.nodes.contains(&next) {
                    continue;
                }
            }
        }
        let mut new_from = from.clone();
        new_from.push(next);
        count += find_all_paths_2(graph, &new_from);
    }
    count
}

fn part2(filename: &str) {
    println!("{}", filename);
    let graph = Graph::parse(filename);
    let mut path = Path::default();
    let start = Node {
        name: "start".to_owned(),
    };
    path.push(&start);
    let count = find_all_paths_2(&graph, &path);
    println!("{}", count);
    assert!(count == 36 || count == 103 || count == 3509 || count == 152837);
}

fn main() {
    println!("Part 1");
    part1("day12-input-test1.txt");
    part1("day12-input-test2.txt");
    part1("day12-input-test3.txt");
    part1("day12-input.txt");

    println!();
    println!("Part 2");
    part2("day12-input-test1.txt");
    part2("day12-input-test2.txt");
    part2("day12-input-test3.txt");
    part2("day12-input.txt");
}
