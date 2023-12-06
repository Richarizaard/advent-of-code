fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

fn parse_race(input: &str) -> Vec<Race> {
    let mut races = Vec::new();

    let lines: Vec<&str> = input.lines().collect();
    let time_values: Vec<u32> = lines[0]
        .split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse().ok())
        .collect();

    let distance_values: Vec<u32> = lines[1]
        .split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse().ok())
        .collect();

    for (time, distance) in time_values.into_iter().zip(distance_values.into_iter()) {
        races.push(Race { time, distance });
    }

    races
}

fn process(_input: &str) -> u32 {
    let mut accumulator = 1;
    let races = parse_race(_input);
    for race in races {
        let mut num_of_wins = 0;
        for holding_time in 0..race.time {
            let calc = (race.time - holding_time) * holding_time;
            if calc > race.distance {
                num_of_wins += 1;
            }
        }
        accumulator *= num_of_wins;
    }
    accumulator
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Time:      7  15   30
        Distance:  9  40  200";
        assert_eq!(288, process(input));
    }
}
