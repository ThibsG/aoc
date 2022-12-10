pub fn main() {
    let input = include_str!("../resources/10.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2:\n{}", part2(input));
}

fn part1(input: &str) -> i32 {
    let cycles = [20, 60, 100, 140, 180, 220];
    let mut sum = 0;
    let mut addx = 1;
    let mut cycle_idx = 1;

    for line in input.lines() {
        if cycles.contains(&cycle_idx) {
            sum += cycle_idx * addx;
        }
        cycle_idx += 1;

        if line.starts_with("noop") {
            continue;
        }

        if cycles.contains(&cycle_idx) {
            sum += cycle_idx * addx;
        }
        cycle_idx += 1;

        addx += line.strip_prefix("addx ").unwrap().parse::<i32>().unwrap();
    }

    sum
}

fn part2(input: &str) -> String {
    let mut output = String::new();
    let mut addx = 1;
    let mut cycle_idx = 1;

    for line in input.lines() {
        let position = (cycle_idx - 1) % 40;
        if position == 0 {
            output.push('\n');
        }

        if (addx - 1) % 40 == position || addx % 40 == position || (addx + 1) % 40 == position {
            output.push('#');
        } else {
            output.push('.');
        }
        cycle_idx += 1;

        if line.starts_with("noop") {
            continue;
        }

        let position = (cycle_idx - 1) % 40;
        if position == 0 {
            output.push('\n');
        }

        if (addx - 1) % 40 == position || addx % 40 == position || (addx + 1) % 40 == position {
            output.push('#');
        } else {
            output.push('.');
        }
        cycle_idx += 1;

        addx += line.strip_prefix("addx ").unwrap().parse::<i32>().unwrap();
    }
    output
}

#[test]
fn test() {
    let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    assert_eq!(part1(input), 13140);
}
