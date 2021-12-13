use std::collections::HashMap;

#[derive(Debug)]
struct DisplayInfo<'a> {
    patterns: Vec<&'a str>,
    digits: Vec<&'a str>,
}

fn digit_basic(digit: &str) -> usize {
    // determine f by intersection of 1 and 6

    // 1 has two segments
    // 2 has 5 segments and does not contains f
    // 3 has 5 segments and all of 1
    // 4 has 4 segments
    // 5 has 5 segments and does contain f
    // 6 has 6 segments but not all of the ones for 1
    // 7 has 3 segments
    // 8 has 7 segments
    // 9 has 6 segments incl all of 4 and 1
    // 0 has 6 segments incl all of 1 but excl all of 4

    match digit.len() {
        2 => 1,
        4 => 4,
        3 => 7,
        7 => 8,
        _ => 0,
    }
}

fn day8(input: Vec<DisplayInfo>) -> usize {
    let mut total = 0;
    for display in input {
        dbg!(&display);
        for d in display.digits {
            dbg!(&d);
            let num = digit_basic(d);
            total += match num {
                1 => 1,
                4 => 1,
                7 => 1,
                8 => 1,
                _ => 0,
            }
        }
    }

    total
}

fn digit(digit: &str, mapping: &HashMap<String, usize>) -> usize {
    let mut chars = digit.chars().collect::<Vec<char>>();
    chars.sort();
    let digit_sorted: String = chars.into_iter().collect();

    return *mapping.get(&digit_sorted).unwrap();
}

fn determine_mapping<'a>(patterns: &[&'a str]) -> HashMap<String, usize> {
    let mut length_map: HashMap<usize, Vec<String>> = HashMap::new();

    for pattern in patterns {
        let mut chars = pattern.chars().collect::<Vec<char>>();
        chars.sort();

        length_map
            .entry(pattern.len())
            .or_insert_with(Vec::new)
            .push(chars.into_iter().collect());
    }

    let mut result = HashMap::new();

    let one = length_map.get(&2).unwrap()[0].clone();
    let seven = length_map.get(&3).unwrap()[0].clone();
    let eight = length_map.get(&7).unwrap()[0].clone();
    let four = length_map.get(&4).unwrap()[0].clone();

    result.insert(one.clone(), 1);
    result.insert(seven.clone(), 7);
    result.insert(eight.clone(), 8);
    result.insert(four.clone(), 4);

    let mut six = "".to_owned();

    for six_chars in length_map.get(&6).unwrap() {
        if one.chars().all(|x| six_chars.contains(x)) {
            if four.chars().all(|x| six_chars.contains(x)) {
                result.insert(six_chars.clone(), 9);
            } else {
                result.insert(six_chars.clone(), 0);
            }
        } else {
            six = six_chars.clone();
            result.insert(six_chars.clone(), 6);
        }
    }
    let actual_f = six.chars().filter(|x| one.contains(*x)).next().unwrap();
    dbg!(actual_f);

    // determine f by intersection of 1 and 6

    // 2 has 5 segments and does not contains f
    // 3 has 5 segments and all of 1
    // 5 has 5 segments and does contain f

    for five_chars in length_map.get(&5).unwrap() {
        if one.chars().all(|x| five_chars.contains(x)) {
            result.insert(five_chars.clone(), 3);
        } else {
            if five_chars.contains(actual_f) {
                result.insert(five_chars.clone(), 5);
            } else {
                result.insert(five_chars.clone(), 2);
            }
        }
    }

    dbg!(&result);

    result
}

fn day8_2(input: Vec<DisplayInfo>) -> usize {
    let mut total = 0;

    for display in input {
        dbg!(&display);

        let mapping = determine_mapping(&display.patterns);

        let mut value = 0;
        for d in display.digits {
            let x = digit(d, &mapping);
            dbg!(&d, x);
            value = (value * 10) + x;
        }

        total += value;
    }

    total
}

fn read_input<'a>(input: &'a str) -> Vec<DisplayInfo<'a>> {
    let mut result: Vec<DisplayInfo> = Vec::new();
    for line in input.split("\n") {
        let (left, right) = line.split_once(" | ").unwrap();
        result.push(DisplayInfo {
            patterns: left.split(" ").collect(),
            digits: right.split(" ").collect(),
        });
    }

    result
}

#[test]
fn day8_example() {
    let input =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    let input = read_input(input);

    assert_eq!(day8(input), 26);
}

#[test]
fn day8_actual() {
    let input = std::fs::read_to_string("./input/day8.txt").unwrap();

    let input = read_input(&input);

    assert_eq!(day8(input), 495);
}

#[test]
fn day8_2_example() {
    let input =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    let input = read_input(input);

    assert_eq!(day8_2(input), 5353);
}

#[test]
fn day8_2_actual() {
    let input = std::fs::read_to_string("./input/day8.txt").unwrap();

    let input = read_input(&input);

    assert_eq!(day8_2(input), 1055164);
}
