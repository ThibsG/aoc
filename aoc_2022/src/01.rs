pub fn main() {
    first_puzzle();
    second_puzzle();
}

fn first_puzzle() {
    let input = include_str!("../resources/01.txt");

    let mut sum = 0;
    let mut max = 0;
    for line in input.lines() {
        if line.is_empty() {
            if sum > max {
                max = sum;
            }
            sum = 0;
            continue;
        }

        sum += line.parse::<i32>().unwrap();
    }

    if sum > max {
        max = sum;
    }

    println!("puzzle 1 result: {max}");
}

fn second_puzzle() {
    let input = include_str!("../resources/01.txt");

    let mut sum = 0;
    let mut top_three = vec![0, 0, 0];

    for line in input.lines() {
        if line.is_empty() {
            if sum > top_three[0] {
                top_three[0] = sum;
                top_three.sort();
            }
            sum = 0;
            continue;
        }

        sum += line.parse::<i32>().unwrap();
    }

    if sum > top_three[0] {
        top_three[0] = sum;
    }

    println!("puzzle 2 result: {}", top_three.iter().sum::<i32>());
}
