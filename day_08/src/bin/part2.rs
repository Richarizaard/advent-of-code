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

    fn navigate(&self, starts: Vec<&str>) -> Vec<u32> {
        let curr: Vec<String> = starts.iter().map(|&s| s.to_string()).collect();

        let mut all_steps: Vec<u32> = vec![];
        let mut steps: u32 = 0;
        let mut node_found = false;

        // This makes me want to cry but I'm sooo tired and want to get this over with.

        for mut start in curr {
            while !node_found {
                for c in self.instructions.chars() {
                    let node = &self.nodes[&start];

                    if c == 'R' {
                        start = node.right.clone()
                    } else {
                        start = node.left.clone()
                    };
                    steps += 1;
                }

                if start.ends_with('Z') {
                    node_found = true
                }
            }
            all_steps.push(steps);

            node_found = false;
            steps = 0;
        }
        all_steps
    }
}

fn get_start_end(_input: &str) -> Vec<&str> {
    let lines: Vec<&str> = _input.lines().skip(2).collect::<Vec<&str>>();

    let mut starts: Vec<&str> = Vec::new();

    for line in lines {
        let id = line.split(" = ").collect::<Vec<&str>>();
        if id[0].ends_with('A') {
            starts.push(id[0].trim());
        }
    }

    starts
}
fn process(_input: &str) -> Vec<u32> {
    let network = Network::new(_input);
    let starts = get_start_end(_input);

    let all_steps: Vec<u32> = network.navigate(starts);

    all_steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_one() {
        let input = "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";
        assert_eq!(vec![2, 6], process(input));
    }
}
