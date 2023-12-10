fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(_input: &str) -> i64 {
    let lines: Vec<&str> = _input.lines().collect::<Vec<&str>>();

    let mut total = 0;

    for line in lines {
        let nums: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let mut sequence: Vec<Vec<i64>> = vec![];
        sequence.push(nums);

        let mut window_start: usize = 0;
        let mut seq_idx: usize = 0;

        let mut all_zeros: bool = false;

        while !all_zeros {
            let mut diff_seq: Vec<i64> = vec![];

            for window_end in 1..sequence[seq_idx].len() {
                let curr_seq: Vec<i64> = sequence[seq_idx].clone();
                dbg!(&curr_seq[window_end], &curr_seq[window_start]);
                let diff = curr_seq[window_end] - curr_seq[window_start];
                diff_seq.push(diff);

                window_start += 1;
            }

            if diff_seq.iter().all(|&num| num == 0) {
                all_zeros = true;
            }

            sequence.push(diff_seq);
            window_start = 0; // reset window
            seq_idx += 1; // increment seq idx
        }

        // Add up the last indexes
        for seq in sequence.iter().rev() {
            let last = *seq.last().unwrap();
            total += last;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";
        assert_eq!(114, process(input));
    }
}
