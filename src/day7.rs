use std::collections::HashMap;

#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> HashMap<String, HashMap<String, usize>> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> (String, HashMap<String, usize>) {
    let (name, expr) = {
        let mut i = line.split(" bags contain ");
        (i.next().unwrap().to_string(), i.next().unwrap())
    };

    let vals = {
        expr.strip_suffix(".")
            .unwrap()
            .split(", ")
            .filter(|&sub_expr| sub_expr != "no other bags")
            .map(|sub_expr| {
                let num_str = sub_expr
                    .chars()
                    .take_while(|c| char::is_numeric(*c))
                    .collect::<String>();
                let sub_name = {
                    let sub_name = &sub_expr[(num_str.len() + 1)..];
                    if let Some(sub_name) = sub_name.strip_suffix(" bags") {
                        sub_name
                    } else if let Some(sub_name) = sub_name.strip_suffix(" bag") {
                        sub_name
                    } else {
                        panic!("expected suffix ` bag` or ` bags` to strip off!");
                    }
                };
                (sub_name.to_string(), num_str.parse::<usize>().unwrap())
            })
            .collect::<HashMap<String, usize>>()
    };

    (name, vals)
}

#[aoc(day7, part1)]
pub fn part1(rules: &HashMap<String, HashMap<String, usize>>) -> usize {
    let mut can_contain_shiny_gold = HashMap::new();
    for (k, v) in rules.iter() {
        for (sub_k, _) in v.iter() {
            if sub_k == "shiny gold" {
                can_contain_shiny_gold.insert(k, true);
                break;
            }
        }
    }
    // instead of creating an inverted graph, we just iterate until it converges
    let mut old_len = 0;
    while old_len != can_contain_shiny_gold.len() {
        old_len = can_contain_shiny_gold.len();
        for (k, v) in rules.iter() {
            for (sub_k, _) in v.iter() {
                if can_contain_shiny_gold.contains_key(sub_k) {
                    can_contain_shiny_gold.insert(k, true);
                    break;
                }
            }
        }
    }

    can_contain_shiny_gold.len()
}

fn dfs(
    rules: &HashMap<String, HashMap<String, usize>>,
    minimum_reqd_bags: &mut HashMap<String, usize>,
    item: &String,
) -> usize {
    if minimum_reqd_bags.contains_key(item) {
        return minimum_reqd_bags[item];
    }

    let mut tot = 0;
    for (child, v) in rules[item].iter() {
        tot += (dfs(rules, minimum_reqd_bags, child) + 1) * v;
    }
    minimum_reqd_bags.insert(item.clone(), tot);

    tot
}

#[aoc(day7, part2)]
pub fn part2(rules: &HashMap<String, HashMap<String, usize>>) -> usize {
    let mut minimum_reqd_bags: HashMap<String, usize> = rules
        .iter()
        .filter(|(_, v)| v.is_empty())
        .map(|(k, _)| (k.clone(), 0))
        .collect();

    dfs(rules, &mut minimum_reqd_bags, &"shiny gold".to_string())
}
