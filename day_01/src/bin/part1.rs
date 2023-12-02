fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(_input: &str) -> String {
    let output = _input
        .lines()
        .map(|line| {
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
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input));
    }
}
