fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read file");
    let lines = input.lines();
    let grid = Grid::new(lines);
    //task1(grid);
    task2(grid);
}

fn task1(mut grid: Grid) {
    let mut marked_cells = Vec::new();
    for x in 0..grid.cells.len() {
        for (y, cell) in grid.cells[x].iter().enumerate() {
            if cell.has_roll {
                let count = grid.count_surounding_rolls(x, y);
                if count < 4 {
                    marked_cells.push((x, y));
                }
            }
        }
    }
    for marked in marked_cells.iter() {
        grid.cells[marked.0][marked.1].marked_as_accessible = true;
    }
    grid.draw();

    assert_eq!(marked_cells.len(), 13);
}

fn task2(mut grid: Grid) {
    let mut marked_cells = 0;
    loop {
        let mut removed_this_iteration = Vec::new();
        for x in 0..grid.cells.len() {
            for (y, cell) in grid.cells[x].iter().enumerate() {
                if cell.has_roll {
                    let count = grid.count_surounding_rolls(x, y);
                    if count < 4 {
                        removed_this_iteration.push((x, y));
                    }
                }
            }
        }
        if removed_this_iteration.is_empty() {
            break;
        }
        for marked in removed_this_iteration.iter() {
            grid.cells[marked.0][marked.1].has_roll = false;
        }
        marked_cells += removed_this_iteration.len();
    }

    grid.draw();

    assert_eq!(marked_cells, 43);
}

struct Cell {
    has_roll: bool,
    marked_as_accessible: bool,
}
struct Grid {
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    fn new(lines: std::str::Lines<'_>) -> Self {
        let mut grid: Vec<Vec<Cell>> = vec![];

        for line in lines.clone() {
            let row: Vec<Cell> = line
                .chars()
                .map(|c| Cell {
                    has_roll: c == '@',
                    marked_as_accessible: false,
                })
                .collect();
            grid.push(row);
        }

        Self { cells: grid }
    }

    fn count_surounding_rolls(&self, x: usize, y: usize) -> u64 {
        let mut count = 0;
        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for direction in directions.iter() {
            let new_x = x as isize + direction.0;
            let new_y = y as isize + direction.1;

            let cell = self.get_cell(new_x, new_y);
            if cell.is_some_and(|c| c.has_roll) {
                count += 1;
            }
        }
        count
    }

    fn get_cell(&self, x: isize, y: isize) -> Option<&Cell> {
        if x >= 0 && y >= 0 {
            self.cells
                .get(x as usize)
                .and_then(|row| row.get(y as usize))
        } else {
            None
        }
    }
    fn draw(&self) {
        for row in self.cells.iter() {
            for cell in row.iter() {
                if cell.marked_as_accessible {
                    print!("x");
                } else if cell.has_roll {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}
