use itertools::Itertools;

pub fn main() {
    let score_it = |c| -> u32 {
        match c {
            c @ 'a'..='z' => c as u32 - 96,
            c @ 'A'..='Z' => c as u32 - 38, // (- 64) + 26
            _ => unimplemented!(),
        }
    };

    // First part
    let input = include_str!("../resources/03.txt");

    let sum = input
        .lines()
        .map(|l| {
            let (first, second) = l.split_at(l.len() / 2);

            first
                .chars()
                .find_map(|c| second.contains(c).then(|| score_it(c)))
                .unwrap()
        })
        .sum::<u32>();
    println!("Part 1 - Score: {sum}");

    let sum = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let (first, second, third) = chunk.collect_tuple().unwrap();

            first
                .chars()
                .find_map(|c| {
                    if second.contains(c) && third.contains(c) {
                        Some(score_it(c))
                    } else {
                        None
                    }
                })
                .unwrap()
        })
        .sum::<u32>();
    println!("Part 2 - Score: {sum}");
}
