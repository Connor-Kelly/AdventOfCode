//
// X_sum = n * Ax + m * Bx
// Y_sum = n * Ay + m * By
// (X_sum - (n * Ax)) / (Bx) = m
// Y_sum = n * Ay + (X_sum - (n * Ax)) / Bx * By
// Y_sum = n * Ay + (X_sum - (n * Ax)) * (By / Bx)
// Y_sum = n * Ay + (X_sum - (n * Ax)) * (By / Bx)
//
//
//
// (Y_sum - (n * Ay)) / By = m
// (X_sum - (n * Ax)) / (Bx) = (Y_sum - (n * Ay)) / By
// (X_sum - (n * Ax)) / (Y_sum - (n * Ay)) = Bx / By
//
//

// X_sum - (n * Ax) = Y_sum - (n * Ay)
// X_sum - Y_sum = n * (Ax + Ay)
// n = (X_sum - Y_sum) / (Ax + Ay)

use core::f64::math::{ceil, floor};

use regex::Regex;

type Number = f64;
type Coord = (Number, Number);
#[derive(Debug, Clone)]
struct Game {
    a: Coord,
    b: Coord,
    prize: Coord,
}

fn parse_input(input: String) -> Vec<Game> {
    // println!(
    //     "input: {input} {:?}",
    //     input.split("\n\n").collect::<Vec<&str>>()
    // );

    input
        .split("\n\n")
        .map(|lines| {
            // println!("lines: {lines}");
            let restring = r"Button A: X\+(\d+), Y\+(\d+)
Button B: X\+(\d+), Y\+(\d+)
Prize: X=(\d+), Y=(\d+)";

            let re = Regex::new(restring).unwrap();

            if let Some(caps) = re.captures(lines) {
                // println!("{caps:?}");
                let (_, [ax, ay, bx, by, tx, ty]) = caps.extract();
                let p = |w: &str| w.parse::<Number>().unwrap();
                Some(Game {
                    a: (p(ax), p(ay)),
                    b: (p(bx), p(by)),
                    prize: (p(tx), p(ty)),
                })
            } else {
                println!("no match!");
                panic!();
            }
        })
        .filter_map(|f| f)
        .collect::<Vec<Game>>()
    // println!("games: {games:?}");
    // vec![]
}

fn coalecse_to_int(val: Number) -> Option<i64> {
    const MAX_DIFF: Number = 0.005;

    if ceil(val) - val < MAX_DIFF {
        Some(ceil(val) as i64)
    } else if val - floor(val) < MAX_DIFF {
        Some(floor(val) as i64)
    } else {
        None
    }
}

fn calculate_game(g: &Game) -> Option<(i64, i64)> {
    let b_pressed = (g.prize.0 - (g.a.0 / g.a.1) * g.prize.1) / (g.b.0 - (g.a.0 / g.a.1) * g.b.1);
    // println!("b_pressed {b_pressed}");
    let a_pressed = (g.prize.0 - b_pressed * g.b.0) / g.a.0;
    // println!("a_pressed {a_pressed}");

    let _a = coalecse_to_int(a_pressed);
    // println!("a {a:?}");
    let _b = coalecse_to_int(b_pressed);
    // println!("b {b:?}");
    if let (Some(a), Some(b)) = (coalecse_to_int(a_pressed), coalecse_to_int(b_pressed)) {
        Some((a, b))
    } else {
        None
    }
    // (a, b)
}

fn part1(input: String) -> i64 {
    let games = parse_input(input);
    let results = games
        .iter()
        .filter_map(|game| {
            // println!("game: {game:?}");
            calculate_game(game)

            // Some(1)
        })
        .filter(|pair| pair.0 <= 100 && pair.1 <= 100)
        .map(|pair| pair.0 * 3 + pair.1)
        .sum::<i64>();
    // .collect::<Vec<_>>();
    println!("{results:?}");

    // println!("{games:?}");
    results
}

fn part2(input: String) -> i64 {
    let games = parse_input(input);
    let results = games
        .iter()
        .filter_map(|game| {
            let mut new_game = game.clone();
            new_game.prize.0 += 10000000000000.;
            new_game.prize.1 += 10000000000000.;
            // println!("game: {game:?}");
            calculate_game(&new_game)

            // Some(1)
        })
        // .filter(|pair| pair.0 <= 100 && pair.1 <= 100)
        .map(|pair| pair.0 * 3 + pair.1)
        .sum::<i64>();
    // .collect::<Vec<_>>();
    println!("{results:?}");

    // println!("{games:?}");
    results
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    #[test]
    fn part1_small() {
        let input = fs::read_to_string("../../inputs/D12/small.txt").unwrap();
        let result = part1(input);
        assert_eq!(result, 480)
    }

    #[test]
    fn part1_main() {
        let input = fs::read_to_string("../../inputs/D12/main.txt").unwrap();
        let result = part1(input);
        assert_eq!(result, 37680)
    }

    #[test]
    fn part2_main() {
        let input = fs::read_to_string("../../inputs/D12/main.txt").unwrap();
        let result = part2(input);
        assert_eq!(result, 87550094242995)
    }
}
