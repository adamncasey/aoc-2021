
fn day6(game: &BingoGame) -> usize {
    0
}


fn read_input(input: String) -> &str {
    let input: Vec<&str> = input.split("\n\n").collect();

    let draws: Vec<u16> = input[0].split(",").map(|x| str::parse::<u16>(x).unwrap()).collect();

    let boards: Vec<BingoBoard> = input[1..].iter().map(|x| read_board(x)).collect();

    ""
}

#[test]
fn day6_example() {
    let input= "";

    let input = read_input(input.to_string());

    assert_eq!(day6(&input), 4512);
}

#[test]
fn day6_actual() {
    let input= std::fs::read_to_string("./input/day6.txt")
        .unwrap();

    let input = read_input(input.to_string());

    assert_eq!(day6(&input), 49686);
}