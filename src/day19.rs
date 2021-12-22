use std::collections::HashSet;
use std::collections::HashMap;

type Position = (i32, i32, i32);

fn diff_pos(pos: &Position, other: &Position) -> Position {
    (pos.0 - other.0, pos.1 - other.1, pos.2 - other.2)
}

fn diff_add(pos: &Position, other: &Position) -> Position {
    (pos.0 + other.0, pos.1 + other.1, pos.2 + other.2)
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    PositiveZ,
    NegativeZ,
    PositiveY,
    NegativeY,
    PositiveX,
    NegativeX,
}

fn get_transform_by_id(transform_id: usize) -> (Direction, usize) {
    let mut transforms: Vec<(Direction, usize)> = Vec::new();

    for dir in [
        Direction::PositiveZ,
        Direction::NegativeZ,
        Direction::PositiveY,
        Direction::NegativeY,
        Direction::PositiveX,
        Direction::NegativeX,
    ] {
        for rotation in [0, 90, 180, 270] {
            transforms.push((dir, rotation));
        }
    }

    transforms[transform_id]
}

fn harry_transform_by_id(rotation: usize, pos: &Position) -> Position
{
  let mut out: Position = (0,0,0);
  match rotation {
    0 => { out.0 =  pos.0; out.1 =  pos.1; out.2 = pos.2; },
    1 => { out.0 =  pos.1; out.1 = -pos.0; out.2 = pos.2; },
    2 => { out.0 = -pos.1; out.1 =  pos.0; out.2 = pos.2; },
    3 => { out.0 = -pos.0; out.1 = -pos.1; out.2 = pos.2; },

    4 => { out.0 =  pos.2; out.1 =  pos.0; out.2 = pos.1; },
    5 => { out.0 =  pos.0; out.1 = -pos.2; out.2 = pos.1; },
    6 => { out.0 = -pos.0; out.1 =  pos.2; out.2 = pos.1; },
    7 => { out.0 = -pos.2; out.1 = -pos.0; out.2 = pos.1; },

    8 =>  { out.0 =  pos.1; out.1 =  pos.2; out.2 = pos.0; },
    9 =>  { out.0 =  pos.2; out.1 = -pos.1; out.2 = pos.0; },
    10 => { out.0 = -pos.2; out.1 =  pos.1; out.2 = pos.0; },
    11 => { out.0 = -pos.1; out.1 = -pos.2; out.2 = pos.0; },

    12 => { out.0 =  pos.1; out.1 =  pos.0; out.2 = -pos.2; },
    13 => { out.0 =  pos.0; out.1 = -pos.1; out.2 = -pos.2; },
    14 => { out.0 = -pos.0; out.1 =  pos.1; out.2 = -pos.2; },
    15 => { out.0 = -pos.1; out.1 = -pos.0; out.2 = -pos.2; },

    16 => { out.0 =  pos.0; out.1 =  pos.2; out.2 = -pos.1; },
    17 => { out.0 =  pos.2; out.1 = -pos.0; out.2 = -pos.1; },
    18 => { out.0 = -pos.2; out.1 =  pos.0; out.2 = -pos.1; },
    19 => { out.0 = -pos.0; out.1 = -pos.2; out.2 = -pos.1; },

    20 => { out.0 =  pos.2; out.1 =  pos.1; out.2 = -pos.0; },
    21 => { out.0 =  pos.1; out.1 = -pos.2; out.2 = -pos.0; },
    22 => { out.0 = -pos.1; out.1 =  pos.2; out.2 = -pos.0; },
    23 => { out.0 = -pos.2; out.1 = -pos.1; out.2 = -pos.0; },
    _ => panic!("Unhandled transform {:?}", rotation),
  }
  return out;
}

fn transform_by_id(transform_id: usize, pos: &Position) -> Position {
    let (dir, rot) = get_transform_by_id(transform_id);

    transform(dir, rot, pos)
}

fn transform(dir: Direction, rotation: usize, pos: &Position) -> Position {
    // face +z: ( 5, 6, 7), (-5,-6,-7) //  = (x,y,z)
    // face -z: (-5, 6,-7), ( 5,-6, 7) //  = (-x,y,-z)

    // face  y: ( 5,-7, 6), (-5, 7,-6) //  = (x,-z,y)
    // face -y: ( 5, 7,-6), (-5, 7,-6) //  = (x,z,-y)

    // face  x: (-7, 6, 5), ( 7,-6,-5)  // = (-z,y,x)
    // face -x: ( 7, 6,-5), ( 5,-6,-6)  // = (z,y,-x)

    let pos = match dir {
        Direction::PositiveZ => pos.clone(),
        Direction::NegativeZ => (-pos.0, pos.1, -pos.2),
        Direction::PositiveY => (pos.0, -pos.2, pos.1),
        Direction::NegativeY => (pos.0, pos.2, -pos.1),
        Direction::PositiveX => (-pos.2, pos.1, pos.0),
        Direction::NegativeX => (pos.2, pos.1, -pos.0),
    };
    
    // rot  0 = (x,y,z)
    // rot 90 = (x,-y,z)
    // rot180 = (-x,-y,z)
    // rot270 = (x,-y,z)
    let pos = match rotation {
        0 => pos.clone(),
        90 => (pos.0, -pos.1, pos.2),
        180 => (-pos.0, -pos.1, pos.2),
        270 => (-pos.0, pos.1, pos.2),
        _ => panic!("Unsupported rotation {:?}", rotation),
    };
    
    pos
}

fn compute_node_diffs(scanners: &[Vec<Position>]) -> Vec<Vec<Vec<HashSet<Position>>>> {
    let mut scanner_diffs = Vec::new();
    for scanner in scanners {
        let mut orientations = Vec::new();
        for transform_id in 0..24 {
            let mut nodes_in_orientation = Vec::new();

            let rotated_nodes: Vec<Position> = scanner
                .iter()
                .map(|x| harry_transform_by_id(transform_id, x))
                .collect();
            for n1 in 0..scanner.len() {
                let mut node_orientation_diffs = HashSet::new();
                for n2 in 0..scanner.len() {
                    if n1 == n2 {
                        continue;
                    }

                    let node1 = &rotated_nodes[n1];
                    let node2 = &rotated_nodes[n2];
                    node_orientation_diffs.insert((
                        node2.0 - node1.0,
                        node2.1 - node1.1,
                        node2.2 - node1.2,
                    ));
                }
                nodes_in_orientation.push(node_orientation_diffs);
            }
            orientations.push(nodes_in_orientation);
        }
        scanner_diffs.push(orientations);
    }

    scanner_diffs
}

type Orientation = usize;

fn find_match(
    merged_spaces: &HashSet<usize>,
    diffs: &Vec<Vec<Vec<HashSet<Position>>>>,
    required_matches: usize,
    scanners: &Vec<Vec<Position>>,
) -> Option<(usize, usize, Position, usize)> {
    for s1 in 0..scanners.len() {
        for s2 in 0..scanners.len() {
            if s1 == s2 || merged_spaces.contains(&s1) || merged_spaces.contains(&s2) {
                continue;
            }

            println!("Checking {} against {}", s1, s2);
            if let Some((s1_dir, n1, n2)) =
                find_match_threshold(&diffs[s1], &diffs[s2], required_matches)
            {
                let match_point = harry_transform_by_id(s1_dir, &scanners[s1][n1]);

                println!("Matched at {:?} {:?}", &match_point, &scanners[s2][n2]);
                return Some((s1, s1_dir, diff_pos(&scanners[s2][n2], &match_point), s2));
            }
        }
    }

    None
}

fn manhatten_len(pos: Position) -> usize {
    (pos.0.abs() + pos.1.abs() + pos.2.abs()).try_into().unwrap()
}

fn day19(scanners: Vec<Vec<Position>>, required_matches: usize) -> usize {
    day19_impl(scanners, required_matches).0
}

fn day19_2(scanners: Vec<Vec<Position>>, required_matches: usize) -> usize {
    day19_impl(scanners, required_matches).1
}

fn day19_impl(mut scanners: Vec<Vec<Position>>, required_matches: usize) -> (usize, usize) {
    let mut merged_spaces: HashSet<usize> = HashSet::new();

    let mut offsets: HashMap<usize, Vec<(usize, Position)>> = HashMap::new();

    while (scanners.len() - merged_spaces.len()) > 1 {
        let diffs = compute_node_diffs(&scanners);
        println!(
            "Computed node diffs. Found matches for {:?}",
            &merged_spaces
        );

        if let Some((s1, s1_dir, s1_offset, s2)) =
            find_match(&merged_spaces, &diffs, required_matches, &scanners)
        {
            println!("{:?}", (s1, s1_dir, s1_offset, s2));
            
            let s1_offsets = offsets.entry(s1).or_default().clone();

            let s2_offsets = offsets.entry(s2).or_insert_with(Vec::new);
            s2_offsets.push((s1, s1_offset));

            for x in s1_offsets {
                s2_offsets.push((x.0, diff_add(&s1_offset, &harry_transform_by_id(s1_dir, &x.1))));
            }

            let mut points: HashSet<Position> = HashSet::from_iter(scanners[s2].iter().cloned());
            points.extend(
                scanners[s1]
                    .iter()
                    .map(|x| diff_add(&harry_transform_by_id(s1_dir, x), &s1_offset)),
            );

            scanners[s2] = points.into_iter().collect();
            merged_spaces.insert(s1);

            println!(
                "Added {} in orientation {:?} nodes to {}, which now has {} nodes",
                s1,
                get_transform_by_id(s1_dir),
                s2,
                scanners[s2].len()
            );
        } else {
            panic!("No progress made. {:?} are merged", &merged_spaces);
        }
    }

    let unmerged_idx = (0..scanners.len()).filter(|idx| !merged_spaces.contains(idx)).next().unwrap();

    let mut positions = offsets[&unmerged_idx].clone();
    positions.push((unmerged_idx, (0,0,0)));
    
    println!("Found {} positions: {:?}", positions.len(), &positions);

    let zero_position = positions.iter().filter(|(scanner, _)| *scanner == 0).next().unwrap().1;

    let positions: Vec<(usize, Position)> = positions.iter().map(|(scanner, pos)| (*scanner, diff_pos(&zero_position, pos))).collect();

    println!("Found {} positions: {:?}", positions.len(), &positions);

    let mut max_dist = 0;
    for pos in 0..positions.len() {
        for pos2 in 0..positions.len() {
            if pos == pos2 {
                continue;
            }

            let new_value = manhatten_len(diff_pos(&positions[pos2].1, &positions[pos].1));
            if new_value > max_dist {
                println!("Found new max {} {:?} {:?}", new_value, positions[pos], positions[pos2]);   
            }
            
            max_dist = std::cmp::max(max_dist, new_value);
        }
    }

    (scanners[unmerged_idx].len(), max_dist)
}

fn find_match_threshold(
    s1_diffs: &Vec<Vec<HashSet<Position>>>,
    s2_diffs: &Vec<Vec<HashSet<Position>>>,
    required_matches: usize,
) -> Option<(Orientation, usize, usize)> {
    for (id1, orientation1) in s1_diffs.iter().enumerate() {
        for (n1idx, n1) in orientation1.iter().enumerate() {
            for (n2idx, n2) in s2_diffs[0].iter().enumerate() { // ???
                // count number of diffs which match between n1 and n2
                let count = n1.iter().filter(|x| n2.contains(x)).count();
                // required_matches -1 because n1/n2 are implied to match
                if count >= (required_matches - 1) {
                    println!("matched {} times at node", count);
                    // can probably break assuming 12 == only match
                    return Some((id1, n1idx, n2idx));
                } else if count > 0 {
                    //println!("Only matched {} times", count);
                }
            }
        }
    }
    return None;
}

fn read_input(input: &str) -> Vec<Vec<Position>> {
    let mut scanner_results: Vec<Vec<Position>> = Vec::new();

    for scanner_input in input.split("\n\n") {
        let mut scanner = Vec::new();

        for beacon in scanner_input.split("\n").skip(1) {
            let dims = beacon
                .split(",")
                .map(|x| str::parse::<i32>(x).unwrap())
                .collect::<Vec<i32>>();

            scanner.push((dims[0], dims[1], dims[2]));
        }

        scanner_results.push(scanner);
    }

    scanner_results
}

#[test]
fn day19_compare_transforms() {
    let p1 = (5,6,7);
    //let p2 = (-5,-6,-7);

    'outer: for tid in 0..24 {
        for tid2 in 0..24 {
            if transform_by_id(tid, &p1) == harry_transform_by_id(tid2, &p1) {
                println!("{:?} {} matches harry's {}", get_transform_by_id(tid), tid, tid2);
                continue 'outer;
            }
        }

        println!("{:?} {} did not match any of harrys", get_transform_by_id(tid), tid);
    }

    for tid in 0..24 {
        println!("transform id {} {:?}", tid, get_transform_by_id(tid));
        //assert_eq!(transform_by_id(tid, &p1), harry_transform_by_id(tid, &p1));
        //assert_eq!(transform_by_id(tid, &p2), harry_transform_by_id(tid, &p2));
    }
}

#[test]
fn day19_example1() {
    let input = "--- scanner 0 ---
0,2,0
4,1,0
3,3,0

--- scanner 1 ---
-1,-1,0
-5,0,0
-2,1,0
10,10,0";

    let input = read_input(&input);

    assert_eq!(day19(input, 3), 4);
}

#[test]
fn day19_example2() {
    let input = std::fs::read_to_string("./input/day19_example1.txt").unwrap();

    let input = read_input(&input);

    assert_eq!(day19(input, 6), 6);
}

#[test]
fn day19_example2_1() {
    let input = std::fs::read_to_string("./input/day19_example1.txt").unwrap();

    /* should output all nodes being able to transform to all nodes, but actually outputs:
    running 1 test 0 rotated by (PositiveY, 180) matches 1
    0 rotated by (PositiveX, 0) matches 2
    0 rotated by (NegativeX, 180) matches 3
    1 rotated by (PositiveY, 180) matches 0
    1 rotated by (NegativeX, 180) matches 4
    2 rotated by (NegativeX, 0) matches 0
    2 rotated by (PositiveZ, 180) matches 3
    3 rotated by (NegativeX, 180) matches 0
    3 rotated by (PositiveZ, 180) matches 2
    4 rotated by (NegativeX, 180) matches 1
    test day19::day19_example2_1 ... ok
    */
    let input = read_input(&input);

    for (s1idx, s1) in input.iter().enumerate() {
        for (s2idx, s2) in input.iter().enumerate() {
            if s1idx == s2idx {
                continue;
            }

            for orientation in 0..24 {
                let rotated: Vec<Position> = s1.iter().map(|x| harry_transform_by_id(orientation, x)).collect();

                if *s2 == rotated {
                    println!("{} rotated by {:?} matches {}", s1idx, get_transform_by_id(orientation), s2idx);
                }
            }
        }
    }

    //assert_eq!(day19(input, 6), 6);
}

#[test]
fn day19_example3() {
    let input = std::fs::read_to_string("./input/day19_example.txt").unwrap();

    let input = read_input(&input);

    assert_eq!(day19(input, 12), 79);
}

#[test]
fn day19_2_example3() {
    let input = std::fs::read_to_string("./input/day19_example.txt").unwrap();

    let input = read_input(&input);

    assert_eq!(day19_2(input, 12), 3621);
}

#[test]
fn day19_actual() {
    let input = std::fs::read_to_string("./input/day19.txt").unwrap();

    let input = read_input(&input);

    assert_eq!(day19(input, 12), 330);
}

#[test]
fn day19_2_example2() {
    let input = std::fs::read_to_string("./input/day19_example1.txt").unwrap();

    let input = read_input(&input);

    assert_eq!(day19_2(input, 6), 0);
}

#[test]
fn day19_2_actual() {
    let input = std::fs::read_to_string("./input/day19.txt").unwrap();

    let input = read_input(&input);

    assert_eq!(day19_2(input, 12), 9634);

    // 24382 too high
    // 21793 too high
    // not 24382
}
