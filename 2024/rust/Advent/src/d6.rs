use std::{convert::Into, fs, vec};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

fn get_direction(s: &str) -> Direction {
    match s {
        "^" => Direction::Up,
        "v" => Direction::Down,
        ">" => Direction::Right,
        "<" => Direction::Left,
        _ => panic!(),
    }
}

#[derive(Debug, Clone, Copy)]
struct Position<T> {
    x: T,
    y: T,
}
impl Into<Position<i32>> for Position<usize> {
    fn into(self) -> Position<i32> {
        Position {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}

fn get_next_position(dir: Direction, cur_pos: Position<i32>) -> Position<i32> {
    match dir {
        Direction::Up => Position {
            x: cur_pos.x,
            y: cur_pos.y - 1,
        },
        Direction::Down => Position {
            x: cur_pos.x,
            y: cur_pos.y + 1,
        },
        Direction::Left => Position {
            x: cur_pos.x - 1,
            y: cur_pos.y,
        },
        Direction::Right => Position {
            x: cur_pos.x + 1,
            y: cur_pos.y,
        },
    }

    // Position { x: next.x as i32, y: next.y as i32 }
}
// struct Position<T> {
//     x: T,
//     y: T,
// }
// impl<T> From<Position<T>> for Position<T> {
//     // fn from(val: Position) -> Self {
//     //     (val.0 as i32, val.1 as i32)
//     // }

//     fn from(val: Position<T>) -> Self {
//         Position{
//             x: val.x as T,
//             y: val.y as T,
//         }
//     }
// }
// fn nextPosition()

fn trace_guard_path(
    position: Position<i32>,
    dir: Direction,
    map: &Vec<Vec<&str>>,
) -> Option<(Direction, Position<i32>)> {
    let forward_position = get_next_position(dir, position);

    // if off of map
    if forward_position.x < 0
        || forward_position.x >= map[0].len() as i32
        || forward_position.y < 0
        || forward_position.y >= map.len() as i32
    {
        // println!("offmap: {:?}", forward_position);
        None
        // vec![]
    } else if map[forward_position.y as usize][forward_position.x as usize] == "#" {
        // colides with object, turn right
        let new_direction = dir.turn();
        let next_position = get_next_position(new_direction, position);

        // println!("collision: {:?} {:?}", next_position, new_direction);
        // let mut rol = trace_guard_path(next_position, new_direction, map);
        Some((new_direction, next_position))

        // rol.push(next_position);
        // rol
        // [
        //     vec![forward_position],
        //     trace_guard_path(next_position, new_direction, map),
        // ]
        // .concat()
    } else {
        // standard move forward case
        // println!("standard: {:?}", forward_position);
        // let mut rol =
        //     trace_guard_path(forward_position, dir, map);

        // rol.push(forward_position);
        // rol
        // [
        //     vec![forward_position],
        //     trace_guard_path(forward_position, dir, map),
        // ]
        // .concat()
        Some((dir, forward_position))
    }
}

fn part1(input: String) -> i32 {
    let mut map: Vec<_> = input
        .split("\n")
        .map(|line| line.split("").collect::<Vec<_>>())
        .collect();
    // println!("map:\n{:?}", map);

    let init_guard_positions = map
        .clone()
        .iter()
        .enumerate()
        .filter_map(|(y, line)| {
            let locations = line
                .iter()
                .enumerate()
                .filter_map(|(x, elem)| {
                    // println!("{:?} {:?} {:?} : {:?}", x, y, elem, *elem == "^");

                    if *elem == "^" {
                        Some(Position { y, x })
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            // println!("locations {:?}", locations);
            if locations.len() > 0 {
                Some(locations)
            } else {
                None
            }
        })
        .flatten()
        .collect::<Vec<_>>();
    // println!("{:?}", initGuardPositions);
    let mut gaurd_position: Position<i32> = init_guard_positions.first().unwrap().to_owned().into();
    let mut dir = Direction::Up;
    let mut visited = vec![gaurd_position];

    println!("guard_position: {:?}", gaurd_position);
    println!("dir: {:?}", dir);
    // println!("len: {:?}", len);
    // let mut result = trace_guard_path((gaurd_position).into(), Direction::Up, map);
    while let Some((new_dir, new_pos)) = trace_guard_path(gaurd_position.clone().into(), dir, &map)
    {
        visited.push(new_pos);
        gaurd_position = new_pos;
        dir = new_dir;
        map[gaurd_position.y as usize][gaurd_position.x as usize] = "X";
//         print!("{}[2J", 27 as char);
//         println!("{}", map.iter().map(|line| line.join("")).collect::<Vec<_>>().join("
// "));
        // println!("visited: {:?}", visited);
        // sleep(Duration::from_secs_f32(0.005));
        ()
    }
    // println!("visited: {:?}", visited);
    // 0
    visited.sort_by(|a, b| {
        if a.x == b.x {
            a.y.cmp(&b.y)
        } else {
            a.x.cmp(&b.x)
        }
    });
    visited.dedup_by(|a, b| a.x == b.x && a.y == b.y);
    // println!("result: {:?}", visited);
    println!("len: {:?}", visited.len());
    
    let mapcount: usize = map.iter().map(|line| line.iter().filter(|elem|{**elem == "X"}).count()).sum();
    println!("mapcount: {:?}", mapcount);

    visited.len() as i32
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    #[test]
    fn part1_small() {
        let input = fs::read_to_string("../../inputs/D6/small.txt").unwrap();
        let result = part1(input);
        assert_eq!(result, 41)
    }

    // #[test]
    // fn part1_main() {
    //     let input = fs::read_to_string("../../inputs/D6/main.txt").unwrap();
    //     let result = part1(input);
    //     assert_eq!(result, 41)
    // }
    // 5454 is too high

    // #[test]
    // fn part1_main() {
    //     let input = fs::read_to_string("../../inputs/D1/main_input.txt").unwrap();
    //     let result = part1(input);
    //     assert_eq!(result, 1941353)
    // }
}

fn main() {
    let input = fs::read_to_string("../../inputs/D6/main.txt").unwrap();
    let result = part1(input);
    println!("result {:?}", result)
}
