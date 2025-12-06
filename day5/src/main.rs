use std::{cmp::max, collections::HashSet};

fn main() {
    let mut input = std::fs::read_to_string("input.txt").expect("Failed to read file");
    let double_newline = "\n\n";
    let index_of_double_newline = input.find(double_newline).unwrap();
    let ingredients = input
        .split_off(index_of_double_newline)
        .trim()
        .split('\n')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();

    let ranges = input
        .trim()
        .split('\n')
        .map(|s| {
            let mut parts = s.split('-');
            let start: u64 = parts.next().unwrap().parse().unwrap();
            let end: u64 = parts.next().unwrap().parse().unwrap();
            (start, end)
        })
        .collect::<Vec<(u64, u64)>>();

    //    task1(ingredients, ranges.clone());
    task2(ranges);
}

fn task2(mut ranges: Vec<(u64, u64)>) {
    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut fresh_ingredients = 0;

    let mut highest_added = 0;
    for (idx, (from, to)) in ranges.iter().enumerate() {
        let start = max(*from, highest_added + 1);
        if start > *to {
            continue;
        }
        fresh_ingredients += *to - start + 1;
        highest_added = max(highest_added, *to);
    }
    assert_eq!(fresh_ingredients, 14);
}

fn task1(ingredients: Vec<u64>, fresh_ids: Vec<(u64, u64)>) {
    let mut fresh_ingredients = 0;

    for id in ingredients {
        for (from, to) in fresh_ids.iter() {
            if id >= *from && id <= *to {
                fresh_ingredients += 1;
                break;
            }
        }
    }

    assert_eq!(fresh_ingredients, 3);
}
