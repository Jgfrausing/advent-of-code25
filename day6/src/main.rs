fn main() {
    task2();
}

fn task2() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read file");
    let lines = input.lines().collect::<Vec<_>>();
    let operators = lines
        .iter()
        .last()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|v| match v {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            operation => panic!("Unknown operation {operation}"),
        })
        .collect::<Vec<_>>();

    let mut parts = Vec::new();
    let mut current = Vec::new();
    for column in 0..lines[0].len() {
        let mut number = "".to_string();
        let mut found_value = false;

        for line in lines.iter().take(lines.len() - 1) {
            let ch = line.chars().nth(column).unwrap();

            if ch == ' ' {
                continue;
            }
            number = format!("{number}{ch}");
            found_value = true;
        }
        if found_value {
            current.push(number);
        } else {
            parts.push(current);
            current = Vec::new();
        }
    }
    parts.push(current);

    let mut sum = 0;
    for (equation, operator) in parts.iter().zip(operators.iter()) {
        let mut result = 0;
        for part in equation {
            let value = part.parse::<u64>().expect("Failed to parse number");
            result = match operator {
                Operation::Add => result + value,
                Operation::Multiply => {
                    if result == 0 {
                        value
                    } else {
                        result * value
                    }
                }
            };
        }
        println!(
            "Result for equation {:?} with operator {:?} is {}",
            equation, operator, result
        );
        sum += result;
    }

    assert_eq!(sum, 3263827);
}

fn task1() {
    let input = std::fs::read_to_string("test.txt").expect("Failed to read file");
    let lines = input.lines();
    // amount of lines
    let num_of_equations = lines.clone().count();
    let parts = lines
        .rev()
        .map(|s| s.trim().split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut sum = 0;
    for idx in 0..parts[0].len() {
        let last = parts.iter().last().unwrap()[idx];
        let operation = match last {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            operation => panic!("Unknown operation {operation}"),
        };
        let mut result = 0;
        let mut position: usize = 0;
        for line in parts.iter().rev().skip(1) {
            let value: u64 = line[idx].parse().expect("Failed to parse number");
            result = match operation {
                Operation::Add => result + value,
                Operation::Multiply => {
                    if result == 0 {
                        value
                    } else {
                        result * value
                    }
                }
            };
        }
    }
    assert_eq!(sum, 3263827);
}

#[derive(Clone, Debug)]
enum Operation {
    Add,
    Multiply,
}
