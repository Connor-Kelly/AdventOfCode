use std::{
    collections::{BinaryHeap, HashMap},
    sync::RwLock,
    time::SystemTime,
};

fn get_next_stones(stone: u128) -> Vec<u128> {
    match stone {
        0 => vec![1],
        s if s.ilog10() % 2 == 1 => {
            let mut first = s.to_string();
            let second = first.split_off(first.len() / 2);
            vec![
                first.parse::<u128>().unwrap_or(0),
                second.parse::<u128>().unwrap_or(0),
            ]
        }

        s => vec![s * 2024],
    }
}

fn get_stones_memoized(stone: u128, precalc: &RwLock<HashMap<u128, Vec<u128>>>) -> Vec<u128> {
    if let Ok(hm) = precalc.get_cloned() {
        if let Some(found) = hm.get(&stone) {
            // println!("memopull: {stone} {found:?}");
            found.clone()
        } else {
            let next_stones = get_next_stones(stone);

            if let Ok(mut lockhm) = precalc.try_write() {
                // println!("memoinsert: {} {next_stones:?}", stone);
                lockhm.insert(stone, next_stones.clone());
            }
            next_stones
        }
    } else {
        println!("memofail");
        get_next_stones(stone)
    }
    // vec![];
}

fn recursive_get_stones(
    stones: Vec<u128>,
    iteration: u32,
    max_found: &RwLock<u32>,
    precalc: &RwLock<HashMap<u128, Vec<u128>>>,
) -> Vec<u128> {
    let found = match iteration {
        0 => vec![],
        _ => stones
            .iter()
            .flat_map(|stone| {
                recursive_get_stones(
                    get_stones_memoized(*stone, precalc),
                    iteration - 1,
                    max_found,
                    precalc,
                )
            })
            .collect(),
    };

    if let Ok(iters) = max_found.get_cloned() {
        if iteration > iters {
            if let Ok(mut mfref) = max_found.try_write() {
                println!("iteration: {iteration} {:?}", SystemTime::now());
                *mfref = iteration;
            }
            // *maxFound.borrow_mut() = iteration;
        }
    }
    found
}

fn part1(stones: Vec<u128>, iterations_to_run: u32) -> u32 {
    // println!("{stones:?}");
    (0..iterations_to_run)
        .fold(stones, |acc, _i| {
            // println!("i: {i}");
            let st = acc.iter().flat_map(|stone| get_next_stones(*stone)).collect();
            // println!("{st:?}");
            st
        })
        .len() as u32
}

#[derive(Debug)]
struct HeapHashSet {
    map: HashMap<u128, u128>,
    worklist: BinaryHeap<u128>,
}
impl HeapHashSet {
    fn new() -> HeapHashSet {
        HeapHashSet {
            map: HashMap::<u128, u128>::new(),
            worklist: BinaryHeap::<u128>::new(),
        }
    }
    fn pop(&mut self) -> Option<u128> {
        self.worklist.pop()
    }
    fn push(&mut self, val: u128) {
        if !self.map.contains_key(&val) {
            println!("inserting new key: {val}");
            self.worklist.push(val);
            self.map.insert(val, 0);
        }
    }
    fn add(&mut self, key: u128, _val: Vec<u128>) {
        if let Some(prev_val) = self.map.get(&key) {
            self.map.insert(key, *prev_val + 1);
        }
    }
    fn get(&mut self, key: u128) -> &u128 {
        self.map.get(&key).unwrap()
    }
}

fn part2(stones: Vec<u128>, iterations_to_run: u32) -> u32 {
    // : RwLock<HashMap<u128, Vec<u128>>>
    // // let precalc = RwLock::new(HashMap::<u128, Vec<u128>>::new());
    // let precalc = HashMap::<u128, Vec<u128>>::new();
    // // let mut worklist = RwLock::new(BinaryHeap::<u128>::new());
    // let mut worklist = BinaryHeap::<u128>::new();
    let hhs = &mut HeapHashSet::new();

    // if let Ok(mut ww) = worklist.write() {
    stones.iter().for_each(|stone| hhs.push(*stone));
    // }
    println!("init worklist: {:?}", hhs);

    // while let Ok(mut ww) = worklist.write() {
    let mut cur_stones = stones.clone();
    (0..iterations_to_run).for_each(|i| {
        cur_stones.iter().for_each(|stone| hhs.push(*stone));

        while let Some(popped) = hhs.pop() {
            // if hhs.get(&popped).is_some() {
            //     continue;
            // }
            // println!("popped: {popped} {worklist:?}");
            // println!("popped: {popped}");
            let next = get_next_stones(popped);
            hhs.add(popped, next.clone());
            // let pc = precalc.ap();
            next.iter()
                // .filter(|stone| precalc.get(&stone).is_none())
                .for_each(|stone| {
                    hhs.push(*stone);
                });
        }
        cur_stones = cur_stones
            .iter()
            .map(|stone| {
                let found = hhs.get(*stone).clone();
                // println!("{found:?} {:?}", hhs.map);
                found
            })
            .collect();

        // println!("cs: {cur_stones:?}")
        println!("iteration: {i} {:?}", cur_stones)
    });

    0
    // println!("{stones:?}");
    // let cur_iter = &mut iterations_to_run.clone();
    // let mut iters = &mut vec![stones.clone()];
    // while iters.len() < iterations_to_run as usize {
    //     let mut next = &mut vec![];
    //     for stone in iters.last().unwrap() {
    //         let mut ns = getStonesMemoized(*stone, precalc);
    //         next.append(&mut ns);
    //     }

    //     iters.push(next.clone());

    //     println!("completed iteration: {}", iters.len())
    //     // iters
    //     //     .iter()
    //     //     .enumerate()
    //     //     .for_each(|iteration| println!("{:?}", iteration));
    // }

    // iters.last().iter().len() as u32
    // (0..iterations_to_run)
    //     .fold(stones, |acc, i| {
    //         // println!("i: {i}");
    //         let st = acc.iter().flat_map(|stone| getNextStones(*stone)).collect();
    //         // println!("{st:?}");
    //         st
    //     })
    //     .len() as u32
}

#[cfg(test)]
mod tests {
    // use std::fs;

    use super::*;
    #[test]
    fn part1_small() {
        // let input = fs::read_to_string("../../inputs/D9/small.txt").unwrap();
        let input = "125 17";

        let result = part1(
            input
                .split(" ")
                .map(|s| s.parse::<u128>().unwrap())
                .collect::<Vec<u128>>(),
            25,
        );
        assert_eq!(result, 55312)
    }

    #[test]
    fn part1_full() {
        // let input = fs::read_to_string("../../inputs/D9/small.txt").unwrap();
        let input = "4022724 951333 0 21633 5857 97 702 6";

        let result = part1(
            input
                .split(" ")
                .map(|s| s.parse::<u128>().unwrap())
                .collect::<Vec<u128>>(),
            25,
        );
        assert_eq!(result, 211306)
    }

    // #[test]
    fn part2_full() {
        // let input = fs::read_to_string("../../inputs/D9/small.txt").unwrap();
        let input = "4022724 951333 0 21633 5857 97 702 6";

        // let _max_found = RwLock::new(0);
        // let result = recursiveGetStones(
        //     input
        //         .split(" ")
        //         .map(|s| s.parse::<u128>().unwrap())
        //         .collect::<Vec<u128>>(),
        //     75,
        //     &max_found,
        //     &precalc,
        // )
        // let result = input
        //     .split(" ")
        //     .map(|s| s.parse::<u128>().unwrap())
        //     .collect::<Vec<u128>>()
        //     .iter()
        //     .flat_map(|stone| getStonesMemoized(*stone, &precalc))
        //     .collect::<Vec<u128>>()
        //     .len();
        let result = part2(
            input
                .split(" ")
                .map(|s| s.parse::<u128>().unwrap())
                .collect::<Vec<u128>>(),
            75,
        );
        assert_eq!(result, 55312)
    }
}
