fn main() {
    let input = include_str!("../../inputs/day02.txt");
    let output1 = part1(input);
    dbg!(output1);
    let output2 = part2(input);
    dbg!(output2);
}

fn is_valid_sequence(differences: &[i64]) -> bool {
    let all_negative =
        differences.iter().all(|&diff| diff < 0 && diff >= -3) && differences.len() > 0;

    let all_positive =
        differences.iter().all(|&diff| diff > 0 && diff <= 3) && differences.len() > 0;

    all_negative || all_positive
}

fn part1(input: &str) -> usize {
    let mut count_valid = 0;
    for line in input.lines() {
        let levels: Vec<&str> = line.split_whitespace().collect();
        let differences: Vec<i64> = levels
            .windows(2)
            .map(|window| window[0].parse::<i64>().unwrap() - window[1].parse::<i64>().unwrap())
            .collect();
        if is_valid_sequence(&differences) {
            count_valid += 1;
        }
    }
    count_valid
}

fn part2(input: &str) -> usize {
    let mut count_valid = 0;
    for line in input.lines() {
        let mut current_valid = 0;
        let levels: Vec<&str> = line.split_whitespace().collect();
        let differences: Vec<i64> = levels
            .windows(2)
            .map(|window| window[0].parse::<i64>().unwrap() - window[1].parse::<i64>().unwrap())
            .collect();
        if is_valid_sequence(&differences) {
            current_valid += 1;
        }

        levels.iter().enumerate().for_each(|(index, _)| {
            let permutation: Vec<_> = levels
                .iter()
                .enumerate()
                .filter(|&(i, _)| i != index)
                .map(|(_, x)| x)
                .collect();
            let differences: Vec<i64> = permutation
                .windows(2)
                .map(|window| window[0].parse::<i64>().unwrap() - window[1].parse::<i64>().unwrap())
                .collect();
            if is_valid_sequence(&differences) {
                current_valid += 1;
            }
        });
        if current_valid > 0 {
            count_valid += 1;
        }
    }

    count_valid
}
