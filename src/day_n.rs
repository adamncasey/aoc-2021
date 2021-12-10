
fn dayn(game: &BingoGame) -> usize {
    0
}


fn read_input(input: String) -> &str {
    let input: Vec<&str> = input.split("\n\n").collect();

    let draws: Vec<u16> = input[0].split(",").map(|x| str::parse::<u16>(x).unwrap()).collect();

    let boards: Vec<BingoBoard> = input[1..].iter().map(|x| read_board(x)).collect();

    ""
}

#[test]
fn day4_example() {
    let input= "";

    let game = read_game(input.to_string());

    assert_eq!(day4(&game), 4512);
}

#[test]
fn day4_actual() {
    let input= std::fs::read_to_string("./input/dayn.txt")
        .unwrap();

    let game = read_game(input.to_string());

    assert_eq!(day4(&game), 49686);
}