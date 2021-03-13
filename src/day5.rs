#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input.lines().map(process_line).collect()
}

fn process_line(line: &str) -> (usize, usize) {
    let mut row = 0;
    for (i, c) in line.chars().take(7).enumerate() {
        let bit = match c {
            'F' => 0,
            'B' => 1,
            _ => panic!("Invalid input in row"),
        };
        row |= bit << (6 - i);
    }
    let mut col = 0;
    for (i, c) in line.chars().skip(7).enumerate() {
        let bit = match c {
            'L' => 0,
            'R' => 1,
            _ => panic!("Invalid input in row"),
        };
        col |= bit << (2 - i);
    }
    (row, col)
}

fn seat_id(row: usize, col: usize) -> usize {
    row * 8 + col
}

#[aoc(day5, part1)]
pub fn part1(seats: &[(usize, usize)]) -> usize {
    seats.iter().map(|&(r, c)| seat_id(r, c)).max().unwrap()
}

#[aoc(day5, part2)]
pub fn part2(seats: &[(usize, usize)]) -> usize {
    let mut seat_ids = seats
        .iter()
        .map(|&(r, c)| seat_id(r, c))
        .collect::<Vec<usize>>();
    seat_ids.sort();

    let mut last_seen = seat_ids[0];
    for &seat_id in &seat_ids[1..(seat_ids.len() - 1)] {
        if last_seen == seat_id - 2 {
            return seat_id - 1;
        }
        last_seen = seat_id;
    }
    panic!("answer not found!");
}
