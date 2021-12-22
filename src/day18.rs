use std::pin::Pin;

#[derive(Debug, PartialEq, Eq)]
enum SnailNum {
    Number(Pin<Box<Number>>),
    Pair(Box<SnailNum>, Box<SnailNum>),
}

#[derive(Eq, Debug, PartialOrd, Ord)]
struct Number {
    val: u32,
    left: *mut Number,
    right: *mut Number,
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl std::fmt::Display for SnailNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnailNum::Pair(left, right) => write!(f, "[{},{}]", *left, *right),
            SnailNum::Number(num) => write!(f, "{}", *num),
        }
    }
}

fn explode(num: SnailNum, depth: usize) -> (bool, SnailNum) {
    //dbg!((depth, &num));
    if let SnailNum::Pair(mut left, mut right) = num {
        if depth >= 4 {
            if let SnailNum::Number(left_num) = &mut *left {
                if let SnailNum::Number(right_num) = &mut *right {
                    // explode `num`
                    println!("Explode [{}, {}]", left_num, right_num);

                    let mut new_num = Box::pin(Number {
                        val: 0,
                        left: (*left_num).left,
                        right: (*right_num).right,
                    });
                    // if left.left
                    if !(*left_num).left.is_null() {
                        // left.left += left
                        unsafe {
                            println!("Left explode {} to {}", left_num, (*(*left_num).left));
                            (*(*left_num).left).val += (*left_num).val;

                            (*(*new_num).left).right = &mut *new_num;
                        }
                    }

                    if !(*right_num).right.is_null() {
                        unsafe {
                            println!("Right explode {} to {}", &right_num, (*(*right_num).right));
                            (*(*right_num).right).val += (*right_num).val;

                            (*(*new_num).right).left = &mut *new_num;
                        }
                    }

                    return (true, SnailNum::Number(new_num));
                }
            }
        }

        let (exploded, new_left) = explode(*left, depth + 1);
        if exploded {
            return (true, SnailNum::Pair(Box::new(new_left), right));
        }

        let (exploded, new_right) = explode(*right, depth + 1);

        return (
            exploded,
            SnailNum::Pair(Box::new(new_left), Box::new(new_right)),
        );
    }

    return (false, num);
}

fn split(num: &mut SnailNum) -> bool {
    match num {
        SnailNum::Pair(left, right) => {
            return split(left) || split(right);
        }
        SnailNum::Number(x) => {
            if (**x).val >= 10 {
                let mut new_left = Box::pin(Number {
                    val: (**x).val / 2,
                    left: x.left,
                    right: 0 as *mut Number,
                });

                let mut new_right = Box::pin(Number {
                    val: ((**x).val + 1) / 2,
                    left: &mut *new_left,
                    right: x.right,
                });
                new_left.right = &mut *new_right;

                unsafe {
                    if !x.left.is_null() {
                        (*x.left).right = &mut *new_left;
                    }
                    if !x.right.is_null() {
                        (*x.right).left = &mut *new_right;
                    }
                }

                *num = SnailNum::Pair(
                    Box::new(SnailNum::Number(new_left)),
                    Box::new(SnailNum::Number(new_right)),
                );

                return true;
            }
        }
    }

    false
}

fn reduce(mut num: SnailNum) -> SnailNum {
    println!("Starting reduce with {}", &num);

    loop {
        let (exploded, new_num) = explode(num, 0);

        num = new_num;
        if exploded {
            println!("Exploded to {}", &num);
            continue;
        }

        if split(&mut num) {
            println!("Split to {}", &num);
            continue;
        } else {
            break;
        }
    }

    num
}

fn find_left_num<'a>(num: &'a mut SnailNum) -> Option<&'a mut Number> {
    match num {
        SnailNum::Number(n) => Some(&mut *n),
        SnailNum::Pair(left, right) => {
            if let Some(n_ref) = find_left_num(left) {
                return Some(n_ref);
            }

            if let Some(n_ref) = find_left_num(right) {
                return Some(n_ref);
            }

            None
        }
    }
}

fn find_right_num<'a>(num: &'a mut SnailNum) -> Option<&'a mut Number> {
    match num {
        SnailNum::Number(n) => Some(&mut *n),
        SnailNum::Pair(left, right) => {
            if let Some(n_ref) = find_right_num(right) {
                return Some(n_ref);
            }

            if let Some(n_ref) = find_right_num(left) {
                return Some(n_ref);
            }
            None
        }
    }
}

fn sum(mut nums: Vec<SnailNum>) -> SnailNum {
    nums.reverse();

    let mut combined_num = nums.pop().unwrap();

    nums.reverse();

    for mut num in nums {
        println!("Adding {:?}", &num);
        let mut left_to_right_ref = find_right_num(&mut combined_num).unwrap();
        let mut right_to_left_ref = find_left_num(&mut num).unwrap();

        left_to_right_ref.right = right_to_left_ref;
        right_to_left_ref.left = left_to_right_ref;
        // need to update num right <-> combined_num left
        combined_num = SnailNum::Pair(Box::new(combined_num), Box::new(num));
        println!("{:?}", &combined_num);
        combined_num = reduce(combined_num);
    }

    combined_num
}

fn magnitude(num: &SnailNum) -> u32 {
    match num {
        SnailNum::Number(num) => num.val,
        SnailNum::Pair(left, right) => 3 * magnitude(&*left) + 2 * magnitude(&*right),
    }
}

fn day18(num: Vec<SnailNum>) -> u32 {
    let summed = sum(num);

    magnitude(&summed)
}

fn day18_2(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut max = 0;
    for n in 0..lines.len() {
        for n2 in 0..lines.len() {
            if n == n2 {
                continue;
            }

            let summed = sum(vec![read_snailnum(lines[n]), read_snailnum(lines[n2])]);
            max = std::cmp::max(max, magnitude(&summed));
        }
    }

    max
}

fn split_snailnum(input: &str) -> (&str, &str) {
    let mut level = 0;
    let mut first: usize = 0;
    for (idx, ch) in input.chars().enumerate() {
        match ch {
            '[' => level += 1,
            ']' => {
                level -= 1;
                if level == 0 {
                    return (&input[1..first], &input[first + 1..idx]);
                }
            }
            ',' => {
                if level == 1 {
                    first = idx;
                }
            }
            _ => (),
        }
    }

    panic!("Could not split: {}", input);
}

fn read_snailnum(input: &str) -> SnailNum {
    read_snailnum_ptr(input, &mut (0 as *mut Number))
}

fn read_snailnum_ptr(input: &str, left_ptr: &mut *mut Number) -> SnailNum {
    if &input[0..1] == "[" {
        let (left, right) = split_snailnum(input);

        SnailNum::Pair(
            Box::new(read_snailnum_ptr(left, left_ptr)),
            Box::new(read_snailnum_ptr(right, left_ptr)),
        )
    } else {
        let mut num = Box::pin(Number {
            val: input.parse::<u32>().unwrap(),
            left: *left_ptr,
            right: 0 as *mut Number,
        });

        unsafe {
            if !(*left_ptr).is_null() {
                (**left_ptr).right = &mut *num;
            }
        }
        *left_ptr = &mut *num;

        SnailNum::Number(num)
    }
}

fn read_input<'a>(input: &str) -> Vec<SnailNum> {
    let input: Vec<SnailNum> = input.split("\n").map(|x| read_snailnum(x)).collect();
    input
}

#[test]
fn day18_sum1() {
    let input = read_input(
        "[1,1]
[2,2]
[3,3]
[4,4]",
    );

    assert_eq!(sum(input), read_snailnum("[[[[1,1],[2,2]],[3,3]],[4,4]]"));
}

#[test]
fn day18_sum2() {
    let input = read_input(
        "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]",
    );

    assert_eq!(sum(input), read_snailnum("[[[[3,0],[5,3]],[4,4]],[5,5]]"));
}

#[test]
fn day18_sum4() {
    let input = read_input(
        "[[[[4,3],4],4],[7,[[8,4],9]]]
[1,1]",
    );

    assert_eq!(
        sum(input),
        read_snailnum("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
    );
}

#[test]
fn day18_sum3() {
    let input = read_input(
        "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]",
    );

    assert_eq!(
        sum(input),
        read_snailnum("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
    );
}

#[test]
fn day18_sum3_1() {
    let input = read_input(
        "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
    );

    let left = sum(input);
    let right = read_snailnum("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");

    println!("Left:  {}", &left);
    println!("Right: {}", &right);

    assert_eq!(left, right);
}

#[test]
fn day18_test_split() {
    assert_eq!(split_snailnum("[5,4]"), ("5", "4"));
}

#[test]
fn day18_explode1() {
    let input = read_snailnum("[[[[[9,8],1],2],3],4]");

    let input = reduce(input);

    assert_eq!(input, read_snailnum("[[[[0,9],2],3],4]"));
}

#[test]
fn day18_explode2() {
    let input = read_snailnum("[[[[[1,1],[2,2]],[3,3]],[4,4]],[5,5]]");

    let input = reduce(input);

    assert_eq!(input, read_snailnum("[[[[3,0],[5,3]],[4,4]],[5,5]]"));
}

#[test]
fn day18_split1() {
    let input = read_snailnum("[10,5]");
    let input = reduce(input);
    assert_eq!(input, read_snailnum("[[5,5],5]"));
}

#[test]
fn day18_split2() {
    let input = read_snailnum("[11,5]");
    let input = reduce(input);
    assert_eq!(input, read_snailnum("[[5,6],5]"));
}

#[test]
fn day18_example2() {
    let input = read_input("[[1,2],[[3,4],5]]");
    assert_eq!(day18(input), 143);

    let input = read_input("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    assert_eq!(day18(input), 1384);

    let input = read_input("[[[[1,1],[2,2]],[3,3]],[4,4]]");
    assert_eq!(day18(input), 445);

    let input = read_input("[[[[3,0],[5,3]],[4,4]],[5,5]]");
    assert_eq!(day18(input), 791);

    let input = read_input("[[[[5,0],[7,4]],[5,5]],[6,6]]");
    assert_eq!(day18(input), 1137);

    let input = read_input("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
    assert_eq!(day18(input), 3488);
}

#[test]
fn day18_example1() {
    let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    let input = read_input(input);

    assert_eq!(day18(input), 4140);
}
#[test]
fn day18_actual() {
    let input = std::fs::read_to_string("./input/day18.txt").unwrap();

    let input = read_input(&input);

    assert_eq!(day18(input), 1615);
}

#[test]
fn day18_2_example1() {
    let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    assert_eq!(day18_2(&input), 3993);
}

#[test]
fn day18_2_actual() {
    let input = std::fs::read_to_string("./input/day18.txt").unwrap();

    assert_eq!(day18_2(&input), 4690);
}
