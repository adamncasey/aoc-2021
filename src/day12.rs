use std::collections::HashMap;
use std::collections::HashSet;

fn day12(
    caves: &mut HashMap<String, Vec<String>>,
    seen: &mut HashSet<String>,
    current: &str,
    small_cave_revisited: Option<&str>,
    route: &mut Vec<String>,
) -> usize {
    if current == "end" {
        println!("{:?}", &route);
        //dbg!((current, small_cave_revisited));
        return 1;
    }

    //dbg!((current, small_cave_revisited, &seen));

    route.push(current.to_string());
    if current.chars().next().unwrap().is_lowercase() {
        seen.insert(current.to_string());
    }

    let mut total_routes = 0;
    let neighbours = caves.entry(current.to_string()).or_default().clone();
    for neighbour in neighbours {
        //dbg!((current, &neighbour));
        if neighbour == "start" {
            continue;
        }
        if seen.contains(&neighbour) {
            if let Some(_cave) = small_cave_revisited {
                continue;
            } else {
                total_routes += day12(caves, seen, &neighbour, Some(&neighbour), route);
            }
        } else {
            total_routes += day12(caves, seen, &neighbour, small_cave_revisited, route);
        }
    }

    if let Some(cave) = small_cave_revisited {
        if cave != current {
            seen.remove(current);
        } else {
            println!("Avoided removing {}", current);
        }
    } else {
        seen.remove(current);
    }
    route.pop();
    total_routes
}

fn read_input<'a>(input: String) -> HashMap<String, Vec<String>> {
    let mut result = HashMap::new();

    for line in input.lines() {
        let (left, right) = line.split_once('-').unwrap();

        result
            .entry(left.to_string())
            .or_insert_with(Vec::new)
            .push(right.to_string());
        result
            .entry(right.to_string())
            .or_insert_with(Vec::new)
            .push(left.to_string());
    }

    dbg!(&result);

    result
}

#[test]
fn day12_example() {
    let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    let mut input = read_input(input.to_string());

    assert_eq!(
        day12(
            &mut input,
            &mut HashSet::new(),
            "start",
            Some("Nope"),
            &mut Vec::new()
        ),
        10
    );
}

#[test]
fn day12_example2() {
    let input = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    let mut input = read_input(input.to_string());

    assert_eq!(
        day12(
            &mut input,
            &mut HashSet::new(),
            "start",
            Some("Nope"),
            &mut Vec::new()
        ),
        19
    );
}

#[test]
fn day12_example3() {
    let input = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    let mut input = read_input(input.to_string());

    assert_eq!(
        day12(
            &mut input,
            &mut HashSet::new(),
            "start",
            Some("Nope"),
            &mut Vec::new()
        ),
        226
    );
}

#[test]
fn day12_actual() {
    let input = std::fs::read_to_string("./input/day12.txt").unwrap();

    let mut input = read_input(input.to_string());

    assert_eq!(
        day12(
            &mut input,
            &mut HashSet::new(),
            "start",
            Some("Nope"),
            &mut Vec::new()
        ),
        3421
    );
}

#[test]
fn day12_2_example1() {
    let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    let mut input = read_input(input.to_string());

    assert_eq!(
        day12(
            &mut input,
            &mut HashSet::new(),
            "start",
            None,
            &mut Vec::new()
        ),
        36
    );
}

#[test]
fn day12_2_example2() {
    let input = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    let mut input = read_input(input.to_string());

    assert_eq!(
        day12(
            &mut input,
            &mut HashSet::new(),
            "start",
            None,
            &mut Vec::new()
        ),
        103
    );
}

#[test]
fn day12_2_example3() {
    let input = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    let mut input = read_input(input.to_string());

    assert_eq!(
        day12(
            &mut input,
            &mut HashSet::new(),
            "start",
            None,
            &mut Vec::new()
        ),
        3509
    );
}

#[test]
fn day12_2_actual() {
    let input = std::fs::read_to_string("./input/day12.txt").unwrap();

    let mut input = read_input(input.to_string());

    assert_eq!(
        day12(
            &mut input,
            &mut HashSet::new(),
            "start",
            None,
            &mut Vec::new()
        ),
        84870
    );
}
