use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

fn room_enterable(board: &Board, colour: AmpipodColour) -> bool {
    board.rooms[dest_col(colour)].all(|x| x == Cell::Empty || x == Cell::Occupied(colour))
}

fn room_hall_pos(dest_room: usize) -> usize {
    3 + 2 * dest_room
}

fn calc_steps_from_hallway(hall_pos: usize, dest_room: usize, num_in_room: usize) -> {
    let hall_dest: i32 = room_hall_pos(dest_room) as i32;

    let hall_steps: usize = (hall_dest - hall_pos as i32).abs() as usize;

    let room_steps = 4 - num_in_room;
    
    room_steps + hall_steps
}

fn clear_path_to_room(board: &Board, hall_pos: usize, dest_room: usize) -> bool {
    let dest_pos = room_hall_pos(dest_room);
    
    if dest_pos > hall_pos {
        board.hallway[(hall_pos+1)..=dest_pos].all(|x| x == Cell::Empty)
    } else {
        board.hallway[dest_pos..hall_pos].all(|x| x == Cell::Empty)
    }
}

/// Return (new hall pos, number of steps within hallway to reach)
fn get_hallway_moves(board: &board, room: usize) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();

    let start_pos = room_hall_pos(room);

    let room_doors = [0,1,2,3].map(room_hall_pos).collect::<Vec<usize>>();

    let mut steps = 0;
    for pos in (start_pos+1)..board.hallway.len() {
        steps += 1;
        if board.hallway[pos] != Cell::Empty {
            break;
        }

        if room_doors.contains(pos) {
            continue;
        }

        moves.push((pos, steps));
    }

    for pos in 0..(start_pos-1) {
        steps += 1;
        if board.hallway[pos] != Cell::Empty {
            break;
        }

        if room_doors.contains(pos) {
            continue;
        }

        moves.push((pos, steps));
    }

    moves
}

// Return a list of possible moves + their cost
fn get_moves(board: &Board) -> Vec<Board> {
    let mut moves = Vec::new();
    for (idx, Amphipod { colour, pos }) in board.amphipods.iter().enumerate() {
        match pos {
            Position::Hallway(pos) => {
                if room_enterable(board, colour) {
                    let dest_room = dest_col(colour);
                    if clear_path_to_room(board, dest_col(colour), pos) {
                        let mut new_board = board.clone();
                        new_board.hallway[pos] = Cell::Empty;
                        let num_in_room = new_board.rooms[dest_room].len();
                        new_board.rooms[dest_room].push(colour);

                        let move_cost = calc_steps_from_hallway(pos, dest_room, num_in_room) * cell_cost(colour);
                        new_board.cost += move_cost;
                        new_board.amphipods[idx] = Amphipod {colour: colour, pos: Position::Column((dest_room, 4 - num_in_room + 1))};
                        moves.push(new_board);
                    }
                }
            }
            Position::Column((colnum, depth)) => {
                if colnum != dest_col(colour) {
                    if board.rooms[colnum].len() > (4 - depth + 1) {
                        // top of the pile, can move to hallway
                        for (hallway_pos, hallway_steps) in get_hallway_moves(board, colnum) {
                            // Add to moves
                            let mut new_board = board.clone();
                            let num_in_room = new_board.rooms[colnum].len();
                            new_board.rooms[colnum].pop();
                            new_board.hallway[hallway_pos] = Cell::Occupied(colour);

                            new_board.amphipos[idx] = Amphipod {colour, pos: Position::Hallway(hallway_pos)};

                            let move_cost = calc_steps_from_hallway(hallway_pos, colnum, num_in_room) * cell_cost(colour);
                            new_board.cost += move_cost;

                            moves.push(new_board);
                        }
                    }
                }
            }
        }
    }

    moves
}

struct Amphipod {
    colour: AmphipodColour,
    pos: Position
}

enum Position {
    Hallway(usize),
    Column((usize, usize)), // column number, depth
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Board {
    hallway: Vec<Cell>,
    rooms: Vec<Vec<AmphipodColour>>,

    amphipods: Vec<Amphipod>,

    cost: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Cell {
    Empty,
    Occupied(AmphipodColour) // amphipod id
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Nope => write!(f, "#"),
            Cell::A => write!(f, "A"),
            Cell::B => write!(f, "B"),
            Cell::C => write!(f, "C"),
            Cell::D => write!(f, "D"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct SearchNode {
    cost: usize,
    state: Vec<Cell>,
}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.state.cmp(&other.state))
    }
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn calc_neighbours(x: usize, y: usize) -> Vec<(usize, usize)> {
    vec![
        (x-1, y-1),
        (x, y-1),
        (x+1, y-1),
        (x-1, y),
        (x+1, y),
        (x-1, y+1),
        (x, y+1),
        (x+1, y+1)
    ]
}

fn draw_grid(grid: &Vec<Cell>, xlen: usize, ylen: usize) {
    for y in 0..ylen {
        for x in 0..xlen {
            print!("{}", grid[y * xlen + x]);
        }
        print!("\n");
    }
}

fn move_cell(mut state: Vec<Cell>, pos: (usize, usize), dest: (usize, usize), xlen: usize) -> Vec<Cell> {
    let cell = state[pos.1 * xlen + pos.0];

    state[pos.1 * xlen + pos.0] = Cell::Empty;  

    let old_cell = state[dest.1 * xlen + dest.0];
    state[dest.1 * xlen + dest.0] = cell;

    if old_cell != Cell::Empty {
        panic!("Tried to move into non-empty cell");
    }
    //dbg!((pos, dest, cell, old_cell));

    state
}

fn cell_cost(colour: AmphipodColour) -> usize {
    match colour {
        AmphipodColour::A => 1,
        AmphipodColour::B => 10,
        AmphipodColour::C => 100,
        AmphipodColour::D => 1000,
    }
}

fn dest_col(cell: AmphipodColour) -> usize {
    match cell {
        AmphipodColour::A => 1,
        AmphipodColour::B => 2,
        AmphipodColour::C => 3,
        AmphipodColour::D => 4,
    }
}

fn cost_to_dest(top: Cell, bottom: Cell, current_col: i32) -> usize {
    // Assume shortest path to the bottom row
    let bottom_dest_col: i32 = dest_col(bottom);
    let top_dest_col: i32 = dest_col(top);

    if top == bottom && current_col == bottom_dest_col {
        return 0;
    }

    let top_cost = cell_cost(top) * (3 + 2 * (current_col - top_dest_col).abs() as usize);

    let bottom_cost = cell_cost(bottom) * (4 + 2 * (current_col - bottom_dest_col).abs() as usize);
    if bottom_dest_col == current_col {
        return top_cost;
    }

    if top_dest_col == current_col {
        return 2 * cell_cost(top) + bottom_cost;
    }

    return top_cost + bottom_cost;
}

fn heuristic(grid: &Vec<Cell>, xlen: usize) -> usize {
    let mut cost_underestimate = 0;

    for col in 0..=3 {
        let top = grid[2 * xlen + 3 + 2*col];
        let bottom = grid[3 * xlen + 3 + 2*col];
        cost_underestimate += cost_to_dest(top, bottom, col as i32);
    }
    
    cost_underestimate
}

fn worth_moving(grid: &Vec<Cell>, x: usize, y: usize, xlen: usize, dest: &Vec<Cell>) -> bool
{
    // If we're on the bottom row & we match the dest, no point moving
    if y == 3 {
        if grid[y * xlen + x] == dest[y * xlen + x] {
            false
        }
        else {
            true
        }   
    } else if y == 2 {
        // if we're on the second row and us & the row below match, no point moving
        if grid[y * xlen + x] == dest[y * xlen + x] && grid[(y + 1) * xlen + x] == dest[(y + 1) * xlen + x] {
            return false
        } else {
            true
        }
    } else {
        true
    }
}

fn day23(grid: Vec<Cell>, dest: Vec<Cell>, ylen: usize, xlen: usize) -> usize {
    let mut heap: BinaryHeap<SearchNode> = BinaryHeap::new();

    dbg!(&dest);
    heap.push(SearchNode {
        cost: 0,
        state: grid.clone(),
    });

    let mut dists: HashMap<Vec<Cell>, usize> = HashMap::new();

    let mut moves = 0;

    while let Some(node) = heap.pop() {
        moves += 1;

        if moves%1000 == 0 {
            println!("Checked {}. Current cost: {}", moves, node.cost);
            draw_grid(&node.state, xlen, ylen);
        }
        //dbg!(&node, heap.len());
        if node.state == dest {
            println!("Found dest");
            draw_grid(&node.state, 13, 5);
            return node.cost;
        }

        if let Some(prev_found_cost) = dists.get(&node.state) {
            if node.cost > *prev_found_cost {
                // heap contains a better route to this node already
                continue;
            }
        }

        for y in 1..ylen {
            for x in 1..xlen {
                let cell = node.state[y * xlen + x];
                match cell {
                    Cell::Nope => (),
                    Cell::Empty => (),
                    Cell::A | Cell::B | Cell::C | Cell::D => {
                        if !worth_moving(&node.state, x, y, xlen, &dest) {
                            // already in the right place, don't bother
                            continue;
                        }   
                        for n in calc_neighbours(x, y) {
                            let n_cell = node.state[n.1 * xlen + n.0];

                            if n_cell == Cell::Empty {
                                let next_state = move_cell(node.state.clone(), (x, y), n, xlen);
                                let next_cost = node.cost + cell_cost(cell) + heuristic(&next_state, xlen);

                                if let Some(prev_found_cost) = dists.get(&next_state) {
                                    if next_cost >= *prev_found_cost {
                                        continue;
                                    }
                                }

                                heap.push(SearchNode {
                                    cost: next_cost,
                                    state: next_state.clone(),
                                });
                                dists.insert(next_state, next_cost);
                            }
                        }
                    }
                }
            }
        }
    }

    panic!("No route");
}

fn read_cell(ch: char) -> Cell {
    match ch {
        '#' => Cell::Nope,
        ' ' => Cell::Nope,
        '.' => Cell::Empty,
        'A' => Cell::A,
        'B' => Cell::B,
        'C' => Cell::C,
        'D' => Cell::D,
        _ => panic!("Bad cell {}", ch),
    }
}

fn read_input(input: &str) -> Vec<Cell> {
    let lines = input.lines().collect::<Vec<&str>>();
    let width = lines[0].len();

    let mut cells = Vec::with_capacity(lines.len() * width);

    for line in &lines {
        for ch in line.chars() {
            cells.push(read_cell(ch));
        }
    }

    cells
}

#[test]
fn day23_example() {
    let input = "#############
#...........#
###B#C#B#D###
    #A#D#C#A#
    #########";

    let dest = read_input(&std::fs::read_to_string("./input/day23_dest.txt").unwrap());
    let cells = read_input(&input);

    assert_eq!(day23(cells, dest, 5, 13), 12521);
}

#[test]
fn day23_actual() {
    let input = std::fs::read_to_string("./input/day23.txt").unwrap();

    let dest = read_input(&std::fs::read_to_string("./input/day23_dest.txt").unwrap());

    let cells = read_input(&input);

    assert_eq!(day23(cells, dest, 5, 13), 1615);
}
