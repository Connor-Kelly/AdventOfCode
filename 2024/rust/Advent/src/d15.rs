use regex::Regex;
use std::fmt::Display;

use crate::matrix::{self, Coord, Direction, Matrix};

#[derive(Debug, Clone, PartialEq)]
enum Object {
    WALL,
    BOX,
    EMPTY,
    ROBOT,
}
impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::WALL => {
                write!(f, "#")
            }
            Object::BOX => {
                write!(f, "O")
            }
            Object::EMPTY => {
                write!(f, ".")
            }
            Object::ROBOT => {
                write!(f, "@")
            }
        }
    }
}

type WarehouseMap = Matrix<Object>;

fn push(map: &mut WarehouseMap, coord: Coord, dir: Direction) -> bool {
    match map[coord.0][coord.1] {
        Object::WALL => false,
        Object::BOX | Object::ROBOT => {
            let next_c = dir.next_coord(coord);
            if match dir {
                Direction::UP => push(map, next_c, dir),
                Direction::DOWN => push(map, next_c, dir),
                Direction::LEFT => push(map, next_c, dir),
                Direction::RIGHT => push(map, next_c, dir),
            } {
                // move the current box and return the modified map
                map[next_c.0][next_c.1] = map[coord.0][coord.1].clone();
                map[coord.0][coord.1] = Object::EMPTY;
                true
                // Some((map, next_c))
            } else {
                // Some((map, coord))
                false
            }
        }
        Object::EMPTY => true,
    }
}

// fn printMap(map: &WarehouseMap) {
//     print!(" ");
//     (0..map.first().unwrap().len()).for_each(|n| print!(" {n}"));
//     print!("\n");
//     map.iter().enumerate().for_each(|(j, row)| {
//         print!("{j}");
//         row.iter().for_each(|entry| print!(" {entry}"));
//         print!("\n");
//     });
// }

fn parse_map(input: &str) -> WarehouseMap {
    input
        .split("\n")
        .map(|line| {
            println!("{line:?}");
            line.split("")
                .filter(|c| *c != "")
                .map(|c| {
                    // println!("{c:?}");
                    match c {
                        "#" => Object::WALL,
                        "O" => Object::BOX,
                        "." => Object::EMPTY,
                        "@" => Object::ROBOT,
                        _ => {
                            panic!()
                        }
                    }
                })
                .collect::<Vec<Object>>()
        })
        .collect::<WarehouseMap>()
    // println!("{map:?}");

    // vec![]
}

fn get_map_value(map: &WarehouseMap) -> u32 {
    map.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(j, o)| match o {
                    Object::BOX => Some(i * 100 + j),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .sum::<usize>() as u32
    // 0
}

fn part1(input: String) -> u32 {
    let mut map_input = input.clone();
    let mut moves_input = map_input.split_off(
        Regex::new("\n\n")
            .unwrap()
            .shortest_match(&map_input)
            .unwrap(),
    );
    moves_input.remove_matches("\n");
    map_input = map_input.trim().to_string();
    println!("map_input {map_input:?}");
    println!("moves_input {moves_input:?}");

    let mut map = parse_map(&map_input);

    let mut robot_location = map
        .iter()
        .enumerate()
        .find_map(|(j, row)| {
            row.iter()
                .position(|c| *c == Object::ROBOT)
                .and_then(|i| Some((j, i)))
        })
        .unwrap();
    // println!("rl: {robot_location:?}");
    matrix::print(&map);

    moves_input
        .split("")
        .filter(|c| *c != "")
        // .take(3)
        .map(|c| Direction::fromStr(c))
        .for_each(|dir| {
            // println!("{dir:?}");
            if push(&mut map, robot_location, dir.clone()) {
                robot_location = dir.next_coord(robot_location);

                matrix::print(&map);
            } else {
                // println!("failed to push")
            }

            // println!("{m:?}");
        });

    get_map_value(&map)
}

fn preprocess_doublewide(input: String) -> String {
    input
        .split("\n")
        .map(|row| {
            println!("row: {row:?}");
            let char_iter = &mut row.chars();
            let mut collapsed_row = "".to_string();
            while let Ok(pair) = char_iter.next_chunk::<2>() {
                println!("pair: {pair:?}");
                match pair {
                    ['#', '#'] => collapsed_row = collapsed_row + "#",
                    ['[', ']'] => collapsed_row = collapsed_row + "O",
                    ['.', '.'] => collapsed_row = collapsed_row + ".",
                    ['@', '.'] => collapsed_row = collapsed_row + "@",

                    _ => {
                        println!("failed pair: {pair:?}");
                        panic!()
                    }
                }
            }
            collapsed_row
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn part2(input: String) -> u32 {
    let mut map_input = input.clone();
    let mut moves_input = map_input.split_off(
        Regex::new("\n\n")
            .unwrap()
            .shortest_match(&map_input)
            .unwrap(),
    );
    moves_input.remove_matches("\n");
    map_input = map_input.trim().to_string();
    map_input = preprocess_doublewide(map_input);
    println!("map_input {map_input:?}");
    println!("moves_input {moves_input:?}");
    part1(format!("{map_input}\n\n{moves_input}").to_string())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    #[test]
    fn part1_small() {
        let input = fs::read_to_string("../../inputs/D15/small.txt").unwrap();
        let result = part1(input);
        assert_eq!(result, 10092)
    }

    #[test]
    fn part1_tiny() {
        let input = fs::read_to_string("../../inputs/D15/tiny.txt").unwrap();
        let result = part1(input);
        assert_eq!(result, 2028)
    }

    #[test]
    fn part1_main() {
        let input = fs::read_to_string("../../inputs/D15/main.txt").unwrap();
        let result = part1(input);
        assert_eq!(result, 1415498)
    }

    // #[test]
    fn part2_tiny() {
        let input = fs::read_to_string("../../inputs/D15/tinyp2.txt").unwrap();
        let result = part2(input);
        assert_eq!(result, 2028)
    }
}
