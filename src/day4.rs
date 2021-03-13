use std::collections::HashMap;

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> Vec<HashMap<String, String>> {
    let mut out = vec![];
    let mut cur_entry = HashMap::new();
    for line in input.lines() {
        if line.is_empty() {
            out.push(cur_entry);
            cur_entry = HashMap::new();
            continue;
        }
        let item_iter = line.split(' ');
        for item in item_iter {
            let mut i = item.split(':');
            let key = i.next().unwrap().to_string();
            let value = i.next().unwrap().to_string();
            cur_entry.insert(key, value);
        }
    }
    out.push(cur_entry);
    out
}

fn passport_is_valid(passport: &HashMap<String, String>) -> bool {
    let required_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    required_keys
        .iter()
        .all(|key| passport.contains_key(&key.to_string()))
}

fn passport_is_valid_part2(passport: &HashMap<String, String>) -> bool {
    let valid_byr = |byr: &str| -> Option<bool> {
        let n = byr.parse::<usize>().ok()?;
        Some(n >= 1920 && n <= 2002)
    };
    let valid_iyr = |iyr: &str| -> Option<bool> {
        let n = iyr.parse::<usize>().ok()?;
        Some(n >= 2010 && n <= 2020)
    };
    let valid_eyr = |eyr: &str| -> Option<bool> {
        let n = eyr.parse::<usize>().ok()?;
        Some(n >= 2020 && n <= 2030)
    };
    let valid_hgt = |hgt: &str| -> Option<bool> {
        if let Some(n) = hgt.strip_suffix("cm") {
            let n = n.parse::<usize>().ok()?;
            Some(n >= 150 && n <= 193)
        } else if let Some(n) = hgt.strip_suffix("in") {
            let n = n.parse::<usize>().ok()?;
            Some(n >= 59 && n <= 76)
        } else {
            None
        }
    };
    let valid_hcl = |hcl: &str| -> Option<bool> {
        let hcl = hcl.strip_prefix("#")?;
        if hcl.len() != 6 {
            return Some(false);
        }
        Some(
            hcl.chars()
                .all(|c| c >= '0' && c <= '9' || c >= 'a' && c <= 'f'),
        )
    };
    let valid_ecl = |ecl: &str| -> bool {
        matches!(ecl, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
    };
    let valid_pid = |pid: &str| -> bool { pid.len() == 9 && pid.chars().all(char::is_numeric) };

    valid_byr(&passport["byr"]).unwrap_or(false)
        && valid_iyr(&passport["iyr"]).unwrap_or(false)
        && valid_eyr(&passport["eyr"]).unwrap_or(false)
        && valid_hgt(&passport["hgt"]).unwrap_or(false)
        && valid_hcl(&passport["hcl"]).unwrap_or(false)
        && valid_ecl(&passport["ecl"])
        && valid_pid(&passport["pid"])
}

#[aoc(day4, part1)]
pub fn part1(passports: &[HashMap<String, String>]) -> usize {
    passports.iter().filter(|p| passport_is_valid(p)).count()
}

#[aoc(day4, part2)]
pub fn part2(passports: &[HashMap<String, String>]) -> usize {
    passports
        .iter()
        .filter(|p| passport_is_valid(p))
        .filter(|p| passport_is_valid_part2(p))
        .count()
}
