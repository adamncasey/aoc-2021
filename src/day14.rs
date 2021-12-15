use std::collections::HashMap;

struct Input {
    chain: Vec<char>,
    rules: HashMap<(char, char), char>,
}

fn day14(input: Input, steps: usize) -> usize {
    let first: char = input.chain[0];
    let mut pair_map = HashMap::new();
    for pair in input.chain.windows(2) {
        *pair_map.entry((pair[0], pair[1])).or_default() += 1;
    }

    for _step in 0..steps {
        pair_insert(&mut pair_map, &input.rules);
    }

    let mut chars: HashMap<char, usize> = HashMap::new();
    for (key, value) in pair_map.iter() {
        *chars.entry(key.1).or_default() += value;  
    }
    *chars.entry(first).or_default() += 1;

    let min = chars.values().min().unwrap();
    let max = chars.values().max().unwrap();
    
    max - min
}

fn pair_insert(pair_map: &mut HashMap<(char, char), usize>, rules: &HashMap<(char, char), char>) {

    let mut new_pair_map: HashMap<(char, char), usize> = HashMap::new();
    for (key, value) in pair_map.iter_mut() {
        match rules.get(key) {
            None => {*new_pair_map.entry(*key).or_default() += *value},
            Some(insert) => {
                *new_pair_map.entry((key.0, *insert)).or_default() += *value;
                *new_pair_map.entry((*insert, key.1)).or_default() += *value;
            }
        }
    }

    *pair_map = new_pair_map.clone();
}
/*
fn day14_2(octo: &mut Vec<Vec<u32>>, days: usize) -> usize {
    let mut day = 1;
    loop {
        let flashes = day14_step(octo);

        if flashes == 100 {
            return day;
        }

        day += 1;
    }
}*/

fn read_input<'a>(input: String) -> Input {
    let (chain_str, rules_lines) = input.split_once("\n\n").unwrap();

    let chain = chain_str.chars().collect::<Vec<char>>();

    let mut rules = HashMap::new();

    for rule in rules_lines.lines() {
        let (left, right) = rule.split_once(" -> ").unwrap();

        let mut chars = left.chars();

        let target = (chars.next().unwrap(), chars.next().unwrap());
        let insert = right.chars().next().unwrap();

        rules.insert(target, insert);
    }

    Input { chain, rules }
}

#[test]
fn day14_example() {
    let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    let input = read_input(input.to_string());
    //assert_eq!(day14(&mut input, 1), 0);
    //assert_eq!(day14(&mut input, 1), 20);

    assert_eq!(day14(input, 10), 1588);
}

#[test]
fn day14_actual() {
    let input = std::fs::read_to_string("./input/day14.txt").unwrap();

    let input = read_input(input.to_string());

    assert_eq!(day14(input, 10), 3342);
}

#[test]
fn day14_2_example() {
    let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    let input = read_input(input.to_string());
    //assert_eq!(day14(&mut input, 1), 0);
    //assert_eq!(day14(&mut input, 1), 20);

    assert_eq!(day14(input, 40), 2188189693529);
}

#[test]
fn day14_2_actual() {
    let input = std::fs::read_to_string("./input/day14.txt").unwrap();

    let input = read_input(input.to_string());

    assert_eq!(day14(input, 40), 3776553567525);
}