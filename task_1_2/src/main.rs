use rand::Rng;

const TIMESTAMPS_COUNT: usize = 50000;

const PROBABILITY_SCORE_CHANGED: f64 = 0.0001;

const PROBABILITY_HOME_SCORE: f64 = 0.45;

const OFFSET_MAX_STEP: i32 = 3;

const INITIAL_STAMP: Stamp = Stamp {
    offset: 0,
    score: Score { home: 0, away: 0 },
};

#[derive(Debug, Clone, Copy)]
struct Score {
    home: i32,
    away: i32,
}

#[derive(Debug, Clone, Copy)]
struct Stamp {
    offset: i32,
    score: Score,
}

fn generate_stamp(previous_value: Stamp) -> Stamp {
    let score_changed: bool = rand::thread_rng().gen_bool(PROBABILITY_SCORE_CHANGED);
    let home_score_change: bool = rand::thread_rng().gen_bool(PROBABILITY_HOME_SCORE);
    let offset_change: i32 = rand::thread_rng().gen_range(1..=OFFSET_MAX_STEP);

    Stamp {
        offset: previous_value.offset + offset_change,
        score: Score {
            home: previous_value.score.home + if score_changed && home_score_change { 1 } else { 0 },
            away: previous_value.score.away + if score_changed && !home_score_change { 1 } else { 0 },
        },
    }
}

fn generate_game() -> Vec<Stamp> {
    let mut stamps = vec![INITIAL_STAMP];
    let mut current_stamp = INITIAL_STAMP;

    for _ in 0..TIMESTAMPS_COUNT {
        current_stamp = generate_stamp(current_stamp);
        stamps.push(current_stamp);
    }

    stamps
}


fn get_score(game_stamps: &[Stamp], offset: i32) -> (i32, i32) {
    // Бинарнный поиск. Алгоритмическая сложность O(log(N))
    let mut right = game_stamps.len() - 1;
    let mut left = 0;
    let right_stamp = game_stamps[right];
    if offset >= right_stamp.offset {
        return (right_stamp.score.home - right_stamp.score.away, offset)
    }
    else if offset <= INITIAL_STAMP.offset {
        return (0, offset);
    }
    while left != right - 1 {
        let middle = (left + right) / 2;
        if game_stamps[middle].offset > offset {
            right = middle;
        }
        else if game_stamps[middle].offset < offset {
            left = middle;
        }
        else if game_stamps[middle].offset == offset {
            let stamp = game_stamps[middle];
            return (stamp.score.home - stamp.score.away, offset);
        }
    }
    let left_stamp = game_stamps[left];
    (left_stamp.score.home - left_stamp.score.away, offset)
}


fn main() {
    let stamps = generate_game();
    println!("{:?}", get_score(&stamps, 9878));
}

#[cfg(test)]
mod tests {
    use crate::get_score;

    use super::{Stamp, Score};
    fn stamps() -> Vec<Stamp> {
        vec![
            Stamp {
                offset: 0,
                score: Score { home: 0, away: 0 },
            },
            Stamp {
                offset: 3,
                score: Score { home: 1, away: 1 },
            },
            Stamp {
                offset: 5,
                score: Score { home: 3, away: 1 },
            },
            Stamp {
                offset: 9,
                score: Score { home: 6, away: 0 },
            },
            Stamp {
                offset: 11,
                score: Score { home: 10, away: 2 },
            },

        ]

    }

    #[test]
    fn offset_exactly_in_stamps() {
        let game_stamps = stamps();
        let offset = 5;
        let res = get_score(&game_stamps[..], offset);
        assert_eq!(res, (2, offset));
    }

    #[test]
    fn offset_between_existing_stamps() {
        let game_stamps = stamps();
        let offset = 7;
        let res = get_score(&game_stamps[..], offset);
        assert_eq!(res, (2, offset));


    }

    #[test]
    fn offset_is_out_of_the_right_stamp_range() {
        let game_stamps = stamps();
        let offset = 12;
        let res = get_score(&game_stamps[..], offset);
        assert_eq!(res, (8, offset));
    }


    #[test]
    fn offset_is_out_of_the_left_stamp_range() {
        let game_stamps = stamps();
        let offset = -5;
        let res = get_score(&game_stamps[..], offset);
        assert_eq!(res, (0, offset));
    }
}


