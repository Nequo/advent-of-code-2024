use std::collections::HashMap;

fn main() {
    let input = include_str!("../../inputs/day01.txt");
    let output1 = part1(input);
    dbg!(output1);
    let output2 = part2(input);
    dbg!(output2);
}

fn part1(input: &str) -> i64 {
    let mut list1: Vec<i64> = vec![];
    let mut list2: Vec<i64> = vec![];
    for line in input.lines() {
        let (fst, snd) = line.split_once("   ").unwrap();
        list1.push(fst.parse::<i64>().unwrap());
        list2.push(snd.parse::<i64>().unwrap());
    }
    list1.sort();
    list2.sort();
    let sum_of_differences: i64 = list1
        .into_iter()
        .zip(list2.into_iter())
        .map(|(x, y)| (y - x).abs())
        .sum();
    sum_of_differences
}

fn part2(input: &str) -> i64 {
    let mut list1: Vec<i64> = vec![];
    let mut frequency: HashMap<i64, i64> = HashMap::new();
    for line in input.lines() {
        let (fst, snd) = line.split_once("   ").unwrap();
        list1.push(fst.parse::<i64>().unwrap());
        *frequency.entry(snd.parse::<i64>().unwrap()).or_insert(0) += 1;
    }
    let result: i64 = list1
        .into_iter()
        .map(|val| {
            if let Some(x) = frequency.get(&val) {
                val * x
            } else {
                0
            }
        })
        .sum();
    result
}
