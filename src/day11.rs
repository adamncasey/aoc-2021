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
    if y > 0 && x > 0 {
        result.push((x - 1, y - 1));
    }
    if y > 0 && x < (xbounds - 1) {
        result.push((x + 1, y - 1));
    }
    if y < (ybounds - 1) && x > 0 {
        result.push((x - 1, y + 1));
    }
    if y < (ybounds - 1) && x < (xbounds - 1) {
        result.push((x + 1, y + 1));
    }

    result
}

fn day11_step(octo: &mut Vec<Vec<u32>>) -> usize {
    let mut total_flashes = 0;
    let mut to_flash = Vec::new();
    for y in 0..octo.len() {
        for x in 0..octo[y].len() {
            octo[y][x] += 1;

            if octo[y][x] > 9 {
                dbg!((octo[y][x], x, y));
                to_flash.push((x, y));
            }
        }
    }

    while let Some(flashing) = to_flash.pop() {
        if octo[flashing.1][flashing.0] == 0 {
            continue;
        }

        total_flashes += 1;
        octo[flashing.1][flashing.0] = 0;

        let sqs = neighbours(flashing.0, flashing.1, 10, 10);

        for sq in sqs {
            let val = &mut octo[sq.1][sq.0];
            match *val {
                0 => {}
                9..=10 => {
                    to_flash.push(sq);
                }
                1..=8 => {
                    *val += 1;
                }
                x => {
                    panic!("Nope {}", x);
                }
            }
        }
    }

    total_flashes
}

fn day11(octo: &mut Vec<Vec<u32>>, days: usize) -> usize {
    let mut total = 0;
    for _day in 0..days {
        total += day11_step(octo);
    }

    total
}

fn day11_2(octo: &mut Vec<Vec<u32>>, days: usize) -> usize {
    let mut day = 1;
    loop {
        let flashes = day11_step(octo);

        if flashes == 100 {
            return day;
        }

        day += 1;
    }
}

fn read_input<'a>(input: String) -> Vec<Vec<u32>> {
    let input: Vec<Vec<u32>> = input
        .split("\n")
        .map(|x| {
            x.chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();
    input
}

#[test]
fn day11_example() {
    let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    let mut input = read_input(input.to_string());
    //assert_eq!(day11(&mut input, 1), 0);
    //assert_eq!(day11(&mut input, 1), 20);

    assert_eq!(day11(&mut input, 100), 1656);
}

#[test]
fn day11_actual() {
    let input = std::fs::read_to_string("./input/day11.txt").unwrap();

    let mut input = read_input(input.to_string());

    assert_eq!(day11(&mut input, 100), 1615);
}

#[test]
fn day11_2_example() {
    let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    let mut input = read_input(input.to_string());

    assert_eq!(day11_2(&mut input, 100), 195);
}

#[test]
fn day11_2_actual() {
    let input = std::fs::read_to_string("./input/day11.txt").unwrap();

    let mut input = read_input(input.to_string());

    assert_eq!(day11_2(&mut input, 100), 249);
}
