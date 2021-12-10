fn day3(input: &[u16], mask: usize) -> usize {
    let mut value: usize = 0;

    for b in (0..12).rev() {
        let new_bit = match most_common_bit(input, b) {
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Less => 0,
        };

        value = (value << 1) + new_bit;
    }

    println!("{:b}", value);

    value * ((!value) & mask)
}

fn most_common_bit(input: &[u16], n: usize) -> std::cmp::Ordering {
    let mut one_count = 0;

    let mask = 1 << n;

    for x in input {
        if (x & mask) != 0 {
            one_count += 1;
        }
    }

    let threshold = input.len();
    dbg!(one_count);

    (one_count * 2).cmp(&threshold)
}

fn select_num(input: &[u16], n: usize, negate: bool) -> usize {
    let mut filtered_input: Vec<u16> = input.to_vec();

    for b in (0..n).rev() {
        dbg!(&filtered_input);
        let common = most_common_bit(&filtered_input, b);
        let select = match common {
            std::cmp::Ordering::Greater => {
                if negate {
                    0
                } else {
                    1
                }
            }
            std::cmp::Ordering::Equal => {
                if negate {
                    0
                } else {
                    1
                }
            }
            std::cmp::Ordering::Less => {
                if negate {
                    1
                } else {
                    0
                }
            }
        };
        dbg!(common, negate, select);

        let mask: u16 = 1 << b;
        filtered_input = filtered_input
            .iter()
            .filter(|x| ((*x & mask) >> b) == select)
            .map(|x| *x)
            .collect();

        if filtered_input.len() == 1 {
            return filtered_input[0] as usize;
        }
    }

    panic!("No value found");
}

fn day3_2(input: &[u16], n: usize) -> usize {
    let oxygen = select_num(input, n, false);
    let co2 = select_num(input, n, true);

    dbg!(oxygen, co2);

    oxygen * co2
}

#[test]
fn day3_example() {
    let input = [
        0b000000000100,
        0b000000011110,
        0b000000010110,
        0b000000010111,
        0b000000010101,
        0b000000001111,
        0b000000000111,
        0b000000011100,
        0b000000010000,
        0b000000011001,
        0b000000000010,
        0b000000001010,
    ];

    assert_eq!(day3(&input, 0b11111), 198);
}

#[test]
fn day3_actual() {
    let input: Vec<u16> = std::fs::read_to_string("./input/day3.txt")
        .unwrap()
        .lines()
        .map(|str| u16::from_str_radix(str, 2).unwrap())
        .collect();

    assert_eq!(day3(&input, 0b111111111111), 3912944);
}

#[test]
fn day3_2_example() {
    let input = [
        0b000000000100,
        0b000000011110,
        0b000000010110,
        0b000000010111,
        0b000000010101,
        0b000000001111,
        0b000000000111,
        0b000000011100,
        0b000000010000,
        0b000000011001,
        0b000000000010,
        0b000000001010,
    ];

    assert_eq!(day3_2(&input, 5), 230);
}

#[test]
fn day3_2_actual() {
    let input: Vec<u16> = std::fs::read_to_string("./input/day3.txt")
        .unwrap()
        .lines()
        .map(|str| u16::from_str_radix(str, 2).unwrap())
        .collect();

    assert_eq!(day3_2(&input, 12), 4996233);
}
