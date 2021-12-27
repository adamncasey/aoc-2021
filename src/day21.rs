use std::collections::HashMap;

struct Dice {
    next_result: usize,
    total_throws: usize,
}

impl Dice {
    fn throw(&mut self) -> usize {
        let result = self.next_result;

        self.next_result += 1;
        if self.next_result > 100 {
            self.next_result = 1;
        }

        self.total_throws += 1;

        result
    }
}

fn day21(p1_start: usize, p2_start: usize) -> usize {
    let mut dice = Dice {
        next_result: 1,
        total_throws: 0,
    };

    let mut state = GameState::new(p1_start, p2_start);

    loop {
        let p1_throw = dice.throw() + dice.throw() + dice.throw();

        let new_state = state.play_p1(p1_throw);

        if new_state.p1_score >= 1000 {
            return new_state.p2_score * dice.total_throws;
        }

        let p2_throw = dice.throw() + dice.throw() + dice.throw();
        let new_state = new_state.play_p2(p2_throw);

        if new_state.p2_score >= 1000 {
            return new_state.p1_score * dice.total_throws;
        }

        state = new_state;
    }
}

fn calc_dice_possibilities() -> HashMap<usize, usize> {
    let mut results = HashMap::new();

    for p1_d1 in 1..=3 {
        for p1_d2 in 1..=3 {
            for p1_d3 in 1..=3 {
                *results.entry(p1_d1 + p1_d2 + p1_d3).or_default() += 1;
            }
        }
    }

    results
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct GameState {
    p1_pos: usize,
    p1_score: usize,
    p2_pos: usize,
    p2_score: usize,
}

impl GameState {
    fn new(p1_pos: usize, p2_pos: usize) -> GameState {
        GameState {
            p1_pos,
            p1_score: 0,
            p2_pos,
            p2_score: 0,
        }
    }

    fn play_p1(&self, p1_roll: usize) -> GameState {
        let p1_pos = ((self.p1_pos + p1_roll - 1) % 10) + 1;

        GameState {
            p1_pos,
            p1_score: self.p1_score + p1_pos,
            p2_pos: self.p2_pos,
            p2_score: self.p2_score,
        }
    }

    fn play_p2(&self, p2_roll: usize) -> GameState {
        let p2_pos = ((self.p2_pos + p2_roll - 1) % 10) + 1;

        GameState {
            p1_pos: self.p1_pos,
            p1_score: self.p1_score,
            p2_pos: p2_pos,
            p2_score: self.p2_score + p2_pos,
        }
    }
}

fn day21_2(p1_start: usize, p2_start: usize) -> usize {
    let possibilities = calc_dice_possibilities();

    println!("{:?}", possibilities);

    let mut current_scores = HashMap::new();
    current_scores.insert(GameState::new(p1_start, p2_start), 1);

    let mut p1_won = 0;
    let mut p2_won = 0;

    for _step in 0..100 {
        let mut next_scores = HashMap::new();
        for (state, count) in current_scores {
            for (p1_throw, p1_count) in &possibilities {
                let p1_state = state.play_p1(*p1_throw);

                if p1_state.p1_score >= 21 {
                    p1_won += count * p1_count;
                } else {
                    for (p2_throw, p2_count) in &possibilities {
                        let p2_state = p1_state.play_p2(*p2_throw);

                        if p2_state.p2_score >= 21 {
                            p2_won += count * p1_count * p2_count;
                        } else {
                            *next_scores.entry(p2_state).or_default() +=
                                count * p1_count * p2_count;
                        }
                    }
                }
            }
        }

        if next_scores.len() == 0 {
            println!("Finished {} {} ", p1_won, p2_won);
            break;
        }

        current_scores = next_scores;
    }

    std::cmp::max(p1_won, p2_won)
}

#[test]
fn day21_example() {
    assert_eq!(day21(4, 8), 739785);
}

#[test]
fn day21_actual() {
    assert_eq!(day21(10, 2), 916083);
}

#[test]
fn day21_2_example() {
    assert_eq!(day21_2(4, 8), 444356092776315);
}

#[test]
fn day21_2_actual() {
    assert_eq!(day21_2(10, 2), 49982165861983);
}

#[test]
fn day21_calc_possibilities() {
    let possibilities = calc_dice_possibilities();

    assert_eq!(possibilities.values().sum::<usize>(), (3 * 3 * 3));
}
