fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct EngineSchematic {
    data: Vec<Vec<char>>,
}

impl EngineSchematic {
    fn new(input: &str) -> String {}
}
fn process(_input: &str) -> String {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("4361", process(input));
    }
}
