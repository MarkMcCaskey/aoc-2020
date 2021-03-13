#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Vec<bool> {
    line.chars().map(|c| c == '#').collect()
}

fn count_collisions(map: &[Vec<bool>], x_diff: usize, y_diff: usize) -> usize {
    let mut num_trees = 0;
    let mut x = 0;
    let mut y = 0;
    let horiz_max = map[0].len();
    let vert_max = map.len();
    while y < vert_max {
        if map[y][x % horiz_max] {
            num_trees += 1;
        }
        x += x_diff;
        y += y_diff;
    }
    num_trees
}

#[aoc(day3, part1)]
pub fn part1(map: &[Vec<bool>]) -> usize {
    count_collisions(map, 3, 1)
}

#[aoc(day3, part2)]
pub fn part2(map: &[Vec<bool>]) -> usize {
    let paths_to_check = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    paths_to_check
        .iter()
        .map(|&(x, y)| count_collisions(map, x, y))
        .product()
}
