use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

type Pair = (String, String);

fn parse_input(input: String) -> Vec<Pair> {
    input
        .split("\n")
        .map(|line| {
            let spl = line.split("-").collect::<Vec<_>>();
            (
                spl.get(0).unwrap().to_string(),
                spl.get(1).unwrap().to_string(),
            )
        })
        .collect()
}

#[derive(Eq, PartialEq, Hash)]
struct Tripp(String, String, String);
impl Tripp {
    pub fn new(a: String, b: String, c: String) -> Self {
        let mut arr = [a.clone(), b.clone(), c.clone()];
        arr.sort();
        Tripp(arr[0].clone(), arr[1].clone(), arr[2].clone())
    }
}
// impl Eq for Tripp {

// }
// impl PartialEq for Tripp {
//     fn eq(&self, other: &Self) -> bool {
//         (self.0 == other.0 || self.0 == other.1 || self.0 == other.2)
//             && (self.1 == other.0 || self.1 == other.1 || self.1 == other.2)
//             && (self.2 == other.0 || self.2 == other.1 || self.0 == other.2)
//         // self.0 == other.0 && self.1 == other.1 && self.2 == other.2
//     }
// }
impl Debug for Tripp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}
// impl Hash for Tripp {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         let mut arr = [self.0.clone(), self.1.clone(), self.2.clone()];
//         arr.sort();
//         arr.iter().for_each(|a| (a).hash(state));
//     }
// }

fn part1(input: String) -> usize {
    let pairs = parse_input(input);
    let mut network_map = HashMap::<String, HashSet<&str>>::new();
    println!("{pairs:?}");

    pairs.iter().for_each(|p| {
        if let Some(first) = network_map.get_mut(&p.0) {
            first.insert(&p.1);
        } else {
            network_map.insert(p.0.clone(), HashSet::from([p.1.as_str()]));
        }
        if let Some(second) = network_map.get_mut(&p.1) {
            second.insert(&p.0);
        } else {
            network_map.insert(p.1.clone(), HashSet::from([p.0.as_str()]));
        }
    });

    dbg!(&network_map);

    let trips = network_map
        .iter()
        .flat_map(|(comp, connected)| {
            connected
                .iter()
                .map(|conn| (comp.to_string(), conn.to_string()))
                .collect::<Vec<_>>()
        })
        .flat_map(|(first, second)| {
            network_map
                .get(second.as_str())
                .unwrap()
                .iter()
                .map(|third| (first.clone(), second.clone(), third.to_string().clone()))
                .collect::<Vec<_>>()
        })
        .filter(|(first, _second, third)| network_map.get(third).unwrap().contains(first.as_str()))
        .map(|(first, second, third)| Tripp::new(first, second, third))
        .fold(HashSet::new(), |mut acc, set| {
            // dbg!(&set);
            acc.insert(set);
            // dbg!(&acc);
            acc
        });
    // dbg!(trips);
    // println!("{network_map:?}");
    println!("{trips:?}");
    println!("len {:?}", trips.len());

    trips
        .iter()
        .filter(|trip| {
            trip.0.starts_with("t") || trip.1.starts_with("t") || trip.2.starts_with("t")
        })
        .count()

    // 0
}

fn part2(input: String) -> String {
    let pairs = parse_input(input);
    let mut network_map = HashMap::<String, HashSet<&str>>::new();
    println!("{pairs:?}");

    pairs.iter().for_each(|p| {
        if let Some(first) = network_map.get_mut(&p.0) {
            first.insert(&p.1);
        } else {
            network_map.insert(p.0.clone(), HashSet::from([p.1.as_str()]));
        }
        if let Some(second) = network_map.get_mut(&p.1) {
            second.insert(&p.0);
        } else {
            network_map.insert(p.1.clone(), HashSet::from([p.0.as_str()]));
        }
    });

    dbg!(&network_map);

    // let _ = network_map.iter().map(|(head, elems)| {
    //     // elems.iter().collect();

    //     let es = elems.iter().collect::<Vec<_>>();

    //     // elems.iter().map(|e| {
    //     //     network_map.get(*e).unwrap()
    //     // })
    //     // es.iter().filter(|e| {
    //     // })
    // });

    // let trips = network_map
    //     .iter()
    //     .flat_map(|(comp, connected)| {
    //         connected
    //             .iter()
    //             .map(|conn| (comp.to_string(), conn.to_string()))
    //             .collect::<Vec<_>>()
    //     })
    //     .flat_map(|(first, second)| {
    //         network_map
    //             .get(second.as_str())
    //             .unwrap()
    //             .iter()
    //             .map(|third| (first.clone(), second.clone(), third.to_string().clone()))
    //             .collect::<Vec<_>>()
    //     })
    //     .filter(|(first, second, third)| network_map.get(third).unwrap().contains(first.as_str()))
    //     .map(|(first, second, third)| Tripp::new(first, second, third))
    //     .fold(HashSet::new(), |mut acc, set| {
    //         // dbg!(&set);
    //         acc.insert(set);
    //         // dbg!(&acc);
    //         acc
    //     });
    // dbg!(trips);
    // println!("{network_map:?}");
    // println!("{trips:?}");
    // println!("len {:?}", trips.len());

    // let mut connected_components: Vec<HashSet<String>> = vec![];
    // // pop a single node off the list of nodes still unexplored
    // while let Some(head_node) = network_map
    //     .keys()
    //     .filter(|label| !connected_components.iter().any(|set| set.contains(*label)))
    //     // .take(1)
    //     .collect::<Vec<_>>()
    //     .first()
    // {
    //     let mut set = HashSet::from([(*head_node).clone()]);

    //     let mut set_len = set.len();
    //     for i in 0.. {
    //         // dbg!(&set);
    //         println!("{i}: {set:?}");
    //         set.clone().iter().for_each(|elem| {
    //             network_map.get(elem).unwrap().iter().for_each(|conn| {
    //                 set.insert(conn.to_string());
    //             });
    //         });

    //         if (set.len() == set_len) {
    //             break;
    //         } else {
    //             set_len = set.len()
    //         }
    //     }
    //     println!("{set:?}");
    //     // dbg!(&set);
    //     connected_components.push(set);
    // }
    // // dbg!(&connected_components);
    //     println!("{connected_components:?}");

    // // create new hashset, push main node into it

    // // check the count, add all nodes connected to all nodes

    // // if the size didnt increase, break

    // connected_components.sort_by(|a, b| b.len().cmp(&a.len()));

    // let largest_componenet = connected_components.first().unwrap();
    // let mut lc = largest_componenet
    //     .iter()
    //     .map(|s| s.to_string())
    //     .collect::<Vec<String>>();

    // lc.sort();
    // lc.join(",")

    // trips
    //     .iter()
    //     .filter(|trip| {
    //         trip.0.starts_with("t") || trip.1.starts_with("t") || trip.2.starts_with("t")
    //     })
    //     .count()
    "".to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    #[test]
    fn part1_small() {
        let input = fs::read_to_string("../../inputs/D23/small.txt").unwrap();
        let result = part1(input);
        assert_eq!(result, 7)
    }
    #[test]
    fn part1_main() {
        let input = fs::read_to_string("../../inputs/D23/main.txt").unwrap();
        let result = part1(input);
        assert_eq!(result, 1368)
    }
    #[test]
    fn part2_small() {
        let input = fs::read_to_string("../../inputs/D23/tiny.txt").unwrap();
        let result = part2(input);
        assert_eq!(result, "co,de,ka,ta")
    }
    // #[test]
    fn part2_test() {
        // let input = fs::read_to_string("../../inputs/D23/tiny.txt").unwrap();
        let input = "ab-ac
ac-ad
da-dc"
            .to_string();
        let result = part2(input);
        assert_eq!(result, "co,de,ka,ta")
    }
    #[test]
    fn part2_main() {
        let input = fs::read_to_string("../../inputs/D23/main.txt").unwrap();
        let result = part2(input);
        assert_eq!(result, "co,de,ka,ta")
    }
}
