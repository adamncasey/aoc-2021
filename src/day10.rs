fn syntax_cost(ch: char) -> u64 {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("unexpected char {}", ch),
    }
}

fn incomplete_cost(chs: &Vec<char>) -> u64 {
    let mut total = 0;
    for ch in chs.iter().rev() {
        let increment = match ch {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("Nope"),
        };
        total = total * 5 + increment;
    }

    dbg!((chs, total));

    total
}

fn opposite(ch: char) -> char {
    match ch {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("unexpected character {}", ch),
    }
}

enum Error {
    Corrupt(char),
    Incomplete(Vec<char>),
}

fn incorrect(input: &str) -> Error {
    let mut stack = Vec::new();

    for ch in input.chars() {
        match ch {
            ')' | ']' | '}' | '>' => match stack.pop() {
                None => return Error::Corrupt(ch),
                Some(x) => {
                    if ch != opposite(x) {
                        return Error::Corrupt(ch);
                    }
                }
            },
            _ => stack.push(ch),
        }
    }

    return Error::Incomplete(stack);
}

fn day10(lines: &Vec<String>) -> u64 {
    let mut total_error = 0;
    for line in lines {
        dbg!(line);
        let error = incorrect(line);
        if let Error::Corrupt(x) = error {
            dbg!((line, x, syntax_cost(x)));
            total_error += syntax_cost(x);
        }
    }

    total_error
}

fn day10_2(lines: &Vec<String>) -> u64 {
    let mut scores = Vec::new();
    for line in lines {
        dbg!(line);
        let error = incorrect(line);
        if let Error::Incomplete(x) = error {
            let cost = incomplete_cost(&x);
            dbg!((line, x, cost));
            scores.push(cost);
        }
    }

    scores.sort();
    dbg!(&scores);

    scores[scores.len() / 2]
}

fn read_input(input: String) -> Vec<String> {
    let input: Vec<String> = input.split("\n").map(|x| x.to_string()).collect();

    input
}

#[test]
fn day10_example() {
    let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    let input = read_input(input.to_string());

    assert_eq!(day10(&input), 26397);
}

#[test]
fn day10_actual() {
    let input = std::fs::read_to_string("./input/day10.txt").unwrap();

    let input = read_input(input.to_string());

    assert_eq!(day10(&input), 299793);
}

#[test]
fn day10_2_example() {
    let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    let input = read_input(input.to_string());

    assert_eq!(day10_2(&input), 288957);
}

#[test]
fn day10_2_actual() {
    let input = std::fs::read_to_string("./input/day10.txt").unwrap();

    let input = read_input(input.to_string());

    assert_eq!(day10_2(&input), 3654963618);
}
