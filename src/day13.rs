use std::collections::HashSet;

fn day13(input: &Input) -> usize {
    dbg!(input);
    let mut points = input.points.clone();

    fold(&mut points, input.folds[0]);

    let mut set = HashSet::new();

    for p in points {
        set.insert(p);
    }

    set.len()
}

fn fold(input: &mut Vec<(usize, usize)>, fold: Fold) {
    for point in input {
        match fold {
            Fold::X(x) => {
                if point.0 > x {
                    let dist = point.0 - x;
                    point.0 -= dist * 2;
                }
            }
            Fold::Y(y) => {
                if point.1 > y {
                    let dist = point.1 - y;
                    point.1 -= dist * 2;
                }
            }
        }
    }
}

fn draw_points(points: &Vec<(usize, usize)>) -> Vec<String> {
    let xbound = *points.iter().map(|(x, _y)| x).max().unwrap() + 1;
    let ybound = *points.iter().map(|(_x, y)| y).max().unwrap() + 1;

    let mut grid = vec![' '; xbound * ybound];
    dbg!((xbound, ybound));

    for point in points {
        grid[point.1 * xbound + point.0] = '#';
    }

    grid.chunks(xbound)
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<String>>()
}

fn day13_2(input: &Input) -> Vec<String> {
    let mut points = input.points.clone();
    for f in &input.folds {
        fold(&mut points, *f);
    }

    let output = draw_points(&points);

    for line in &output {
        println!("{}", line);
    }

    output
}

#[derive(Debug)]
struct Input {
    points: Vec<(usize, usize)>,
    folds: Vec<Fold>,
}

#[derive(Debug, Clone, Copy)]
enum Fold {
    X(usize),
    Y(usize),
}

fn read_input<'a>(input: String) -> Input {
    let (point_defs, fold_lines) = input.split_once("\n\n").unwrap();

    let mut points = Vec::new();
    for point in point_defs.lines() {
        let (x, y) = point.split_once(",").unwrap();
        let (x, y) = (
            str::parse::<usize>(x).unwrap(),
            str::parse::<usize>(y).unwrap(),
        );

        points.push((x, y));
    }

    let mut folds = Vec::new();
    for fold in fold_lines.lines() {
        let (left, right) = fold.split_once("=").unwrap();

        if left == "fold along x" {
            folds.push(Fold::X(str::parse::<usize>(right).unwrap()));
        } else if left == "fold along y" {
            folds.push(Fold::Y(str::parse::<usize>(right).unwrap()));
        } else {
            panic!("Nope");
        }
    }

    Input { points, folds }
}

#[test]
fn day13_example() {
    let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    let mut input = read_input(input.to_string());

    assert_eq!(day13(&mut input), 17);
}

#[test]
fn day13_actual() {
    let input = std::fs::read_to_string("./input/day13.txt").unwrap();

    let mut input = read_input(input.to_string());

    assert_eq!(day13(&mut input), 695);
}

#[test]
fn day13_2_example() {
    let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    let mut input = read_input(input.to_string());

    assert_eq!(
        day13_2(&mut input),
        vec!["#####", "#   #", "#   #", "#   #", "#####"]
    );
}

#[test]
fn day13_2_actual() {
    let input = std::fs::read_to_string("./input/day13.txt").unwrap();

    let mut input = read_input(input.to_string());

    assert_eq!(
        day13_2(&mut input),
        vec![
            " ##    ## ####  ##  #    #  # ###    ##",
            "#  #    #    # #  # #    #  # #  #    #",
            "#       #   #  #    #    #  # #  #    #",
            "# ##    #  #   # ## #    #  # ###     #",
            "#  # #  # #    #  # #    #  # #    #  #",
            " ###  ##  ####  ### ####  ##  #     ## "
        ]
    );
}
