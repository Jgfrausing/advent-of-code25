fn main() {
    task2();
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Origin {
    SplitLeft,
    SplitRight,
    NoSplit,
}

fn task2() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read file");
    let mut lines = input.lines();

    let index_of_start = lines.next().unwrap().find('S').unwrap();
    let mut beams = vec![vec![(index_of_start, Origin::NoSplit)]];
    while let Some(line) = lines.next() {
        let mut new_beams = Vec::new();
        for (idx, cell) in line.chars().enumerate() {
            let beam_above = beams
                .iter()
                .last()
                .unwrap()
                .iter()
                .any(|(id, _)| id == &idx);
            if !beam_above {
                continue;
            }
            match cell {
                '.' => {
                    new_beams.push((idx, Origin::NoSplit));
                }
                '^' => {
                    new_beams.push((idx - 1, Origin::SplitLeft));
                    new_beams.push((idx + 1, Origin::SplitRight));
                }
                _ => panic!("Unknown cell type {}", cell),
            }
        }
        beams.push(new_beams);
    }

    let mut paths_to_idx = vec![(index_of_start, 1)];

    for beam_line in beams {
        let mut new_paths_to_idx = vec![];
        for (beam_idx, origin) in beam_line {
            let sum_of_paths: u64 = paths_to_idx
                .iter()
                .filter_map(|(idx, count)| {
                    if (idx == &beam_idx && origin == Origin::NoSplit)
                        || *idx == &beam_idx + 1 && origin == Origin::SplitLeft
                        || *idx == &beam_idx - 1 && origin == Origin::SplitRight
                    {
                        Some(*count)
                    } else {
                        None
                    }
                })
                .sum();
            if new_paths_to_idx.iter().any(|(idx, _)| idx == &beam_idx) {
                let existing = new_paths_to_idx
                    .iter_mut()
                    .find(|(idx, _)| idx == &beam_idx)
                    .unwrap();
                existing.1 += sum_of_paths;
            } else {
                new_paths_to_idx.push((beam_idx, sum_of_paths));
            }
        }
        paths_to_idx = new_paths_to_idx;
    }

    let paths = paths_to_idx.iter().map(|(_, count)| count).sum::<u64>();

    assert_eq!(paths, 40);
}

fn task1() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read file");
    let mut lines = input.lines();

    let mut beams = vec![lines.next().unwrap().find('S').unwrap()];
    let mut splits = 0;
    while let Some(line) = lines.next() {
        let mut new_beams = Vec::new();
        for (idx, cell) in line.chars().enumerate() {
            let beam_above = beams.contains(&idx);
            if !beam_above {
                continue;
            }
            match cell {
                '.' => {
                    new_beams.push(idx);
                }
                '^' => {
                    splits += 1;
                    new_beams.push(idx - 1);
                    new_beams.push(idx + 1);
                }
                _ => panic!("Unknown cell type {}", cell),
            }
        }
        beams = new_beams;
        print_beams(&beams, line.len());
    }

    assert_eq!(splits, 21);
}

fn print_beams(beams: &Vec<usize>, width: usize) {
    let mut line = String::new();
    for i in 0..width {
        if beams.contains(&i) {
            line.push('|');
        } else {
            line.push('.');
        }
    }
    println!("{}", line);
}
