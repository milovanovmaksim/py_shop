# Задание 1. Разработать функцию определения счета в игре

## Задача
В примере кода ниже генерируется список фиксаций состояния счета игры в течение матча.
Разработайте функцию getScore(gameStamps, offset), которая вернет счет на момент offset в списке gameStamps.
Нужно суметь понять суть написанного кода, заметить нюансы, разработать функцию вписывающуюся стилем в существующий код, желательно адекватной алгоритмической сложности.


```rust
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
  // Implement this function
}
```

# Задание 2. Разработать тесты для функции определения счета в игре

## Задача
Для разработанной в предыдущем задании функции getScore(game_stamps, offset) разработайте unit-тесты.
Тесты должны учитывать все возможные случаи использования функции, концентрироваться на проверке одного случая, не повторяться, название тестов должно отражать суть выполняемой проверки.


## Как запустить:

1. Установить Rust https://www.rust-lang.org/tools/install.

2. Клонировать репозиторий https://github.com/milovanovmaksim/py_shop.

3. Зайти в директорию проекта.
```
cd py_shop/task_1_2
```
4. Компилирование проекта.
```
cargo build
```
5. Запуск тестов.
```
cargo test
```

6. Запуск.
```
cargo run
```