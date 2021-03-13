#[derive(Debug, Clone)]
pub struct PwPolicy {
    lower: usize,
    upper: usize,
    c: char,
    password: String,
}

impl PwPolicy {
    pub fn is_valid(&self) -> bool {
        let mut count = 0;
        for c in self.password.chars() {
            if c == self.c {
                count += 1;
            }
        }
        count >= self.lower && count <= self.upper
    }
    pub fn is_valid_part2(&self) -> bool {
        let mut count = 0;
        let indexable_chars = self.password.chars().collect::<Vec<char>>();
        if let Some(c) = indexable_chars.get(self.lower - 1) {
            if *c == self.c {
                count += 1;
            }
        }
        if let Some(c) = indexable_chars.get(self.upper - 1) {
            if *c == self.c {
                count += 1;
            }
        }
        count == 1
    }
}

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<PwPolicy> {
    input.lines().map(parse_pw_policy).collect()
}

fn parse_pw_policy(line: &str) -> PwPolicy {
    let (left, password) = {
        let mut l = line.split(": ");
        (l.next().unwrap(), l.next().unwrap().to_string())
    };
    let (range_expr, c) = {
        let mut l = left.split(' ');
        (l.next().unwrap(), l.next().unwrap().chars().next().unwrap())
    };
    let (lower, upper) = {
        let mut l = range_expr.split('-');
        (
            l.next().unwrap().parse::<usize>().unwrap(),
            l.next().unwrap().parse::<usize>().unwrap(),
        )
    };
    PwPolicy {
        lower,
        upper,
        c,
        password,
    }
}

#[aoc(day2, part1)]
pub fn part1(pws: &[PwPolicy]) -> usize {
    pws.iter().filter(|pw| pw.is_valid()).count()
}

#[aoc(day2, part2)]
pub fn part2(pws: &[PwPolicy]) -> usize {
    pws.iter().filter(|pw| pw.is_valid_part2()).count()
}
