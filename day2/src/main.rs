fn main() {
    // read file input.csv
    // split by comma `,`
    // for each split also split (int,int) by `-`
    //

    let input = std::fs::read_to_string("input.csv").expect("Failed to read file");
    let ranges = input
        .trim()
        .split(',')
        .map(|s| {
            let mut parts = s.split('-');
            let start: u64 = parts.next().unwrap().parse().unwrap();
            let end: u64 = parts.next().unwrap().parse().unwrap();
            (start, end)
        })
        .collect::<Vec<(u64, u64)>>();

    let mut ids = vec![];
    for (from, to) in ranges {
        for id in from..=to {
            ids.push(id);
        }
    }
    println!("IDs: {:?}", ids.len());

    task1(ids.clone());
    task2(ids);
}

fn task1(ids: Vec<u64>) {
    let mut sum_of_ids: u64 = 0;

    for id in ids {
        let as_str = format!("{id}");
        let str_len = as_str.len();
        let first_half = as_str.chars().take(str_len / 2);
        let second_half = as_str.chars().skip(str_len / 2);
        if first_half.eq(second_half) {
            sum_of_ids += id;
        }
    }
    println!("Task1: Sum of IDs: {}", sum_of_ids);
}
fn task2(ids: Vec<u64>) {
    let mut sum_of_ids: u64 = 0;

    for id in ids {
        check_id(&mut sum_of_ids, id);
    }

    println!("Task2: Sum of IDs: {}", sum_of_ids);
}

fn check_id(sum_of_ids: &mut u64, id: u64) {
    let as_str = format!("{id}");
    let str_len = as_str.len();

    for i in 1..str_len {
        if str_len % i != 0 {
            continue;
        }
        let prefix: String = as_str.chars().take(i).collect();
        let parts = str_len / i;
        for part in 0..parts {
            let start = part * i;
            let segment: String = as_str.chars().skip(start).take(i).collect();
            if !prefix.eq(&segment) {
                break;
            }
            // if we are at the last part and all parts are equal
            // add to sum
            if part == parts - 1 {
                println!("Found repeating ID: {}", id);
                *sum_of_ids += id;
                return;
            }
        }
    }
}
