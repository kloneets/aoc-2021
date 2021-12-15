use std::collections::HashMap;
use std::io::{self, BufRead};
use std::{fs::File, path::Path};

fn main() {
    let data = read_data("./debug.txt".to_owned());
    let paired_data: Vec<Vec<&str>> = data
        .iter()
        .map(|s| {
            let pair: Vec<&str> = s.split("-").map(|s| s.trim()).collect();
            pair
        })
        .collect();
    println!("Data: {:?}", paired_data);
    let tree = build_nodes(paired_data);
    println!("tree {:?}", tree);
    let paths = walk_nodes("start", tree.clone(), vec![], 1);
    let finished: Vec<Vec<&str>> = paths
        .into_iter()
        .filter(|path| path[path.len() - 1] == "end")
        .collect();
    println!("Part 1. Paths count: {}", finished.len());
    let paths = walk_nodes("start", tree, vec![], 2);
    let finished: Vec<Vec<&str>> = paths
        .into_iter()
        .filter(|path| path[path.len() - 1] == "end")
        .collect();
    println!("Part 2. Paths count: {}", finished.len());
}

fn walk_nodes<'a>(
    node: &'a str,
    tree: HashMap<&str, Vec<&'a str>>,
    done: Vec<&'a str>,
    times: usize,
) -> Vec<Vec<&'a str>> {
    let mut paths: Vec<Vec<&'a str>> = vec![];
    let mut d = done.clone();
    d.push(node.clone());
    if node != "end" {
        let next = tree.get(node).unwrap();
        for next_node in next {
            let mut keyed = HashMap::new();
            for c in &d {
                if &c.to_uppercase().as_str() != c {
                    keyed.entry(c).or_insert(vec![]).push(c);
                }
            }
            let mut small_times = true;
            if keyed.contains_key(next_node) {
                small_times = keyed.get(next_node).unwrap().len() < times;
            }
            let mut no_parasites = true;
            for (_, v) in keyed {
                if v.len() >= times && times != 1 {
                    println!("parasite: {:?}, d {:?}", v, d);
                    no_parasites = false;
                    break;
                }
            }
            if &next_node.to_uppercase().as_str() == next_node || (small_times && no_parasites) {
                let mut p = walk_nodes(next_node, tree.clone(), d.clone(), times);
                paths.append(&mut p);
            }
        }
    }
    paths.push(d.clone());
    paths
}

fn build_nodes(data: Vec<Vec<&str>>) -> HashMap<&str, Vec<&str>> {
    let mut nodes: HashMap<&str, Vec<&str>> = HashMap::new();
    for d in data {
        if nodes.contains_key(d[0]) && d[1] != "start" {
            nodes.get_mut(d[0]).map(|p| p.push(d[1]));
        } else if d[1] != "start" {
            nodes.insert(d[0], vec![d[1]]);
        }

        if nodes.contains_key(d[1]) && d[0] != "start" {
            nodes.get_mut(d[1]).map(|p| p.push(d[0]));
        } else if d[0] != "start" {
            nodes.insert(d[1], vec![d[0]]);
        }
    }
    nodes
}

fn read_data(data_file: String) -> Vec<String> {
    let mut data: Vec<String> = vec![];
    if let Ok(lines) = read_lines(data_file) {
        for line in lines {
            if let Ok(d) = line {
                let d = d.trim().to_owned();
                if d != "" {
                    data.push(d);
                }
            }
        }
    }
    data
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
