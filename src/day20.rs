use std::collections::HashMap;

pub fn neighbours(x: i64, y: i64) -> Vec<(i64, i64)> {
    let mut result = Vec::new();

    result.push((x - 1, y - 1));
    result.push((x, y - 1));
    result.push((x + 1, y - 1));

    result.push((x - 1, y));
    result.push((x, y));
    result.push((x + 1, y));

    result.push((x - 1, y + 1));
    result.push((x, y + 1));
    result.push((x + 1, y + 1));

    result
}

fn min_max_xy(image: &HashMap<(i64, i64), bool>) -> (i64, i64, i64, i64) {
    // Need to expand the grid by one in all directions each step
    let min_x = image.keys().map(|(x, _y)| x).min().unwrap() - 1;
    let max_x = image.keys().map(|(x, _y)| x).max().unwrap() + 1;

    let min_y = image.keys().map(|(_x, y)| y).min().unwrap() - 1;
    let max_y = image.keys().map(|(_x, y)| y).max().unwrap() + 1;

    (min_x, max_x, min_y, max_y)
}

fn read_value(x: i64, y: i64, image: &HashMap<(i64, i64), bool>, infinite_value: bool) -> usize {
    let neighbours = neighbours(x, y);

    let mut value = 0;
    for n in neighbours {
        value = value << 1;
        value |= if *image.get(&n).unwrap_or(&infinite_value) {
            0b1
        } else {
            0b0
        };
    }

    value
}

fn day20(input: (Vec<bool>, HashMap<(i64, i64), bool>), iterations: usize) -> usize {
    let algo = input.0;
    let mut image = input.1;

    let mut infinite_value = false;

    for step in 0..iterations {
        let mut next_image = HashMap::new();
        let bounds = min_max_xy(&image);
        let (min_x, max_x, min_y, max_y) = bounds;
        println!(
            "Iteration {}, infinite value {} {:?}",
            step, infinite_value, bounds
        );

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let value = read_value(x, y, &image, infinite_value);

                //println!("{} {} {} {}", x, y, value, algo[value]);
                next_image.insert((x, y), algo[value]);
            }
        }

        image = next_image;
        infinite_value = if infinite_value { algo[511] } else { algo[0] };
    }

    image.iter().filter(|(_key, value)| **value).count()
}

fn read_input(input: &str) -> (Vec<bool>, HashMap<(i64, i64), bool>) {
    let (algo, grid) = input.split_once("\n\n").unwrap();

    let algo = algo
        .chars()
        .map(|x| if x == '#' { true } else { false })
        .collect();

    let mut image = HashMap::new();

    let lines = grid.lines().collect::<Vec<&str>>();
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            image.insert(
                (x as i64, y as i64),
                if lines[y].as_bytes()[x] == b'#' {
                    true
                } else {
                    false
                },
            );
        }
    }

    (algo, image)
}

#[test]
fn day20_example() {
    let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    let input = read_input(input);

    assert_eq!(day20(input, 2), 35);
}

#[test]
fn day20_actual() {
    let input = std::fs::read_to_string("./input/day20.txt").unwrap();

    let input = read_input(&input);

    assert_eq!(day20(input, 2), 5316);
    // 5518 too high
}

#[test]
fn day20_2_example() {
    let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    let input = read_input(input);

    assert_eq!(day20(input, 50), 3351);
}

#[test]
fn day20_2_actual() {
    let input = std::fs::read_to_string("./input/day20.txt").unwrap();

    let input = read_input(&input);

    assert_eq!(day20(input, 50), 16728);
    // 5518 too high
}
