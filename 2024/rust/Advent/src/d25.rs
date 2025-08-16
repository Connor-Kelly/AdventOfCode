use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

type Schematic = [i32; 5];

#[derive(Debug)]
enum LockOrKey<T> {
    Lock(T),
    Key(T),
}

fn parse_schematic(input: &str) -> LockOrKey<Schematic> {
    let lines = input.split("\n").collect::<Vec<_>>();

    let mut positions = [-1, -1, -1, -1, -1];

    match *(lines.first().unwrap()) {
        "#####" => {
            // its a lock
            lines.iter().enumerate().for_each(|(j, line)| {
                line.chars().enumerate().for_each(|(i, c)| match c {
                    '.' => {
                        if positions[i] == -1 {
                            positions[i] = (j as i32) - 1
                        }
                    }
                    '#' => {}
                    _ => panic!(),
                });
            });

            LockOrKey::Lock(positions)
            // todo!()
        }

        "....." => {
            // its a key
            let height = lines.len();

            lines.iter().enumerate().for_each(|(j, line)| {
                line.chars().enumerate().for_each(|(i, c)| match c {
                    '#' => {
                        if positions[i] == -1 {
                            positions[i] = (height as i32) - (j as i32) - 1
                        }
                    }
                    '.' => {}
                    _ => panic!(),
                });
            });

            // (0..5).for_each(|i| positions[i] = 5 - (i as i32));

            LockOrKey::Key(positions)
            // todo!()
        }
        s => {
            println!("found: {s}");
            panic!()
        }
    }
}
fn parse_input(input: String) -> (Vec<Schematic>, Vec<Schematic>) {
    // let mut locks = Vec::<Schematic>::new();
    // let mut locks = Vec::<Schematic>::new();

    input
        .split("\n\n")
        .map(|sche| parse_schematic(sche))
        .fold((vec![], vec![]), |mut acc, lok| match lok {
            LockOrKey::Lock(sche) => {
                acc.0.push(sche);
                acc
            }
            LockOrKey::Key(sche) => {
                acc.1.push(sche);
                acc
            }
        })
    // .collect::<Vec<_>>();

    // println!("matching {matching:?}");

    // // println!("los {los:?}");
    // println!("locks {locks:?}");
    // println!("keys {keys:?}");

    // (vec![], vec![])
}

fn part1(input: String) -> usize {
    let (locks, keys) = parse_input(input);

    let matching: usize = locks
        .par_iter()
        .map(|lock| {
            keys.par_iter()
                .filter(|key| {
                    // return true if they dont overlap
                    (0..5).all(|i| lock[i] + key[i] <= 5)
                })
                .count()
            // .collect::<Vec<_>>()
        })
        .sum();
    // .collect::<Vec<_>>();

    matching
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    #[test]
    fn part1_small() {
        let input = fs::read_to_string("../../inputs/D25/small.txt").unwrap();
        let result = part1(input);
        assert_eq!(result, 3)
    }
    #[test]
    fn part1_main() {
        let input = fs::read_to_string("../../inputs/D25/main.txt").unwrap();
        let result = part1(input);
        assert_eq!(result, 3)
    }
}
