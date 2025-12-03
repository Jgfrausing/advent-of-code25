use std::ops::ControlFlow;

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read file");
    let lines = input.lines();
    // task1(lines.clone());
    task2(lines.clone());
}

fn task1(lines: std::str::Lines<'_>) {
    let mut output = 0;
    for line in lines {
        let mut hightest_1 = 0;
        let mut hightest_2 = 0;

        for (idx, char) in line.chars().enumerate() {
            let value: u64 = format!("{char}").parse().unwrap();
            let is_last = idx == line.len() - 1;
            if value > hightest_1 && !is_last {
                hightest_2 = 0;
                hightest_1 = value;
            } else if value > hightest_2 {
                hightest_2 = value;
            }
        }
        let highest12 = hightest_1 * 10 + hightest_2;
        output += highest12;
    }
    println!("Output: {}", output);
}

fn task2(lines: std::str::Lines<'_>) {
    let mut output = 0;

    println!("line {}", lines.clone().count());
    for (idx, line) in lines.enumerate() {
        let mut as_numbers = line
            .chars()
            .map(|c| format!("{c}").parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let len = as_numbers.len();

        let mut best = vec![];

        let len = as_numbers.len();
        let batteries = 12;
        let mut index_of_last = 0;
        for position in 0..batteries {
            let last_possible = len - 12 + position;

            let mut best_for_postition: u64 = 0;
            for i in index_of_last..=last_possible {
                let num = as_numbers[i];
                if num > best_for_postition {
                    best_for_postition = num;
                    index_of_last = i + 1;
                }
            }
            best.push(best_for_postition);
        }
        let as_str = best.into_iter().map(|n| n.to_string()).collect::<String>();
        let as_value: u64 = as_str.parse().unwrap();
        output += as_value;
    }
    println!("Output: {}", output);
}
