fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug, PartialEq, Default)]
struct Entries {
    entries: Vec<Entry>,
}

impl From<Vec<Entry>> for Entries {
    fn from(entries: Vec<Entry>) -> Self {
        Self { entries }
    }
}

impl Entries {
    fn add(&mut self, other: Entry) {
        self.entries.push(other)
    }
    fn max(&self, color: &str) -> Option<u32> {
        self.entries
            .iter()
            .filter(|&entry| entry.color == color)
            .map(|entry| entry.count)
            .max()
    }
}

#[derive(Debug, PartialEq)]
struct Entry {
    count: u32,
    color: String,
}

fn max_colors(entries: &Entries) -> u32 {
    let mut max_colors = 1;

    for color in ["red", "blue", "green"] {
        max_colors *= entries.max(color).unwrap();
    }
    max_colors
}
fn parse_game(game: &str) -> u32 {
    let mut parsed_entries: Entries = Entries::default();

    let scores = game.split(":").collect::<Vec<_>>()[1];
    let subsets = scores.split(";").collect::<Vec<_>>();

    for subset in subsets {
        let entries = subset.trim().split(",").collect::<Vec<_>>();
        for entry in entries {
            let a = entry.trim().split_whitespace().collect::<Vec<_>>();
            parsed_entries.add(Entry {
                count: a[0].parse::<u32>().unwrap(),
                color: a[1].to_string(),
            });
        }
    }
    max_colors(&parsed_entries)
}

fn process(_input: &str) -> u32 {
    let games = _input.lines().collect::<Vec<_>>();
    let mut accumulator: u32 = 0;

    for game in games {
        accumulator += parse_game(game);
    }
    // let ans: u16 = possible_game_ids.iter().sum();
    return accumulator;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(2286, process(input));
    }
}
