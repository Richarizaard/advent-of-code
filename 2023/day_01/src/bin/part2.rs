fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(_input: &str) -> String {
    let output = _input
        .lines()
        .map(|line| {
            let line = line
                .replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e");

            let mut iter = line.chars().filter_map(|c| c.to_digit(10));

            let first = iter.next().expect("should be a valid number");
            let last = iter.last();

            (
                match last {
                    Some(val) => format!("{first}{val}").parse::<u32>(),
                    None => format!("{first}{first}").parse::<u32>(),
                }
            ).expect("should also be a valid number")
        })
        .sum::<u32>();

    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input =
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input));
    }
}
