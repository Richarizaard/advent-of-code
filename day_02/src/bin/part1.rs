fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug, PartialEq, Default)]
struct Entries {
    entries: Vec<Entry>,
}

impl Entries {
    fn add(&mut self, other: Entry) {
        self.entries.push(other)
    }
    fn validate(&self, other: &Vec<Entry>) -> bool {
        for key_entry in &self.entries {
            let color_entries: Vec<&Entry> = other
                .iter()
                .filter(|&e| e.color == key_entry.color)
                .collect();
            for other_entry in color_entries {
                // Any instance in which we go above our threshold, we return false
                if key_entry.count < other_entry.count {
                    return false;
                }
            }
        }
        return true;
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Entry {
    count: u16,
    color: String,
}

fn is_game_possible(game: &str) -> bool {
    let mut key: Entries = Entries::default();
    key.add(Entry { count: 12, color: "red".to_string() });
    key.add(Entry { count: 13, color: "green".to_string() });
    key.add(Entry { count: 14, color: "blue".to_string() });

    let mut parsed_entries: Entries = Entries::default();

    let scores = game.split(":").collect::<Vec<_>>()[1];
    let subsets = scores.split(";").collect::<Vec<_>>();

    for subset in subsets {
        let entries = subset.trim().split(",").collect::<Vec<_>>();
        for entry in entries {
            let a = entry.trim().split_whitespace().collect::<Vec<_>>();
            parsed_entries.add(Entry {
                count: a[0].parse::<u16>().unwrap(),
                color: a[1].to_string(),
            });
        }
    }
    return key.validate(&parsed_entries.entries);
}

fn process(_input: &str) -> u16 {
    let games = _input.lines().collect::<Vec<_>>();
    let mut possible_game_ids: Vec<u16> = Vec::new();

    for game in games {
        let id = game
            .split_whitespace()
            .nth(1)
            .and_then(|s| s.trim_end_matches(":").parse::<u16>().ok())
            .unwrap();

        if is_game_possible(game) {
            possible_game_ids.push(id);
        }
    }
    let ans: u16 = possible_game_ids.iter().sum();
    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input =
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(8, process(input));
    }
}
