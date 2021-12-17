use std::cmp::max;

fn within(pos: (i32, i32), xbounds: (i32, i32), ybounds: (i32, i32)) -> bool {
    if pos.0 >= xbounds.0 && pos.0 <= xbounds.1 {
        if pos.1 >= ybounds.0 && pos.1 <= ybounds.1 {
            return true;
        }
    }

    false
}

fn max_height_intersect(start_v: (i32, i32), xbounds: (i32, i32), ybounds: (i32, i32)) -> Option<i32> {
    let mut max_height = 0;

    let mut pos = (0, 0);
    let mut velocity = start_v;

    let mut steps = 0;
    
    loop {
        pos = (pos.0 + velocity.0, pos.1 + velocity.1);
        max_height = max(max_height, pos.1);

        if within(pos, xbounds, ybounds) {
            return Some(max_height);
        }

        if pos.0 > xbounds.1 {
            return None;
        }
        if pos.1 < ybounds.0 && velocity.1 < 0 {
            return None;
        }

        velocity = (max(velocity.0 - 1, 0), velocity.1 - 1);

        steps += 1;

        if steps > 100000 {
            panic!("max iterations");
        }
    }
}

fn day17(xbounds: (i32, i32), ybounds: (i32, i32)) -> i32 {
    let mut max_height = 0;

    for xv in 1..=xbounds.1 {
        for yv in ybounds.0..=200 {
            if let Some(height) = max_height_intersect((xv, yv), xbounds, ybounds) {
                max_height = max(max_height, height);
            }
        }
    }

    max_height
}


fn day17_2(xbounds: (i32, i32), ybounds: (i32, i32)) -> i32 {
    let mut count = 0;

    for xv in 1..=xbounds.1 {
        for yv in ybounds.0..=200 {
            if let Some(_) = max_height_intersect((xv, yv), xbounds, ybounds) {
                dbg!((xv, yv));
                count += 1;
            }
        }
    }

    count
}

#[test]
fn day17_example() {
    // target area: x=20..30, y=-10..-5
    let (xbounds, ybounds) = ((20, 30), (-10, -5));

    assert_eq!(day17(xbounds, ybounds), 45);
}

#[test]
fn day17_actual() {
    // target area: x=57..116, y=-198..-148
    let (xbounds, ybounds) = ((57, 116), (-198, -148));

    assert_eq!(day17(xbounds, ybounds), 19503);
}

#[test]
fn day17_2_example() {
    // target area: x=20..30, y=-10..-5
    let (xbounds, ybounds) = ((20, 30), (-10, -5));

    assert_eq!(day17_2(xbounds, ybounds), 112);
}

#[test]
fn day17_2_actual() {
    // target area: x=57..116, y=-198..-148
    let (xbounds, ybounds) = ((57, 116), (-198, -148));

    assert_eq!(day17_2(xbounds, ybounds), 5200);
}

