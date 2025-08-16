use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, PartialEq)]
struct InitValue(String, bool);

fn parse_init_inputs(input: String) -> Vec<InitValue> {
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let sline = line.split(": ").collect::<Vec<_>>();
            println!("{sline:?}");
            InitValue(
                sline[0].to_owned(),
                sline[1].parse::<u32>().map(|n| n == 1).unwrap(),
            )
        })
        .collect()
}

#[derive(PartialEq, Debug)]
enum Operation {
    And,
    Or,
    Xor,
}

impl Operation {
    fn apply(&self, in1: bool, in2: bool) -> bool {
        match self {
            Operation::And => in1 && in2,
            Operation::Or => in1 || in2,
            Operation::Xor => in1 != in2,
        }
    }
}

#[derive(PartialEq, Debug)]
struct Gate {
    inputs: (String, String),
    operation: Operation,
    output: String,
}

fn parse_gate_inputs(input: String) -> Vec<Gate> {
    input
        .split("\n")
        .map(|line| {
            let sline = line.split(" ").collect::<Vec<_>>();
            let (in1, in2, op, out) = (sline[0], sline[2], sline[1], sline[4]);

            let operation = match op {
                "AND" => Operation::And,
                "OR" => Operation::Or,
                "XOR" => Operation::Xor,
                _ => todo!(),
            };

            Gate {
                inputs: (in1.to_owned(), in2.to_owned()),
                operation: operation,
                output: out.to_owned(),
            }
        })
        .collect::<Vec<_>>()
}

fn parse_main_input(input: String) -> (Vec<InitValue>, Vec<Gate>) {
    let split: Vec<_> = input.split("\n\n").collect();
    let (init_inputs, gate_inputs) = (split[0], split[1]);
    let inits = parse_init_inputs(init_inputs.to_owned());
    let gates = parse_gate_inputs(gate_inputs.to_owned());

    (inits, gates)
}

fn part1(input: String) -> u64 {
    let (inits, gates) = parse_main_input(input);
    let mut completed_gates = HashMap::<String, bool>::new();
    inits.iter().for_each(|init| {
        completed_gates.insert(init.0.clone(), init.1);
    });

    let mut working_gates = gates;
    while !working_gates.is_empty() {
        let completed: Vec<usize> = working_gates
            .iter()
            .enumerate()
            .filter_map(|(i, gate)| {
                if let Some(in1) = completed_gates.get(&gate.inputs.0)
                    && let Some(in2) = completed_gates.get(&gate.inputs.1)
                {
                    let val = gate.operation.apply(in1.to_owned(), in2.to_owned());
                    completed_gates.insert(gate.output.clone(), val);
                    Some(i)
                } else {
                    None
                }
            })
            .collect();
        completed.iter().rev().for_each(|i| {
            working_gates.remove(*i);
        });
    }

    let re = Regex::new(r"z[0-9]+").unwrap();
    let mut zkeys = completed_gates
        .keys()
        .filter(|key| re.is_match(key))
        .collect::<Vec<_>>();
    zkeys.sort();
    zkeys.reverse();

    let bin_string = zkeys
        .iter()
        .map(|k| match completed_gates.get(*k).unwrap() {
            true => "1",
            false => "0",
        })
        .collect::<Vec<_>>()
        .join("");

    u64::from_str_radix(&bin_string, 2).unwrap()
}

fn part2(input: String) {
    todo!()
}



#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    #[test]
    fn part1_small() {
        let input = fs::read_to_string("../../inputs/D24/small.txt").unwrap();
        let result = part1(input);
        assert_eq!(result, 2024)
    }

    #[test]
    fn init_parse() {
        let input = "x00: 1
x01: 0
x02: 1
y00: 1
y01: 1
y02: 1"
            .to_owned();
        let result = parse_init_inputs(input);
        assert_eq!(
            result,
            vec![
                InitValue("x00".to_string(), true),
                InitValue("x01".to_string(), false),
                InitValue("x02".to_string(), true),
                InitValue("y00".to_string(), true),
                InitValue("y01".to_string(), true),
                InitValue("y02".to_string(), true)
            ]
        )
    }

    #[test]
    fn gate_parse() {
        let input = "x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"
            .to_owned();
        let result = parse_gate_inputs(input);
        assert_eq!(
            result,
            vec![
                Gate {
                    inputs: ("x00".to_string(), "y00".to_string()),
                    operation: Operation::And,
                    output: "z00".to_string()
                },
                Gate {
                    inputs: ("x01".to_string(), "y01".to_string()),
                    operation: Operation::Xor,
                    output: "z01".to_string()
                },
                Gate {
                    inputs: ("x02".to_string(), "y02".to_string()),
                    operation: Operation::Or,
                    output: "z02".to_string()
                }
            ]
        )
    }
    #[test]
    fn part1_main() {
        let input = fs::read_to_string("../../inputs/D24/main.txt").unwrap();
        let result = part1(input);
        assert_eq!(result, 53258032898766)
    }
}
