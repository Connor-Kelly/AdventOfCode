use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display,
    rc::Rc,
};

use crate::matrix::{print, Coord, Direction, Matrix};

type Num = u32;

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    WALL,
    EMPTY,
    START,
    END,
    MARKED(usize),
}
impl Tile {
    fn from_str(from: &str) -> Tile {
        match from {
            "#" => Tile::WALL,
            "." => Tile::EMPTY,
            "S" => Tile::START,
            "E" => Tile::END,
            _ => panic!(),
        }
    }
}
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::WALL => "#",
            Tile::EMPTY => " ",
            Tile::START => "S",
            Tile::END => "E",
            Tile::MARKED(n) => &n.to_string(),
        };
        write!(f, "{c}")
        // Ok(())
    }
}

fn get_coord_for_direction(dir: Direction, coord: &Coord) -> (Direction, Coord) {
    match dir {
        Direction::UP => (dir, (coord.0 - 1, coord.1)),
        Direction::DOWN => (dir, (coord.0 + 1, coord.1)),
        Direction::LEFT => (dir, (coord.0, coord.1 - 1)),
        Direction::RIGHT => (dir, (coord.0, coord.1 + 1)),
    }
}

type Maze = Matrix<Tile>;

fn parse_input(input: String) -> Maze {
    input
        .split("\n")
        .map(|row| {
            row.split("")
                .filter(|e| *e != "")
                .map(|e| Tile::from_str(e))
                .collect()
        })
        .collect()

    // vec![]
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
struct Link {
    value: Num,
    dir: Direction,
    node: Rc<Node>,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
struct Node {
    coord: Coord,
    // in_list: Vec<Rc<Box<Node>>>,
    // out_list: Option<Vec<Link>>,
}

// fn traverse_map(map: &Maze, node: &Node) {}

fn get_cost(cur_dir: Direction, new_dir: Direction) -> Option<usize> {
    match cur_dir {
        _ if cur_dir == new_dir => Some(1),
        _ if new_dir as usize == cur_dir as usize + 2 % 4 => None,
        _ => Some(1001),
    }
}
// adjacentcy should take in the  map, current node
// then should return a list of valid Nodes with their respective direction
fn get_adjacent(map: &Maze, node: &Node) -> Vec<(Direction, Coord)> {
    Direction::into_iter(Direction::UP)
        // vec![
        //     (Direction::RIGHT, map[node.coord.0 + 1][node.coord.1]),
        //     (Direction::LEFT, map[node.coord.0 + 1][node.coord.1]),
        //     (Direction::RIGHT, map[node.coord.0 + 1][node.coord.1]),
        //     (Direction::RIGHT, map[node.coord.0 + 1][node.coord.1]),
        // ]
        // .iter()
        .map(|dir| get_coord_for_direction(dir, &node.coord))
        .map(|(dir, coord)| (dir, coord, map[coord.0][coord.1]))
        .filter_map(|(dir, coord, elem)| match elem {
            Tile::WALL => None,
            _ => Some((
                dir.clone(),
                coord, // Node {
                       //     coord: todo!(),
                       //     in_list: todo!(),
                       //     out_list: todo!(),
                       // },
            )),
        })
        .collect::<Vec<_>>()
}

#[derive(Debug, PartialEq, Eq, Ord, Clone)]
struct HeapElem(usize, Node, Direction);

impl<'a> PartialOrd for HeapElem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // self.0.partial_cmp(&other.0)
        other.0.partial_cmp(&self.0)
    }

    fn lt(&self, other: &Self) -> bool {
        self.partial_cmp(other)
            .is_some_and(std::cmp::Ordering::is_lt)
    }

    fn le(&self, other: &Self) -> bool {
        self.partial_cmp(other)
            .is_some_and(std::cmp::Ordering::is_le)
    }

    fn gt(&self, other: &Self) -> bool {
        self.partial_cmp(other)
            .is_some_and(std::cmp::Ordering::is_gt)
    }

    fn ge(&self, other: &Self) -> bool {
        self.partial_cmp(other)
            .is_some_and(std::cmp::Ordering::is_ge)
    }
}

fn part1(input: String) -> Num {
    let maze = parse_input(input);
    let mut cost_map = HashMap::<Coord, usize>::new();
    print(&maze);

    let start_coord = maze
        .iter()
        .enumerate()
        .find_map(|(j, row)| {
            row.iter()
                .position(|elem| *elem == Tile::START)
                .and_then(|i| Some((j, i)))
        })
        .unwrap();
    let end_coord = maze
        .iter()
        .enumerate()
        .find_map(|(j, row)| {
            row.iter()
                .position(|elem| *elem == Tile::END)
                .and_then(|i| Some((j, i)))
        })
        .unwrap();
    println!("s: {start_coord:?}");
    println!("e: {end_coord:?}");

    let mut worklist = BinaryHeap::<HeapElem>::new();

    worklist.push(HeapElem(
        0,
        Node {
            coord: start_coord,
            // in_list: vec![],
            // out_list: None,
        },
        Direction::RIGHT,
        // Rc::new(None), // vec![],
    ));

    // println!("{worklist:?}");
    let mut fin_cost = None;
    while let Some(he) = worklist.pop() {
        let _heref = Rc::new(Some(he.clone()));
        let HeapElem(cost, node, dir) = he;

        // println!("{worklist:?}");
        println!("node:{node:?}");
        println!("cost:{cost:?}");
        if node.coord == end_coord {
            // we found the accepting condition hopefully structurally
            println!("found coord{:?}", node.coord);
            println!("cost: {cost}");
            fin_cost = Some(cost);
            // panic!();
            break;
        }

        cost_map.insert(node.coord.clone(), cost);
        // get adjacent nodes
        let adj = get_adjacent(&maze, &node);
        adj.iter().for_each(|(adj_dir, adj_coord)| {
            if let Some(additional_cost) = get_cost(dir, *adj_dir) {
                if !cost_map.contains_key(adj_coord) {
                    worklist.push(HeapElem(
                        cost + additional_cost,
                        Node { coord: *adj_coord },
                        adj_dir.clone(),
                        // Rc::new(Some(he)), // vec![prev_nodes.clone(), vec![adj_coord.clone()]].concat(),
                        // heref.clone(), // vec![prev_nodes.clone(), vec![adj_coord.clone()]].concat(),
                    ));
                }
            }
        });
        // sleep(Duration::from_millis(10));
    }

    // let startNode = Node {
    //     coord: startCoord,
    //     in_list: vec![],
    //     out_list: None,
    // };

    fin_cost.unwrap() as u32
}

fn part2(input: String) -> Num {
    let maze = parse_input(input);
    let mut cost_map = HashMap::<Coord, usize>::new();
    print(&maze);

    let start_coord = maze
        .iter()
        .enumerate()
        .find_map(|(j, row)| {
            row.iter()
                .position(|elem| *elem == Tile::START)
                .and_then(|i| Some((j, i)))
        })
        .unwrap();
    let end_coord = maze
        .iter()
        .enumerate()
        .find_map(|(j, row)| {
            row.iter()
                .position(|elem| *elem == Tile::END)
                .and_then(|i| Some((j, i)))
        })
        .unwrap();
    println!("s: {start_coord:?}");
    println!("e: {end_coord:?}");

    let mut worklist = BinaryHeap::<HeapElem>::new();

    // push start node
    worklist.push(HeapElem(
        0,
        Node {
            coord: start_coord,
            // in_list: vec![],
            // out_list: None,
        },
        Direction::RIGHT,
        // Rc::new(None),
    ));

    // println!("{worklist:?}");
    let mut fin_cost = None;
    let mut marked_map = maze.clone();

    let seats = HashSet::<Coord>::new();
    // let _traceback = HashMap::<Coord, Vec<Coord>>::new();
    while let Some(he) = worklist.pop() {
        let heref = Some(he.clone());
        let HeapElem(cost, node, dir, ) = he;
        println!("{cost}");
        // iter until a most effecient path is found, then break when we find a heap elem with greater than that cost
        match &marked_map[node.coord.0][node.coord.1] {
            Tile::EMPTY => marked_map[node.coord.0][node.coord.1] = Tile::MARKED(1),
            Tile::MARKED(n) => marked_map[node.coord.0][node.coord.1] = Tile::MARKED(n + 1),
            _ => {}
        }
        // print(&marked_map);

        if let Some(found_cost) = fin_cost {
            if cost > found_cost {
                break;
            }
        }
        // println!("node:{node:?}");
        // println!("cost:{cost:?}");
        if node.coord == end_coord {
            // we found the accepting condition hopefully structurally
            println!("found coord{:?}", node.coord);
            println!("cost: {cost}");
            println!("heref: {heref:?}");
            fin_cost = Some(cost);

            // prev_nodes.iter().for_each(|c| {
            //     seats.insert(*c);
            // });
            // seats.
            // panic!();
            continue;
        }

        cost_map.insert(node.coord.clone(), cost);
        // get adjacent nodes
        let adj = get_adjacent(&maze, &node);
        adj.iter().for_each(|(adj_dir, adj_coord)| {
            if let Some(additional_cost) = get_cost(dir, *adj_dir) {
                if let Some(precomp_cost) = cost_map.get(adj_coord) {
                    if *precomp_cost > additional_cost {
                        worklist.push(HeapElem(
                            cost + additional_cost,
                            Node { coord: *adj_coord },
                            adj_dir.clone(),
                            // Rc::new(heref.clone()), // vec![prev_nodes.clone(), vec![adj_coord.clone()]].concat(),
                        ));
                        // }
                        // if !cost_map.contains_key(adj_coord) {
                    }
                } else {
                    worklist.push(HeapElem(
                        cost + additional_cost,
                        Node { coord: *adj_coord },
                        adj_dir.clone(),
                        // Rc::new(heref.clone()), // vec![prev_nodes.clone(), vec![adj_coord.clone()]].concat(),
                                                // heref.clone(), // vec![prev_nodes.clone(), vec![adj_coord.clone()]].concat(),
                                                // Rc::new(Some(node)), // vec![prev_nodes.clone(), vec![adj_coord.clone()]].concat(),
                    ));
                }
            }
        });
        // sleep(Duration::from_millis(10));
    }
    println!("{seats:?} {}", seats.len());

    (seats.len() + 1) as u32
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    #[test]
    fn part1_small() {
        let input = fs::read_to_string("../../inputs/D16/small.txt").unwrap();
        let result = part1(input);
        assert_eq!(result, 7036)
    }

    #[test]
    fn part1_small2() {
        let input = fs::read_to_string("../../inputs/D16/small2.txt").unwrap();
        let result = part1(input);
        assert_eq!(result, 11048)
    }

    #[test]
    fn part1_full() {
        let input = fs::read_to_string("../../inputs/D16/full.txt").unwrap();
        let result = part1(input);
        assert_eq!(result, 123540)
    }

    #[test]
    fn part2_small() {
        let input = fs::read_to_string("../../inputs/D16/small.txt").unwrap();
        let result = part2(input);
        assert_eq!(result, 45)
    }

    #[test]
    fn part2_small2() {
        let input = fs::read_to_string("../../inputs/D16/small2.txt").unwrap();
        let result = part2(input);
        assert_eq!(result, 64)
    }

    #[test]
    fn part2_full() {
        let input = fs::read_to_string("../../inputs/D16/full.txt").unwrap();
        let result = part2(input);
        assert_eq!(result, 123540)
    }
}
