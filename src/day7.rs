fn day7(mut crabs: Vec<usize>) -> usize {
    crabs.sort();
    let median = crabs[crabs.len() / 2];
    dbg!(median);

    crabs
        .iter()
        .map(|pos| ((*pos as i32) - median as i32).abs() as usize)
        .sum::<usize>()
}

fn day7_2(crabs: Vec<usize>) -> usize {
    let max = *crabs.iter().max().unwrap();

    let mut targets: Vec<usize> = vec![0; max + 1];
    let mut costs = vec![0; max + 1];

    for dist in 1..=max {
        costs[dist] = dist + costs[dist - 1];
    }

    for crab in crabs {
        for target in 0..=max {
            targets[target] += costs[((crab as i32 - target as i32).abs()) as usize];
        }
    }

    *targets.iter().min().unwrap()
}

fn read_input(input: String) -> Vec<usize> {
    let input = input
        .split(",")
        .map(|x| str::parse::<usize>(x).unwrap())
        .collect();

    input
}

#[test]
fn day7_example() {
    let input = "16,1,2,0,4,2,7,1,2,14";

    let input = read_input(input.to_string());

    assert_eq!(day7(input), 37);
}

#[test]
fn day7_actual() {
    let input = std::fs::read_to_string("./input/day7.txt").unwrap();

    let input = read_input(input.to_string());

    assert_eq!(day7(input), 354129);
}

#[test]
fn day7_2_example() {
    let input = "16,1,2,0,4,2,7,1,2,14";

    let input = read_input(input.to_string());

    assert_eq!(day7_2(input), 168);
}

#[test]
fn day7_2_actual() {
    let input = std::fs::read_to_string("./input/day7.txt").unwrap();

    let input = read_input(input.to_string());

    assert_eq!(day7_2(input), 98905973);
}
