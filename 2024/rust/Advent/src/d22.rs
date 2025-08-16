use std::{collections::HashSet, ops::BitXor, rc::Rc};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

type Num = u64;
fn mix(secret: Num, mix: Num) -> Num {
    secret.bitxor(mix)
}
fn prune(secret: Num) -> Num {
    secret % 16777216
}

struct SecretGenerator {
    current_secret: u64,
}
impl Iterator for SecretGenerator {
    type Item = Num;

    fn next(&mut self) -> Option<Self::Item> {
        let calc = |secret| {
            let s1 = prune(mix(secret, secret * 64));

            let s2 = prune(mix(s1, ((s1 as f64) / 32.).floor() as Num));
            let s3 = prune(mix(s2, s2 * 2048));
            s3
        };
        let next_secret = calc(self.current_secret);
        self.current_secret = next_secret;
        Some(self.current_secret)
        // todo!()
    }
}

fn part1(input: Vec<Num>) -> Vec<Num> {
    let secrets = input
        .par_iter()
        .map(|init_secret| {
            SecretGenerator {
                current_secret: *init_secret,
            }
            .nth(2000 - 1)
        })
        .map(|a| a.unwrap())
        .collect::<Vec<_>>();
    secrets
}

fn part2(input: Vec<Num>) -> Vec<Num> {
    // add all possible sequesnces into the sequences
    let mut sequences = HashSet::<Vec<_>>::new();
    let secrets = input
        .iter()
        .map(|init_secret| {
            let mut secret_list = SecretGenerator {
                current_secret: *init_secret,
            }
            .take(10)
            .map(|n| n % 10)
            .collect::<Vec<_>>();
            secret_list.insert(0, init_secret % 10);
            // secret_list

            let secrets = secret_list;
            let values = secrets.clone();
            println!("val: {values:?}");

            let mut seq = vec![];
            let mut prev = secrets.first().unwrap();
            for secret in secrets.iter() {
                seq.push((*secret) as i64 - (*prev as i64));
                prev = secret;
            }
            println!("{seq:?}");
            (0..seq.len() - 3).for_each(|i| {
                let window = seq.get(i..i + 4).unwrap();
                sequences.insert((window).to_vec());
            });

            (values, seq)

        })
        .collect::<Vec<_>>();
    println!("seqences: {sequences:?}");
    println!("secrets:  {secrets:?}");

    // make a heap, for each seqence in sequences, find the value of the secret
    sequences.iter().map(|seq| {
        secrets.iter().map(|secret| {
            (0..secret.len()-3).take_while(|&i| {
                secret.get(i..i+4).unwrap() != seq

            }).count()

        })

    });

    // secrets
    vec![1]
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    #[test]
    fn part1_small() {
        // let input = fs::read_to_string("../../inputs/D23/small.txt").unwrap();
        let result = part1([1, 10, 100, 2024].to_vec());
        assert_eq!(result, vec![8685429, 4700978, 15273692, 8667524]);

        assert_eq!(result.iter().sum::<Num>(), 37327623);
    }
    #[test]
    fn part1_main() {
        let input = fs::read_to_string("../../inputs/D22/main.txt").unwrap();
        let parsed = input
            .split("\n")
            .map(|l| l.parse::<Num>().unwrap())
            .collect::<Vec<_>>();

        let result = part1(parsed);

        assert_eq!(result.iter().sum::<Num>(), 12979353889);
        // assert_eq!(result, vec![8685429, 4700978, 15273692, 8667524])
    }
    #[test]
    fn part2_small() {
        // let input = fs::read_to_string("../../inputs/D23/small.txt").unwrap();
        // let result = part2(vec![1, 2, 3, 2024]);
        let result = part2(vec![123]);
        // assert_eq!(result, vec![8685429, 4700978, 15273692, 8667524]);

        assert_eq!(result.iter().sum::<Num>(), 37327623);
    }
    #[test]
    fn mix_works() {
        assert_eq!(mix(42, 15), 37);
    }
    #[test]
    fn prune_works() {
        assert_eq!(prune(100000000), 16113920);
    }
}
