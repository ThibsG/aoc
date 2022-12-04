use std::str::FromStr;

#[derive(Clone)]
struct Section(std::ops::RangeInclusive<u32>);

impl Section {
    // fully contains or is fully contained by
    fn contains(&self, other: &Section) -> bool {
        // does `self` fully contains `other`?
        (self.0.start() <= other.0.start() && self.0.end() >= other.0.end()) ||
        // does `other` fully contains `self`?
        (other.0.start() <= self.0.start() && other.0.end() >= self.0.end())
    }

    fn overlaps(&self, other: &Section) -> bool {
        // negate the case when they don't overlap
        !(self.0.end() < other.0.start() || self.0.start() > other.0.end())
    }
}

impl FromStr for Section {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split('-').collect::<Vec<_>>();
        let (start, end) = (
            u32::from_str(split[0]).unwrap(),
            u32::from_str(split[1]).unwrap(),
        );
        Ok(Self(std::ops::RangeInclusive::new(start, end)))
    }
}

pub fn main() {
    let input = include_str!("../resources/04.txt");

    let sections = input
        .lines()
        .map(|line| {
            let line = line.split(',').collect::<Vec<_>>();
            (
                Section::from_str(line[0]).unwrap(),
                Section::from_str(line[1]).unwrap(),
            )
        })
        .collect::<Vec<_>>();

    // First part
    let sum = sections
        .iter()
        .map(|(section1, section2)| u32::from(section1.contains(section2)))
        .sum::<u32>();
    println!("Part 1 - Score: {sum}");

    // Second part
    let sum = sections
        .iter()
        .map(|(section1, section2)| u32::from(section1.overlaps(section2)))
        .sum::<u32>();
    println!("Part 2 - Score: {sum}");
}
