use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

#[derive(Debug)]
struct Network {
    instructions: String,
    nodes: HashMap<String, Node>,
}

impl Network {
    fn new(input: &str) -> Self {
        let mut nodes: HashMap<String, Node> = HashMap::new();
        let instructions = input.lines().nth(0).unwrap().to_string();
        let lines = input.lines().skip(2).collect::<Vec<_>>();

        for line in lines {
            let parts = line.split(" = ").collect::<Vec<&str>>();
            let children = parts[1]
                .trim_matches(|c| c == '(' || c == ')')
                .split(", ")
                .collect::<Vec<&str>>();
            let id = parts[0].trim();
            let left = children[0];
            let right = children[1];
            nodes.insert(
                id.to_string(),
                Node {
                    left: left.to_string(),
                    right: right.to_string(),
                },
            );
        }

        Network {
            instructions,
            nodes,
        }
    }

    fn navigate(&self, start: &str, finish: &str) -> u32 {
        let mut curr = start.to_string();
        let mut steps = 0;
        let mut node_found = false;

        while !node_found {
            for c in self.instructions.chars() {
                let node = &self.nodes[&curr];
                if c == 'R' {
                    curr = node.right.clone();
                } else if c == 'L' {
                    curr = node.left.clone();
                }
                if curr == finish {
                    node_found = true
                }
                steps += 1;
            }
        }

        steps
    }
}

fn process(_input: &str) -> u32 {
    let network = Network::new(_input);
    let steps: u32 = network.navigate("AAA", "ZZZ");

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_one() {
        let input = "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)";
        assert_eq!(2, process(input));
    }

    #[test]
    fn test_case_two() {
        let input = "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";
        assert_eq!(6, process(input));
    }
}
