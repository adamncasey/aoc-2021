enum Input {
    Forward(usize),
    Down(usize),
    Up(usize),
}

fn day2(input: &[Input]) -> usize {
    let mut depth = 0;
    let mut forward = 0;
    for step in input {
        match step {
            Input::Forward(x) => forward += x,
            Input::Down(x) => depth += x,
            Input::Up(x) => depth -= x,
        }
    }

    depth * forward
}

fn day2_2(input: &[Input]) -> usize {
    let mut aim = 0;
    let mut depth = 0;
    let mut forward = 0;
    for step in input {
        match step {
            Input::Forward(x) => {
                forward += x;
                depth += aim * x;
            }
            Input::Down(x) => aim += x,
            Input::Up(x) => aim -= x,
        }
    }

    depth * forward
}

fn read_input(cmds: &str) -> Vec<Input> {
    let mut input = Vec::new();
    for line in cmds.lines() {
        if let Some((x, y)) = line.split_once(' ') {
            input.push(match x {
                "forward" => Input::Forward(str::parse::<usize>(y).unwrap()),
                "down" => Input::Down(str::parse::<usize>(y).unwrap()),
                "up" => Input::Up(str::parse::<usize>(y).unwrap()),
                _ => panic!("Bad input {} {}", x, y),
            });
        }
    }

    input
}

#[test]
fn day2_example() {
    let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    let input = read_input(input);

    assert_eq!(day2(&input), 150);
}

#[test]
fn day2_actual() {
    let input = std::fs::read_to_string("./input/day2.txt").unwrap();

    let input = read_input(&input);

    assert_eq!(day2(&input), 1383564);
}

#[test]
fn day2_2_example() {
    let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    let input = read_input(input);

    assert_eq!(day2_2(&input), 900);
}

#[test]
fn day2_2_actual() {
    let input = std::fs::read_to_string("./input/day2.txt").unwrap();

    let input = read_input(&input);

    assert_eq!(day2_2(&input), 1488311643);
}
