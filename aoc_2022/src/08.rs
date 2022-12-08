pub fn main() {
    let input = include_str!("../resources/08.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &[Vec<char>]) -> u32 {
    let mut visible_trees = 0u32;
    for l in 0..input.len() {
        for c in 0..(input[l].len()) {
            let tree = input[l][c];
            let is_visible = |t: &char| -> bool { t < &tree };

            // edges
            if l == 0 || c == 0 || l == (input.len() - 1) || c == (input.len() - 1)
                // left
                || (input[l])[0..c].iter().all(is_visible) 
                // right
                || (input[l])[(c + 1)..].iter().all(is_visible) 
                // top
                || input[0..l].iter().all(|l| l[c] < tree) 
                // bottom
                || input[(l + 1)..].iter().all(|l| l[c] < tree)
            {
                visible_trees += 1;
            }
        }
    }
    visible_trees
}

fn part2(input: &[Vec<char>]) -> u32 {
    let mut highest = 0;

    for l in 0..input.len() {
        for c in 0..(input[l].len()) {
            let tree = input[l][c];

            let mut total;
            let mut score = 0;
            for t in input[0..l].iter().rev() {
                score += 1;
                if t[c] >= tree {
                    break;
                }
            }
            total = score;

            let mut score = 0;
            for t in &input[(l + 1)..] {
                score += 1;
                if t[c] >= tree {
                    break;
                }
            }
            total *= score;

            let mut score = 0;
            for t in (input[l])[0..c].iter().rev() {
                score += 1;
                if t >= &tree {
                    break;
                }
            }
            total *= score;

            let mut score = 0;
            for t in &(input[l])[(c + 1)..] {
                score += 1;
                if t >= &tree {
                    break;
                }
            }
            total *= score;

            if total > highest {
                highest = total;
            }
        }
    }
    highest
}

#[test]
fn test() {
    let input = "30373
    25512
    65332
    33549
    35390"
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    assert_eq!(part1(&input), 21);
    assert_eq!(part2(&input), 8);

    let input = include_str!("../resources/08.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    assert_eq!(part1(&input), 1533);
    assert_eq!(part2(&input), 345744);
}
