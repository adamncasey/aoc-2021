use std::collections::HashSet;
use std::collections::HashMap;

struct Variation {
    div_z: i64, // 1 or 26
    add_x: i64,
    add_y: i64,
}

/// For a given `inp` and z carry over, calculate the next carry over
fn digit(w: i64, mut z: i64, params: &Variation) -> i64 {
    // inp w

    // mul x 0
    // add x z
    // mod x 26
    let mut x: i64 = z % 26;

    // div z 1 OR 26
    z = z / params.div_z; // 1 or 26

    // add x 10, OR something else // -2
    x += params.add_x;

    // eql x w
    // eql x 0
    x = if x != w { 1 } else { 0 };

    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    let y: i64 = (25 * x) + 1;

    // mul z y
    z = z * y;

    // mul y 0
    // add y w
    // add y 5, OR something else // 15
    // mul y x
    let y: i64 = (w + params.add_y) * x;

    // add z y
    z = z + y;

    z
}

fn calc_routes(digits: Vec<Variation>) -> Vec<HashMap<i64, Vec<(i64, i64)>>> {
    let mut routes: Vec<HashMap<i64, Vec<(i64, i64)>>> = Vec::new();

    let mut targets: HashSet<i64> = HashSet::new();
    targets.insert(0);

    for d in (0..=13).rev() {
        println!("Searching digit {}/{} for {:?}", d+1, digits.len(), targets);
        let mut new_targets = HashSet::new();

        let mut digit_routes: HashMap<i64, Vec<(i64, i64)>> = HashMap::new();

        for z in -100000..=100000 {
            for w in 1..=9 {
                let result = digit(w, z, &digits[d]);
                if targets.contains(&result) {
                    println!("Step {} digit{} in_z{} out_z{}", d, w, z, result);
                    new_targets.insert(z);
                    digit_routes.entry(z).or_insert_with(Vec::new).push((w, result));
                }
            }
        }
        targets = new_targets;

        routes.push(digit_routes);
    }

    routes
}

fn day24(digits: Vec<Variation>) -> Vec<i64> {

    // map z_input -> [(digit, z_output)]
    let routes = calc_routes(digits);

    let mut digits = Vec::new();

    let mut z_in = 0;
    for options in routes.iter().rev() {
        println!("{:?}", options);
        let mut best_step = (0,0);
        for step in options.get(&z_in).unwrap() {
            if step.0 > best_step.0 {
                best_step = *step;
            }
        }
        digits.push(best_step.0);

        println!("z_in {} -> {:?}", z_in, best_step);
        z_in = best_step.1;
    }

    digits
}


fn day24_2(digits: Vec<Variation>) -> Vec<i64> {
    // map z_input -> [(digit, z_output)]
    let routes = calc_routes(digits);

    let mut digits = Vec::new();

    let mut z_in = 0;
    for options in routes.iter().rev() {
        println!("{:?}", options);
        let mut best_step = (10,0);
        for step in options.get(&z_in).unwrap() {
            if step.0 < best_step.0 {
                best_step = *step;
            }
        }
        digits.push(best_step.0);

        println!("z_in {} -> {:?}", z_in, best_step);
        z_in = best_step.1;
    }

    digits
}

fn read_third(input: &str) -> i64 {
    dbg!(input);
    str::parse::<i64>(input.split(" ").skip(2).next().unwrap()).unwrap()
}

fn read_input(input: &str) -> Vec<Variation> {
    let mut digit_programs = Vec::new();
    for program in input.split("inp w\n").skip(1) {
        println!("{}", program);
        let lines = program.lines().collect::<Vec<&str>>();

        digit_programs.push(Variation {
            div_z: read_third(lines[3]),
            add_x: read_third(lines[4]),
            add_y: read_third(lines[14]),
        });
    }

    digit_programs
}

#[test]
fn day24_actual() {
    let input = std::fs::read_to_string("./input/day24.txt").unwrap();

    let digit_programs = read_input(&input);

    assert_eq!(day24(digit_programs), vec![9,9,9,1,9,6,9,2,4,9,6,9,3,9]);
}

#[test]
fn day24_2_actual() {
    let input = std::fs::read_to_string("./input/day24.txt").unwrap();

    let digit_programs = read_input(&input);

    assert_eq!(day24_2(digit_programs), vec![8,1,9,1,4,1,1,1,1,6,1,7,1,4]);
}