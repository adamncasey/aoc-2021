use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

fn room_enterable(board: &Board, colour: AmphipodColour) -> bool {
    let room = &board.rooms[dest_room(colour)];
    room.is_empty() || room.iter().all(|x| *x == colour)
}

fn room_hall_pos(dest_room: usize) -> usize {
    2 + 2 * dest_room
}

fn calc_steps_from_hallway(
    room_size: usize,
    hall_pos: usize,
    dest_room: usize,
    num_in_room: usize,
) -> usize {
    let hall_dest: i32 = room_hall_pos(dest_room) as i32;

    let hall_steps: usize = (hall_dest - hall_pos as i32).abs() as usize;

    let room_steps = room_size - num_in_room + 1;

    //dbg!((room_size, hall_pos, dest_room, num_in_room, room_steps, hall_steps));

    room_steps + hall_steps
}

fn clear_path_to_room(board: &Board, hall_pos: usize, dest_room: usize) -> bool {
    let dest_pos = room_hall_pos(dest_room);
    //dbg!((dest_room, dest_pos));

    if dest_pos > hall_pos {
        board.hallway[(hall_pos + 1)..=dest_pos]
            .iter()
            .all(|x| *x == Cell::Empty)
    } else {
        board.hallway[dest_pos..hall_pos]
            .iter()
            .all(|x| *x == Cell::Empty)
    }
}

/// Return (new hall pos, number of steps within hallway to reach)
fn get_hallway_moves(board: &Board, room: usize) -> Vec<usize> {
    let mut moves = Vec::new();

    let start_pos = room_hall_pos(room);

    let room_doors = vec![
        room_hall_pos(0),
        room_hall_pos(1),
        room_hall_pos(2),
        room_hall_pos(3),
    ];

    //dbg!((room, start_pos, &room_doors));

    for pos in (start_pos + 1)..board.hallway.len() {
        //println!("{}", pos);
        if board.hallway[pos] != Cell::Empty {
            break;
        }

        if room_doors.contains(&pos) {
            continue;
        }

        moves.push(pos);
    }

    for pos in (0..(start_pos)).rev() {
        if board.hallway[pos] != Cell::Empty {
            break;
        }

        if room_doors.contains(&pos) {
            continue;
        }

        moves.push(pos);
    }

    moves
}

/// Return true if everything below depth in room is the correct colour
fn room_below_ok(board: &Board, room: usize, depth: usize, expected: AmphipodColour) -> bool {
    let num_to_check = board.room_size - (depth + 1);

    if num_to_check < 1 {
        return true;
    }

    if num_to_check > board.rooms[room].len() {
        println!("Fail {:?} {} {}", board, room, depth);
    }

    board.rooms[room][0..num_to_check]
        .iter()
        .all(|x| *x == expected)
}

// Return a list of possible moves + their cost
fn get_moves(board: &Board, current_cost: usize) -> Vec<(Board, usize)> {
    let mut moves = Vec::new();
    for (idx, Amphipod { colour, pos }) in board.amphipods.iter().enumerate() {
        match pos {
            Position::Hallway(pos) => {
                if room_enterable(board, *colour) {
                    let dest_room = dest_room(*colour);
                    if clear_path_to_room(board, *pos, dest_room) {
                        //println!("Amphipod {} {:?} in hallway pos {} can move to room", idx, colour, pos);
                        let mut new_board = board.clone();
                        new_board.hallway[*pos] = Cell::Empty;
                        let num_in_room = new_board.rooms[dest_room].len();
                        new_board.rooms[dest_room].push(*colour);

                        let move_cost = calc_steps_from_hallway(
                            board.room_size,
                            *pos,
                            dest_room,
                            num_in_room + 1,
                        ) * cell_cost(*colour);

                        new_board.amphipods[idx] = Amphipod {
                            colour: *colour,
                            pos: Position::Room((dest_room, board.room_size - 1 - num_in_room)),
                        };
                        //new_board.history.push(format!("R {:?} {} to {} {}", *colour, pos, dest_room, current_cost+move_cost));
                        moves.push((new_board, current_cost + move_cost));
                    }
                }
            }
            Position::Room((colnum, depth)) => {
                let expected_room = dest_room(*colour);
                if *colnum != expected_room || !room_below_ok(board, *colnum, *depth, *colour) {
                    let top_depth = board.room_size - board.rooms[*colnum].len();
                    if top_depth == *depth {
                        //println!("Getting hallway moves for {:?}", (colnum, depth));
                        // top of the pile, can move to hallway
                        for hallway_pos in get_hallway_moves(board, *colnum) {
                            // Add to moves
                            let mut new_board = board.clone();
                            let num_in_room = new_board.rooms[*colnum].len();
                            new_board.rooms[*colnum].pop();
                            new_board.hallway[hallway_pos] = Cell::Occupied(*colour);

                            new_board.amphipods[idx] = Amphipod {
                                colour: *colour,
                                pos: Position::Hallway(hallway_pos),
                            };

                            let move_cost = calc_steps_from_hallway(
                                board.room_size,
                                hallway_pos,
                                *colnum,
                                num_in_room,
                            ) * cell_cost(*colour);
                            //new_board.history.push(format!("H {:?} {:?} to {} {}", *colour, (colnum, depth), hallway_pos, current_cost + move_cost));
                            moves.push((new_board, current_cost + move_cost));
                            //println!("Amphipod {} {:?} in room pos {:?} can move to hallway {} for {}", idx, colour, (colnum,depth), hallway_pos, move_cost);
                        }
                    } else {
                        //println!("{:?} {:?} can't exit", (colnum, depth), colour);
                    }
                } else {
                    //println!("{:?} {:?} doesn't need to move", (colnum, depth), colour);
                }
            }
        }
    }

    if moves.len() == 0 {
        //println!("No moves from {:?}", board);
    }

    moves
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Amphipod {
    colour: AmphipodColour,
    pos: Position,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AmphipodColour {
    A,
    B,
    C,
    D,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Position {
    Hallway(usize),
    Room((usize, usize)), // Room number, depth
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Board {
    hallway: Vec<Cell>,
    rooms: Vec<Vec<AmphipodColour>>,
    room_size: usize,

    amphipods: Vec<Amphipod>,
    history: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Cell {
    Empty,
    Occupied(AmphipodColour), // amphipod id
}

#[derive(Debug, PartialEq, Eq)]
struct SearchNode {
    board: Board,
    cost: usize,
}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.board.cmp(&self.board))
    }
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn cell_cost(colour: AmphipodColour) -> usize {
    match colour {
        AmphipodColour::A => 1,
        AmphipodColour::B => 10,
        AmphipodColour::C => 100,
        AmphipodColour::D => 1000,
    }
}

fn dest_room(cell: AmphipodColour) -> usize {
    match cell {
        AmphipodColour::A => 0,
        AmphipodColour::B => 1,
        AmphipodColour::C => 2,
        AmphipodColour::D => 3,
    }
}

fn draw_grid(board: &Board) {
    println!("{:?}", board);
}

fn day23(board: Board, _dest: Board) -> usize {
    let mut heap: BinaryHeap<SearchNode> = BinaryHeap::new();

    dbg!(&board);
    heap.push(SearchNode { board, cost: 0 });

    let mut dists: HashMap<Board, usize> = HashMap::new();

    let mut moves = 0;

    while let Some(node) = heap.pop() {
        moves += 1;

        if moves % 10000 == 0 {
            draw_grid(&node.board);
            println!(
                "Checked {}. Current cost: {}. Move count: {}",
                moves,
                node.cost,
                node.board.history.len()
            );
        }
        //dbg!(&node, heap.len());
        if node.board.hallway.iter().all(|x| *x == Cell::Empty)
            && node.board.rooms[0] == vec![AmphipodColour::A; node.board.room_size]
            && node.board.rooms[1] == vec![AmphipodColour::B; node.board.room_size]
            && node.board.rooms[2] == vec![AmphipodColour::C; node.board.room_size]
            && node.board.rooms[3] == vec![AmphipodColour::D; node.board.room_size]
        {
            println!("Found dest");
            draw_grid(&node.board);
            return node.cost;
        }

        if let Some(prev_found_cost) = dists.get(&node.board) {
            if node.cost > *prev_found_cost {
                // heap contains a better route to this node already
                continue;
            }
        }

        for (new_board, cost) in get_moves(&node.board, node.cost) {
            if let Some(prev_found_cost) = dists.get(&new_board) {
                if cost > *prev_found_cost {
                    // heap contains a better route to this node already
                    continue;
                }
            }

            heap.push(SearchNode {
                board: new_board.clone(),
                cost,
            });
            dists.insert(new_board, cost);
        }
    }

    panic!("No route");
}

fn read_colour(ch: char) -> AmphipodColour {
    match ch {
        'A' => AmphipodColour::A,
        'B' => AmphipodColour::B,
        'C' => AmphipodColour::C,
        'D' => AmphipodColour::D,
        _ => panic!("Bad input {}", ch),
    }
}

fn read_input(input: &str, room_height: usize) -> Board {
    let lines = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut rooms = vec![Vec::new(); 4];
    let mut amphipods = Vec::new();

    for room in 0..=3 {
        let x = 3 + 2 * room;
        for id in (0..room_height).rev() {
            let y = 2 + id;

            let colour = read_colour(lines[y][x]);
            rooms[room].push(colour);
            amphipods.push(Amphipod {
                colour,
                pos: Position::Room((room, id)),
            })
        }
    }

    Board {
        hallway: vec![Cell::Empty; 11],
        rooms,
        room_size: room_height,
        amphipods,
        history: Vec::new(),
    }
}

#[test]
fn day23_example() {
    let input = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

    let dest = read_input(
        &std::fs::read_to_string("./input/day23_dest.txt").unwrap(),
        2,
    );
    let cells = read_input(&input, 2);

    assert_eq!(day23(cells, dest), 12521);
}

#[test]
fn day23_moves_example() {
    let input = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

    let board = read_input(&input, 2);
    println!("{:?}", &board);
    let moves = get_moves(&board, 0);
    moves.iter().for_each(|x| println!("{:?}", x));

    assert_eq!(moves.len(), 28);
}

#[test]
fn day23_actual() {
    let input = std::fs::read_to_string("./input/day23.txt").unwrap();

    let dest = read_input(
        &std::fs::read_to_string("./input/day23_dest.txt").unwrap(),
        2,
    );

    let cells = read_input(&input, 2);

    assert_eq!(day23(cells, dest), 15365);
}

#[test]
fn day23_moves_out_of_room() {
    let mut board = Board {
        hallway: vec![Cell::Empty; 11],
        rooms: vec![Vec::new(); 4],
        room_size: 2,
        amphipods: Vec::new(),
        history: Vec::new(),
    };

    board.rooms[1].push(AmphipodColour::A);
    board.amphipods.push(Amphipod {
        colour: AmphipodColour::A,
        pos: Position::Room((1, 1)),
    });
    let moves = get_moves(&board, 0);
    moves.iter().for_each(|x| println!("{:?}", x));

    assert_eq!(moves.len(), 7);

    board.amphipods.push(Amphipod {
        colour: AmphipodColour::A,
        pos: Position::Room((1, 0)),
    });
    board.rooms[1].push(AmphipodColour::A);

    let moves = get_moves(&board, 0);
    moves.iter().for_each(|x| println!("{:?}", x));

    assert_eq!(moves.len(), 7);
}

#[test]
fn day23_moves_out_of_room_2() {
    let mut board = Board {
        hallway: vec![Cell::Empty; 11],
        rooms: vec![Vec::new(); 4],
        room_size: 2,
        amphipods: Vec::new(),
        history: Vec::new(),
    };

    board.rooms[0].push(AmphipodColour::A);
    board.rooms[1].push(AmphipodColour::B);
    board.rooms[1].push(AmphipodColour::B);
    board.rooms[2].push(AmphipodColour::C);
    board.rooms[3].push(AmphipodColour::A);
    board.amphipods.push(Amphipod {
        colour: AmphipodColour::A,
        pos: Position::Room((3, 1)),
    });
    board.amphipods.push(Amphipod {
        colour: AmphipodColour::A,
        pos: Position::Room((0, 0)),
    });
    board.amphipods.push(Amphipod {
        colour: AmphipodColour::B,
        pos: Position::Room((1, 1)),
    });
    board.amphipods.push(Amphipod {
        colour: AmphipodColour::B,
        pos: Position::Room((1, 0)),
    });
    board.amphipods.push(Amphipod {
        colour: AmphipodColour::C,
        pos: Position::Room((2, 1)),
    });
    board.amphipods.push(Amphipod {
        colour: AmphipodColour::C,
        pos: Position::Hallway(10),
    });
    board.amphipods.push(Amphipod {
        colour: AmphipodColour::D,
        pos: Position::Hallway(0),
    });
    board.amphipods.push(Amphipod {
        colour: AmphipodColour::D,
        pos: Position::Hallway(9),
    });
    board.hallway[0] = Cell::Occupied(AmphipodColour::D);
    board.hallway[9] = Cell::Occupied(AmphipodColour::D);
    board.hallway[10] = Cell::Occupied(AmphipodColour::C);
    let moves = get_moves(&board, 0);
    moves.iter().for_each(|x| println!("{:?}", x));

    assert_eq!(moves.len(), 4);
}

#[test]
fn day23_moves_out_of_hallway() {
    let mut board = Board {
        hallway: vec![Cell::Empty; 11],
        rooms: vec![Vec::new(); 4],
        room_size: 2,
        amphipods: Vec::new(),
        history: Vec::new(),
    };

    board.hallway[0] = Cell::Occupied(AmphipodColour::B);
    board.amphipods.push(Amphipod {
        colour: AmphipodColour::B,
        pos: Position::Hallway(0),
    });
    let moves = get_moves(&board, 0);
    moves.iter().for_each(|x| println!("{:?}", x));

    assert_eq!(moves.len(), 1);
}

#[test]
fn day23_moves_out_of_hallway2() {
    let mut board = Board {
        hallway: vec![Cell::Empty; 11],
        rooms: vec![Vec::new(); 4],
        room_size: 2,
        amphipods: Vec::new(),
        history: Vec::new(),
    };

    board.hallway[0] = Cell::Occupied(AmphipodColour::A);
    board.hallway[1] = Cell::Occupied(AmphipodColour::A);
    board.amphipods.push(Amphipod {
        colour: AmphipodColour::A,
        pos: Position::Hallway(0),
    });
    board.amphipods.push(Amphipod {
        colour: AmphipodColour::A,
        pos: Position::Hallway(1),
    });
    let moves = get_moves(&board, 0);
    moves.iter().for_each(|x| println!("{:?}", x));

    assert_eq!(moves.len(), 1);

    let (board, cost) = &moves[0];
    let moves = get_moves(&board, *cost);
    moves.iter().for_each(|x| println!("{:?}", x));
    assert_eq!(moves.len(), 1);
    assert_eq!(moves[0].1, 6);
}

#[test]
fn day23_test() {
    let input = "#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########";

    let board = read_input(&input, 2);

    assert_eq!(day23(board.clone(), board), 0);
}

#[test]
fn day23_example_breakdown1() {
    let input = "#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########";

    let dest = read_input(
        &std::fs::read_to_string("./input/day23_dest.txt").unwrap(),
        2,
    );
    let mut cells = read_input(&input, 2);
    cells.hallway[9] = Cell::Occupied(AmphipodColour::A);
    cells.rooms[0].pop();
    println!("{:?}", cells.amphipods[0]);
    cells.amphipods[0].pos = Position::Hallway(9);

    assert_eq!(day23(cells, dest), 8);
}

#[test]
fn day23_example_breakdown2() {
    let input = "#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########";

    let dest = read_input(
        &std::fs::read_to_string("./input/day23_dest.txt").unwrap(),
        2,
    );
    let mut cells = read_input(&input, 2);
    cells.hallway[9] = Cell::Occupied(AmphipodColour::A);
    cells.hallway[7] = Cell::Occupied(AmphipodColour::D);
    cells.hallway[5] = Cell::Occupied(AmphipodColour::D);
    cells.rooms[0].pop();
    cells.rooms[3].pop();
    cells.rooms[3].pop();
    println!("{:?}", cells.amphipods[0]);
    cells.amphipods[0].pos = Position::Hallway(9);
    cells.amphipods[7].pos = Position::Hallway(7);
    cells.amphipods[6].pos = Position::Hallway(5);

    assert_eq!(day23(cells, dest), 7008);
}

#[test]
fn day23_2_example() {
    let input = "#############
#...........#
###B#C#B#D###
  #D#C#B#A#
  #D#B#A#C#
  #A#D#C#A#
  #########";

    let cells = read_input(&input, 4);

    println!("{:?}", cells);

    assert_eq!(day23(cells.clone(), cells), 44169);
}

#[test]
fn day23_2_actual() {
    let input = std::fs::read_to_string("./input/day23_2.txt").unwrap();
    let cells = read_input(&input, 4);

    println!("{:?}", cells);

    assert_eq!(day23(cells.clone(), cells), 52055);
}
