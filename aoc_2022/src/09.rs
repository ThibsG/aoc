use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;

pub fn main() {
    let input = include_str!("../resources/09.txt");
    part1(input);
    part2(input);
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "R" => Self::Right,
            "L" => Self::Left,
            _ => panic!("unknown direction"),
        })
    }
}

fn part1(input: &str) -> usize {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited = HashSet::new();

    input.lines().for_each(|line| {
        let (direction, steps) = line.split(' ').collect_tuple().unwrap();

        let direction = Direction::from_str(direction).unwrap();
        let steps = steps.parse::<i32>().unwrap();

        (0..steps).for_each(|_step| {
            visited.insert(tail);

            let is_diag = i32::abs(head.0 - tail.0) == 1 && i32::abs(head.1 - tail.1) == 1;

            match direction {
                Direction::Up => head.1 += 1,
                Direction::Down => head.1 -= 1,
                Direction::Right => head.0 += 1,
                Direction::Left => head.0 -= 1,
            }

            let diff_0 = tail.0 - head.0;
            let diff_1 = tail.1 - head.1;

            if diff_0 == 2 {
                tail.0 -= 1;

                if is_diag {
                    if diff_1 < 0 {
                        tail.1 += 1;
                    } else {
                        tail.1 -= 1;
                    }
                }
            } else if diff_0 == -2 {
                tail.0 += 1;

                if is_diag {
                    if diff_1 < 0 {
                        tail.1 += 1;
                    } else {
                        tail.1 -= 1;
                    }
                }
            } else if diff_1 == 2 {
                tail.1 -= 1;

                if is_diag {
                    if diff_0 < 0 {
                        tail.0 += 1;
                    } else {
                        tail.0 -= 1;
                    }
                }
            } else if diff_1 == -2 {
                tail.1 += 1;

                if is_diag {
                    if diff_0 < 0 {
                        tail.0 += 1;
                    } else {
                        tail.0 -= 1;
                    }
                }
            }
        });

        visited.insert(tail);
    });
    println!("Part1 - Tail visited {} positions", visited.len());
    visited.len()
}

fn part2(input: &str) -> usize {
    let mut rope = vec![(0, 0); 10];
    let mut visited = HashSet::new();

    input.lines().for_each(|line| {
        let (direction, steps) = line.split(' ').collect_tuple().unwrap();

        let direction = Direction::from_str(direction).unwrap();
        let steps = steps.parse::<i32>().unwrap();

        (0..steps).for_each(|_step| {
            match direction {
                Direction::Up => rope[0].1 += 1,
                Direction::Down => rope[0].1 -= 1,
                Direction::Right => rope[0].0 += 1,
                Direction::Left => rope[0].0 -= 1,
            }

            (1..rope.len()).for_each(|knot_idx| {
                let head = rope[knot_idx - 1];
                let mut tail = &mut rope[knot_idx];

                let diff_0 = head.0 - tail.0;
                let diff_1 = head.1 - tail.1;

                if i32::abs(diff_0) > 1 || i32::abs(diff_1) > 1 {
                    if diff_0 == 2 {
                        tail.0 += 1;
                    } else if diff_0 == -2 {
                        tail.0 -= 1;
                    } else {
                        tail.0 += diff_0;
                    }
                    if diff_1 == 2 {
                        tail.1 += 1;
                    } else if diff_1 == -2 {
                        tail.1 -= 1;
                    } else {
                        tail.1 += diff_1;
                    }
                }
            });

            visited.insert(*rope.last().unwrap());
        });
    });
    println!("Part2 - Tail visited {} positions", visited.len());
    visited.len()
}

#[test]
fn test() {
    let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    assert_eq!(part1(input), 13);
    let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
    assert_eq!(part2(input), 36);
}
