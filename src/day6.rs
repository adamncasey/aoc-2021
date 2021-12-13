fn day6(fish: Vec<u8>, days: usize) -> usize {
    let fish_states: &mut [usize; 9] = &mut [0; 9];

    for f in &fish {
        fish_states[*f as usize] += 1;
    }

    dbg!(&fish, days);

    for _day in 0..days {
        dbg!(&fish_states);
        let old_states = fish_states.clone();
        fish_states[0] = old_states[1];
        fish_states[1] = old_states[2];
        fish_states[2] = old_states[3];
        fish_states[3] = old_states[4];
        fish_states[4] = old_states[5];
        fish_states[5] = old_states[6];
        fish_states[6] = old_states[7] + old_states[0];
        fish_states[7] = old_states[8];
        fish_states[8] = old_states[0];
    }

    fish_states.iter().sum()
}

fn read_input(input: String) -> Vec<u8> {
    let input = input
        .split(",")
        .map(|x| str::parse::<u8>(x).unwrap())
        .collect();

    input
}

#[test]
fn day6_example() {
    let input = "3,4,3,1,2";

    let input = read_input(input.to_string());

    assert_eq!(day6(input, 80), 5934);
}

#[test]
fn day6_actual() {
    let input = std::fs::read_to_string("./input/day6.txt").unwrap();

    let input = read_input(input.to_string());

    assert_eq!(day6(input, 80), 359999);
}

#[test]
fn day6_2_actual() {
    let input = std::fs::read_to_string("./input/day6.txt").unwrap();

    let input = read_input(input.to_string());

    assert_eq!(day6(input, 256), 1631647919273);
}
