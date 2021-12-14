pub fn neighbours(x: usize, y: usize, xbounds: usize, ybounds: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    if y > 0 {
        result.push((x, y - 1));
    }
    if x > 0 {
        result.push((x - 1, y));
    }
    if x < (xbounds - 1) {
        result.push((x + 1, y));
    }
    if y < (ybounds - 1) {
        result.push((x, y + 1));
    }

    result
}

fn is_lowest(value: u32, sqs: &[(usize, usize)], game: &Vec<Vec<u32>>) -> bool {
    for sq in sqs {
        if value >= game[sq.1][sq.0] {
            return false;
        }
    }

    return true;
}

fn basin_size(
    x: usize,
    y: usize,
    game: &Vec<Vec<u32>>,
    seen: &mut [bool],
    xlen: usize,
    ylen: usize,
) -> usize {
    if seen[y * xlen + x] || game[y][x] == 9 {
        return 0;
    }

    seen[y * xlen + x] = true;

    let sqs = neighbours(x, y, xlen, ylen);
    dbg!((x, y, &sqs));
    let mut total = 1;
    for sq in sqs {
        total += basin_size(sq.0, sq.1, game, seen, xlen, ylen);
    }

    total
}

fn day9(game: &Vec<Vec<u32>>) -> u32 {
    let ylen = game.len();
    let xlen = game[0].len();
    let mut total_lowest_risk = 0;

    for y in 0..game.len() {
        for x in 0..game[y].len() {
            let sqs = neighbours(x, y, xlen, ylen);

            let value = game[y][x];

            if is_lowest(value, &sqs, game) {
                total_lowest_risk += 1 + value;
            }
        }
    }

    total_lowest_risk
}

fn print_seen(seen: &Vec<bool>) {
    let seen_ch = seen
        .iter()
        .map(|s| if *s { 'x' } else { ' ' })
        .collect::<Vec<char>>();

    let seen_str: Vec<String> = seen_ch
        .chunks(10)
        .map(|c| c.iter().collect::<String>())
        .collect();
    println!("{}", seen_str.join("\n"));
}

fn day9_2(game: &Vec<Vec<u32>>) -> u32 {
    let ylen = game.len();
    let xlen = game[0].len();
    let mut seen = vec![false; ylen * xlen];

    let mut basin_sizes: Vec<usize> = Vec::new();

    for y in 0..ylen {
        for x in 0..xlen {
            let size = basin_size(x, y, game, &mut seen, xlen, ylen);
            if size > 0 {
                print_seen(&seen);
                dbg!((x, y, size));
                basin_sizes.push(size);
            }
        }
    }

    basin_sizes.sort();
    dbg!(&basin_sizes);

    basin_sizes
        .iter()
        .rev()
        .take(3)
        .cloned()
        .reduce(|a, b| a * b)
        .unwrap() as u32
}

fn read_input(input: String) -> Vec<Vec<u32>> {
    let input: Vec<Vec<u32>> = input
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    input
}

#[test]
fn day9_example() {
    let input = "2199943210
3987894921
9856789892
8767896789
9899965678";

    let input = read_input(input.to_string());

    assert_eq!(day9(&input), 15);
}

#[test]
fn day9_actual() {
    let input = std::fs::read_to_string("./input/day9.txt").unwrap();

    let input = read_input(input.to_string());

    assert_eq!(day9(&input), 496);
}

#[test]
fn day9_2_example() {
    let input = "2199943210
3987894921
9856789892
8767896789
9899965678";

    let input = read_input(input.to_string());

    assert_eq!(day9_2(&input), 1134);
}

#[test]
fn day9_2_actual() {
    let input = std::fs::read_to_string("./input/day9.txt").unwrap();

    let input = read_input(input.to_string());

    assert_eq!(day9_2(&input), 902880);
}
