fn day1(input: &[u32]) -> usize {
    let mut count = 0;
    for (prev, next) in input.iter().zip(input[1..].iter()) {
        if next > prev {
            count += 1;
        }
    }

    count
}

fn day1_part2(input: &[u32]) -> usize {
    let window_sums: Vec<u32> = input
        .windows(3)
        .map(|x| x.iter().map(|x| *x).sum())
        .collect();

    dbg!(&window_sums);

    day1(&window_sums)
}

#[test]
fn day1_example() {
    let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    assert_eq!(day1(&input), 7);
}

#[test]
fn day1_actual() {
    let input: Vec<u32> = std::fs::read_to_string("./input/day1.txt")
        .unwrap()
        .lines()
        .map(|str| str::parse::<u32>(str).unwrap())
        .collect();

    assert_eq!(day1(&input), 1215);
}

#[test]
fn day1_part2_example() {
    let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    assert_eq!(day1_part2(&input), 5);
}

#[test]
fn day1_part2_actual() {
    let input: Vec<u32> = std::fs::read_to_string("./input/day1.txt")
        .unwrap()
        .lines()
        .map(|str| str::parse::<u32>(str).unwrap())
        .collect();

    assert_eq!(day1_part2(&input), 1150);
}
