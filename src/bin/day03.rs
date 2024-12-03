use regex::Regex;

fn main() {
    let input = include_str!("../../inputs/day03.txt");
    let output1 = part1(input);
    dbg!(output1);
    let output2 = part2(input);
    dbg!(output2);
}

fn part1(input: &str) -> u64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut result = 0;
    for line in input.lines() {
        for (_, [num1, num2]) in re.captures_iter(line).map(|c| c.extract()) {
            let a = num1.parse::<u64>().unwrap();
            let b = num2.parse::<u64>().unwrap();
            println!("{:?} times {:?}", a, b);
            result += a * b;
        }
    }
    result
}

fn part2(input: &str) -> u64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut result = 0;
    let mut enabled = true;
    for line in input.lines() {
        let matches: Vec<_> = re.captures_iter(line).collect();
        for m in matches {
            match m.get(0).map(|m| m.as_str()) {
                Some("do()") => {
                    enabled = true;
                }
                Some("don't()") => {
                    enabled = false;
                }
                Some(_) => {
                    if enabled {
                        dbg!(m.get(1));
                        let x: u64 = m.get(1).unwrap().as_str().parse().unwrap();
                        let y: u64 = m.get(2).unwrap().as_str().parse().unwrap();
                        result += x * y;
                    }
                }
                None => {}
            }
        }
    }
    result
}
