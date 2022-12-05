use itertools::Itertools;

pub fn main() {
    let input = include_str!("../resources/05.txt");

    // first part
    let (ship, guide) = input.split("\n\n").collect_tuple().unwrap();

    let mut ship = ship.lines().rev();
    let stack_cols = ship
        .next()
        .map(|line| {
            line.split_whitespace()
                .map(|c| c.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .unwrap()
        .len();

    let mut stacks = vec![vec![]; stack_cols];
    ship.for_each(|line| {
        line.chars()
            .chunks(4)
            .into_iter()
            .enumerate()
            .for_each(|(idx, mut chunk)| {
                let letter = chunk.nth(1).unwrap();
                if letter != ' ' {
                    stacks[idx].push(letter);
                }
            });
    });

    let mut stacks2 = stacks.clone();

    guide.lines().for_each(|line| {
        let split = line.split_whitespace().collect::<Vec<_>>();
        let (nb_stacks, from, to) = (
            split[1].parse::<usize>().unwrap(),
            split[3].parse::<usize>().unwrap(),
            split[5].parse::<usize>().unwrap(),
        );

        // first part
        let split_at = stacks[from - 1].len() - nb_stacks;
        let mut moved = stacks[from - 1].split_off(split_at);
        moved.reverse();
        stacks[to - 1].append(&mut moved);

        // second part
        let mut moved = stacks2[from - 1].split_off(split_at);
        stacks2[to - 1].append(&mut moved);
    });

    print!("Part 1: ");
    stacks.iter().for_each(|s| print!("{}", s.last().unwrap()));
    println!();

    print!("Part 2: ");
    stacks2.iter().for_each(|s| print!("{}", s.last().unwrap()));
    println!();
}
