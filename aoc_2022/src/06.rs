use std::collections::HashSet;

use itertools::Itertools;

pub fn main() {
    let input = include_str!("../resources/06.txt");

    // first part
    for (idx, (one, two, three, four)) in input.chars().tuple_windows().enumerate() {
        if ![one, two, three].contains(&four)
            && ![one, two, four].contains(&three)
            && ![one, three, four].contains(&two)
            && ![two, three, four].contains(&one)
        {
            println!("Part 1: start of packet {}", idx + 4);
            break;
        }
    }

    // second part
    for (idx, window) in input.chars().collect::<Vec<_>>().windows(14).enumerate() {
        let mut set = HashSet::new();
        (0..14).for_each(|i| {
            set.insert(window[i]);
        });

        if set.len() == 14 {
            println!("Part 2: start of message {}", idx + 14);
            break;
        }
    }
}
