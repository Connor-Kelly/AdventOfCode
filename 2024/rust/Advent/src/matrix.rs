use std::fmt::Display;

pub type Coord = (usize, usize);

pub type Matrix<A> = Vec<Vec<A>>;

// pub struct Matrix<A>(pub Vec<Vec<A>>);
// trait MatDisplay: Display {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;

// }
// impl<A> Display for Matrix<A> {
pub fn print<A: Display>(mat: &Matrix<A>) {
    // fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // todo!();
    let left_padding = (mat.len().ilog10() + 1) as usize;
    let down_padding = (mat.first().unwrap().len().ilog10() + 1) as usize;
    let pad_left = |s: String, padding: usize| {
        if s.len() >= padding {
            s
        } else {
            " ".repeat(padding - s.len() + 0) + &s
        }
    };
    let pad_down = |s: String, padding: usize| {
        if s.len() >= padding {
            s
        } else {
            " ".repeat(padding - s.len() + 0) + &s
        }
    };

    print!("{}", pad_left(" ".to_string(), left_padding));
    (0..down_padding).for_each(|row| {
        print!("\n{}", " ".repeat(left_padding));
        (0..mat.first().unwrap().len())
            .map(|n| pad_down(n.to_string(), down_padding))
            .for_each(|n| print!(" {}", n.get(row..row + 1).unwrap()));
    });
    print!("\n");
    mat.iter().enumerate().for_each(|(j, row)| {
        print!("{}", pad_left(j.to_string(), left_padding));
        row.iter().for_each(|entry| print!(" {entry}"));
        print!("\n");
    });
}
// }

// impl<A: Display> Display for SMAtrix<A> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         // todo!();
//         print!(" ");
//         (0..self.0.first().unwrap().len()).for_each(|n| print!(" {n}"));
//         print!("\n");
//         self.0.iter().enumerate().for_each(|(j, row)| {
//             print!("{j}");
//             row.iter().for_each(|entry| print!(" {entry}"));
//             print!("\n");
//         });
//         Ok(())
//     }
// }

#[derive(Debug, Clone,Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Direction {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}

struct EnumIterator<Item> {
    curr: Vec<Item>,
}

impl Iterator for EnumIterator<Direction> {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl IntoIterator for Direction {
    type Item = Self;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            Direction::UP,
            Direction::DOWN,
            Direction::LEFT,
            Direction::RIGHT,
        ]
        .into_iter()
        // EnumIterator{
        //     curr: vec![
        //         Direction::UP,
        //         Direction::DOWN,
        //         Direction::LEFT,
        //         Direction::RIGHT,
        //     ]
        // }
    }
}
impl Direction {
    pub fn fromStr(c: &str) -> Self {
        match c {
            "^" => Direction::UP,
            "v" => Direction::DOWN,
            ">" => Direction::RIGHT,
            "<" => Direction::LEFT,
            _ => {
                panic!();
            }
        }
    }
    pub fn next_coord(&self, coord: Coord) -> Coord {
        match self {
            Direction::UP => (coord.0 - 1, coord.1),
            Direction::DOWN => (coord.0 + 1, coord.1),
            Direction::LEFT => (coord.0, coord.1 - 1),
            Direction::RIGHT => (coord.0, coord.1 + 1),
        }
    }
}
