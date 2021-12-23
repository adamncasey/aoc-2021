struct Instruction {
    set_on: bool, // Otherwise set off
    x_range: (i32, i32),
    y_range: (i32, i32),
    z_range: (i32, i32),
}

fn day22(instrs: Vec<Instruction>) -> usize {
    let mut reactor = &mut [false; 101 * 101 * 101];
    println!("{}", reactor.len());

    for instr in instrs {
        for x in instr.x_range.0..=instr.x_range.1 {
            if x < -50 || x > 50 {
                continue;
            }
            for y in instr.y_range.0..=instr.y_range.1 {
                if y < -50 || y > 50 {
                    continue;
                }
                for z in instr.z_range.0..=instr.z_range.1 {
                    if z < -50 || z > 50 {
                        continue;
                    }
                    let (x, y, z) = ((x + 50) as usize, (y + 50) as usize, (z + 50) as usize);
                    println!("{} {} {}", x, y, z);
                    reactor[x * 101 * y * 101 + z] = instr.set_on;
                }
            }
        }
    }

    reactor.iter().filter(|x| **x).count()
}

fn read_input(input: &str) -> Vec<Instruction> {

    let mut instructions = Vec::new();

    for instr in input.lines() {
        let (on_off, rest) = instr.split_once(" x=").unwrap();

        let (xs, rest) = rest.split_once(",y=").unwrap();

        let (ys, zs) = rest.split_once(",z=").unwrap();

        let xbounds = xs.split("..").map(|n| str::parse::<i32>(n).unwrap()).collect::<Vec<i32>>();
        let ybounds = ys.split("..").map(|n| str::parse::<i32>(n).unwrap()).collect::<Vec<i32>>();
        let zbounds = zs.split("..").map(|n| str::parse::<i32>(n).unwrap()).collect::<Vec<i32>>();

        instructions.push(Instruction {
            set_on: on_off == "on",
            x_range: (xbounds[0], xbounds[1]),
            y_range: (ybounds[0], ybounds[1]),
            z_range: (zbounds[0], zbounds[1]),
        });
    }

    instructions
}

#[test]
fn day22_example() {
    let input = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";

    let input = read_input(input);

    assert_eq!(day22(input), 39);
}

#[test]
fn day22_actual() {
    let input = std::fs::read_to_string("./input/day22.txt").unwrap();

    let input = read_input(&input);

    assert_eq!(day22(input), 1615);
}
