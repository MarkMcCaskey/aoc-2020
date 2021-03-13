use std::collections::HashMap;

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<isize> {
    input
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(nums: &[isize]) -> isize {
    let inverse_map = nums
        .iter()
        .map(|&n| (2020 - n, n))
        .collect::<HashMap<isize, isize>>();

    for num in nums {
        if let Some(val) = inverse_map.get(num) {
            return num * val;
        }
    }
    panic!("No result found!");
}

#[aoc(day1, part2)]
pub fn part2(nums: &[isize]) -> isize {
    let inverse_map = nums
        .iter()
        .map(|&n| (2020 - n, n))
        .collect::<HashMap<isize, isize>>();

    for num1 in nums {
        for num2 in nums {
            if let Some(val) = inverse_map.get(&(num1 + num2)) {
                return num1 * num2 * val;
            }
        }
    }
    panic!("No result found!");
}
