use std::u32;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn parse_numbers(_numbers: &str) -> Vec<Vec<u32>> {
    _numbers
        .split("|")
        .map(|section| {
            section
                .split_whitespace()
                .map(|col| col.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

fn process(_input: &str) -> u32 {
    let cards = _input.lines().collect::<Vec<_>>();
    let mut games_played = vec![0; _input.lines().count()];

    for (idx, card) in cards.iter().enumerate() {
        let numbers = card.split(":").collect::<Vec<_>>()[1].trim();
        let parsed_numbers = parse_numbers(&numbers);
        games_played[idx] += 1;

        if let Some(winning_numbers) = parsed_numbers.first().cloned() {
            let intersect: Vec<u32> = winning_numbers
                .into_iter()
                .filter(|&num| parsed_numbers.iter().all(|v| v.contains(&num)))
                .collect::<Vec<u32>>();

            for won in 1..intersect.len() + 1 {
                games_played[idx + won] += games_played[idx];
            }
        }
    }
    let total: u32 = games_played.iter().sum();
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(30, process(input));
    }
}
