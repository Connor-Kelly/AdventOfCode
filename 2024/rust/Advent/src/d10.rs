fn part1(input: String) -> i32 {
    let mat = input.split("\n").map(|line| {
        line.split("")
            .filter(|i| i.len() > 0)
            .map(|i| i.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    });

    let trailheads = mat
        .enumerate()
        .flat_map(|(j, row)| {
            let v = row
                .iter()
                .enumerate()
                .filter(|(_, elem)| **elem == 0)
                .map(|(i, _)| (j.clone(), i.clone()))
                .collect::<Vec<_>>();
            v
        })
        .collect::<Vec<_>>();

    println!("trailheads: {:?}", trailheads);
    0
}

// fn part2(input: String) -> i32 {
//     let list = input
//         .split("\n")
//         .map(|line| line.split("   ").collect::<Vec<_>>())
//         .filter(|line| line.len() == 2)
//         .fold(vec![vec![], vec![]], |mut acc, line| {
//             acc[0].push(line[0]);
//             acc[1].push(line[1]);
//             acc
//         });
//     let mut a = list[0].clone();
//     a.sort();
//     let mut b = list[1].clone();
//     b.sort();
//     a.iter().map(|va| {
//         (*va).parse::<usize>().unwrap() *
//         b.clone().iter().filter(|vb| {va.eq(*vb)}).count()
//     }).sum::<usize>() as i32
// let diff = a.iter().zip(b.iter()).fold(0, |acc, (va, vb)| {
//     println!("{:?} {:?}", va, vb);

//     acc + ((*vb).parse::<i32>().unwrap() - (*va).parse::<i32>().unwrap()).abs()
// });

// // println!("diff: {:?}", diff);
// diff
// }

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    #[test]
    fn part1_small() {
        let input = fs::read_to_string("../../inputs/D10/small.txt").unwrap();
        let result = part1(input);
        assert_eq!(result, 36)
    }

    // #[test]
    // fn part1_main() {
    //     let input = fs::read_to_string("../../inputs/D1/main_input.txt").unwrap();
    //     let result = part1(input);
    //     assert_eq!(result, 1941353)
    // }

    // #[test]
    // fn part2_small() {
    //     let input = fs::read_to_string("../../inputs/D1/small_input.txt").unwrap();
    //     let result = part2(input);
    //     assert_eq!(result, 31)
    // }

    // #[test]
    // fn part2_main() {
    //     let input = fs::read_to_string("../../inputs/D1/main_input.txt").unwrap();
    //     let result = part2(input);
    //     assert_eq!(result, 22539317)
    // }
}
