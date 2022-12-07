use std::{collections::HashMap, path::PathBuf};

use itertools::Itertools;

pub fn main() {
    let input = include_str!("../resources/07.txt");

    let mut tree = HashMap::new();
    let mut current_path = PathBuf::new();

    input.lines().for_each(|line| {
        if line.starts_with("$ cd") {
            let (_, cd_dir) = line.split("$ cd ").collect_tuple().unwrap();
            if cd_dir == ".." {
                current_path.pop();
            } else {
                current_path = current_path.join(cd_dir);
            }
        } else if !line.starts_with("$ ls") && !line.starts_with("dir ") {
            let (size, _) = line.split(' ').collect_tuple().unwrap();
            let mut tmp_path = PathBuf::new();
            for path in &current_path {
                tmp_path = tmp_path.join(path);
                *tree.entry(tmp_path.clone()).or_insert(0) += size.parse::<u32>().unwrap();
            }
        }
    });

    // First part
    let sum = tree.values().filter(|s| s < &&100_000).sum::<u32>();
    println!("Part 1: {:?}", sum);

    // Second part
    let threshold_space = 30_000_000 - (70_000_000 - tree[&PathBuf::from("/")]);
    let min = tree
        .values()
        .filter(|s| s > &&threshold_space)
        .min()
        .unwrap();
    println!("Part 2: {:?}", min);
}
