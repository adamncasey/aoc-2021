#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    East,
    South,
    Empty,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::East => write!(f, ">"),
            Cell::South => write!(f, "v"),
            Cell::Empty => write!(f, "."),
        }
    }
}

fn draw_grid(grid: &Vec<Cell>, xlen: usize, ylen: usize) {
    for y in 0..ylen {
        for x in 0..xlen {
            print!("{}", grid[y * xlen + x]);
        }
        print!("\n");
    }
}

fn day25(grid: Vec<Cell>, ylen: usize, xlen: usize, max_steps: usize) -> usize {
    let mut steps = 0;

    let mut prev_grid = grid;

    loop {
        draw_grid(&prev_grid, xlen, ylen);
        steps += 1;
        let mut num_moved = 0;
        let mut next_grid = vec![Cell::Empty; prev_grid.len()];

        for y in 0..ylen {
            for x in 0..xlen {
                let cell = prev_grid[y * xlen + x];
                if cell == Cell::East {
                    let next_x = (x + 1) % xlen;
                    if prev_grid[y * xlen + next_x] == Cell::Empty {
                        next_grid[y * xlen + next_x] = Cell::East;
                        num_moved += 1;
                    } else {
                        next_grid[y * xlen + x] = Cell::East;
                    }
                } else if cell == Cell::South {
                    next_grid[y * xlen + x] = cell;
                }
            }
        }

        prev_grid = next_grid;
        //dbg!(num_moved);
        //draw_grid(&prev_grid, xlen, ylen);
        let mut next_grid = vec![Cell::Empty; prev_grid.len()];

        for y in 0..ylen {
            for x in 0..xlen {
                let cell = prev_grid[y * xlen + x];
                if cell == Cell::South {
                    let next_y = (y + 1) % ylen;
                    let next_cell = prev_grid[next_y * xlen + x];
                    if next_cell == Cell::South {
                        next_grid[y * xlen + x] = Cell::South;
                    } else if next_cell == Cell::East {
                        next_grid[y * xlen + x] = Cell::South;
                    } else {
                        next_grid[next_y * xlen + x] = Cell::South;
                        num_moved += 1;
                    }
                } else if cell == Cell::East {
                    next_grid[y * xlen + x] = cell;
                }
            }
        }

        prev_grid = next_grid;

        //dbg!((num_moved, steps));

        if num_moved == 0 || steps >= max_steps {
            break;
        }
    }

    steps
}

fn read_cell(ch: char) -> Cell {
    match ch {
        '>' => Cell::East,
        'v' => Cell::South,
        '.' => Cell::Empty,
        _ => panic!("Bad cell {}", ch),
    }
}

fn read_input(input: &str) -> (Vec<Cell>, usize, usize) {
    let lines = input.lines().collect::<Vec<&str>>();
    let width = lines[0].len();

    let mut cells = Vec::with_capacity(lines.len() * width);

    for line in &lines {
        for ch in line.chars() {
            cells.push(read_cell(ch));
        }
    }

    (cells, lines.len(), width)
}

#[test]
fn day25_example() {
    let input = "...>...
.......
......>
v.....>
......>
.......
..vvv..";

    let (cells, height, width) = read_input(&input);

    assert_eq!(day25(cells, height, width, 10), 10);
}

#[test]
fn day25_example2() {
    let input = "...>>>>>...";

    let (cells, height, width) = read_input(&input);

    assert_eq!(day25(cells, height, width, 10), 10);
}

#[test]
fn day25_example3() {
    let input = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    let (cells, height, width) = read_input(&input);

    assert_eq!(day25(cells, height, width, 100), 58);
}

#[test]
fn day25_actual() {
    let input = std::fs::read_to_string("./input/day25.txt").unwrap();

    let (cells, height, width) = read_input(&input);

    assert_eq!(day25(cells, height, width, 1000000), 1615);
}
