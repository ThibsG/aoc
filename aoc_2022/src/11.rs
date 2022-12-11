use std::str::FromStr;

pub fn main() {
    let input = include_str!("../resources/11.txt");

    println!("Part 1: {}", do_parts(input, true, 20));
    println!("Part 2: {}", do_parts(input, false, 10_000));
}

fn do_parts(input: &str, is_part_1: bool, rounds: u32) -> i64 {
    let mut monkeys = input
        .split("\n\n")
        .into_iter()
        .map(|raw| Monkey::from_str(raw).unwrap())
        .collect::<Vec<_>>();

    // Only relevant for part 2
    // manage worry levels by using the lowest common multiple
    // https://en.wikipedia.org/wiki/Chinese_remainder_theorem
    // https://en.wikipedia.org/wiki/Coprime_integers
    let modulo = monkeys.iter().map(|m| m.divide_by).product::<i64>();

    let mut visited_items = vec![0; monkeys.len()];
    for _round in 0..rounds {
        for idx in 0..monkeys.len() {
            let m = monkeys[idx].clone();
            for worry_level in &m.items {
                let new_wl =
                    operation(*worry_level, m.operand.unwrap_or(*worry_level), &m.operator);
                let new_wl = if is_part_1 { new_wl / 3 } else { new_wl } % modulo;
                let next_monkey = test_div(new_wl, m.divide_by, m.cond_true, m.cond_false) as usize;
                monkeys[next_monkey].items.push(new_wl);

                visited_items[idx] += 1;
            }
            monkeys[idx].items.clear();
        }
    }

    visited_items.sort();
    visited_items.reverse();
    visited_items[0] * visited_items[1]
}

fn operation(old: i64, operand: i64, operator: &str) -> i64 {
    match operator {
        "/" => old / operand,
        "*" => old * operand,
        "+" => old + operand,
        "-" => old - operand,
        _ => panic!(),
    }
}

fn test_div(worry_level: i64, divide_by: i64, cond_true: i64, cond_false: i64) -> i64 {
    if worry_level % divide_by == 0 {
        cond_true
    } else {
        cond_false
    }
}

#[derive(Clone, Debug)]
pub struct Monkey {
    pub items: Vec<i64>,
    pub operand: Option<i64>,
    pub operator: String,
    pub divide_by: i64,
    pub cond_true: i64,
    pub cond_false: i64,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let raw = raw.lines().skip(1).collect::<Vec<_>>();
        let items = raw[0]
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|item| item.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let (operator, operand) = raw[1]
            .split_once("old ")
            .unwrap()
            .1
            .split_once(' ')
            .unwrap();
        let divide_by = raw[2].split_once("by ").unwrap().1;
        let cond_true = raw[3].split_once("monkey ").unwrap().1;
        let cond_false = raw[4].split_once("monkey ").unwrap().1;
        Ok(Self {
            items,
            operand: operand.parse::<i64>().ok(),
            operator: operator.to_string(),
            divide_by: divide_by.parse::<i64>().unwrap(),
            cond_true: cond_true.parse::<i64>().unwrap(),
            cond_false: cond_false.parse::<i64>().unwrap(),
        })
    }
}

#[test]
fn test() {
    let input = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
    assert_eq!(do_parts(input, true, 20), 10605);
    assert_eq!(do_parts(input, false, 10_000), 2713310158);
}
