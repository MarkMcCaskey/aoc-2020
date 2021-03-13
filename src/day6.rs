use std::collections::HashSet;

#[aoc_generator(day6)]
pub fn parse_input(input: &str) -> Vec<Vec<HashSet<char>>> {
    let mut out = vec![];
    let mut cur = vec![];
    for line in input.lines() {
        if line.is_empty() {
            out.push(cur);
            cur = vec![];
            continue;
        }
        cur.push(line.chars().collect::<HashSet<char>>());
    }
    out.push(cur);
    out
}

#[aoc(day6, part1)]
pub fn part1(questions: &[Vec<HashSet<char>>]) -> usize {
    questions
        .iter()
        .map(|group| {
            group
                .iter()
                .map(|answers| answers.iter().cloned())
                .flatten()
                .collect::<HashSet<char>>()
                .len()
        })
        .sum()
}

#[aoc(day6, part2)]
pub fn part2(questions: &[Vec<HashSet<char>>]) -> usize {
    questions
        .iter()
        .map(|group| {
            group
                .iter()
                .fold(
                    ('a'..='z').collect::<HashSet<char>>(),
                    |acc: HashSet<char>, v: &HashSet<char>| acc.intersection(v).cloned().collect(),
                )
                .len()
        })
        .sum()
}
