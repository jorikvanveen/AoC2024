use std::fs::read_to_string;

fn get_input() -> Vec<(usize, Vec<usize>)> {
    read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| {
            let mut split = line.split(": ");
            let answer: usize = split.next().unwrap().parse().unwrap();
            let numbers: Vec<usize> = split
                .next()
                .unwrap()
                .split(" ")
                .map(|number| number.parse().unwrap())
                .collect();
            (answer, numbers)
        })
        .collect()
}

mod part1 {
    use super::*;

    #[derive(Debug, Clone)]
    enum Operator {
        Add,
        Multiply,
    }

    fn generate_operator_sequences(n: usize) -> Vec<Vec<Operator>> {
        if n < 1 {
            panic!("Invalid sequence length")
        }
        if n == 1 {
            return vec![vec![Operator::Add], vec![Operator::Multiply]];
        }
        let smaller_sequences = generate_operator_sequences(n - 1);

        let add_sequences: Vec<Vec<Operator>> = smaller_sequences
            .iter()
            .map(|sequence| {
                let mut sequence = sequence.clone();
                sequence.insert(0, Operator::Add);
                sequence
            })
            .collect();

        let mult_sequences: Vec<Vec<Operator>> = smaller_sequences
            .iter()
            .map(|sequence| {
                let mut sequence = sequence.clone();
                sequence.insert(0, Operator::Multiply);
                sequence
            })
            .collect();

        let mut sequences = add_sequences;
        sequences.extend(mult_sequences.into_iter());
        sequences
    }

    pub fn part1() -> usize {
        let input = get_input();
        input
            .into_iter()
            .filter_map(|(test_value, numbers)| {
                let op_sequences = generate_operator_sequences(numbers.len() - 1);
                for sequence in op_sequences {
                    let mut result = numbers[0];
                    for i in 0..numbers.len() - 1 {
                        match sequence[i] {
                            Operator::Add => result += numbers[i + 1],
                            Operator::Multiply => result *= numbers[i + 1],
                        }
                    }

                    if result == test_value {
                        return Some(test_value);
                    }
                }
                return None;
            })
            .sum::<usize>()
    }
}

mod part2 {
    use rayon::iter::{IntoParallelIterator, ParallelIterator};

    use super::*;

    #[derive(Debug, Clone)]
    enum Operator {
        Add,
        Multiply,
        Concatenate,
    }

    fn generate_operator_sequences(n: usize) -> Vec<Vec<Operator>> {
        if n < 1 {
            panic!("Invalid sequence length")
        }
        if n == 1 {
            return vec![
                vec![Operator::Add],
                vec![Operator::Multiply],
                vec![Operator::Concatenate],
            ];
        }
        let smaller_sequences = generate_operator_sequences(n - 1);

        vec![Operator::Add, Operator::Multiply, Operator::Concatenate]
            .into_iter()
            .flat_map(|op| {
                smaller_sequences.iter().map(move |sequence| {
                    let mut sequence = sequence.clone();
                    sequence.insert(0, op.clone());
                    sequence
                })
            })
            .collect()
    }

    pub fn part2() -> usize {
        let input = get_input();
        input
            .into_par_iter()
            .filter_map(|(test_value, numbers)| {
                let op_sequences = generate_operator_sequences(numbers.len() - 1);
                for sequence in op_sequences {
                    let mut result = numbers[0];
                    for i in 0..numbers.len() - 1 {
                        match sequence[i] {
                            Operator::Add => result += numbers[i + 1],
                            Operator::Multiply => result *= numbers[i + 1],
                            Operator::Concatenate => {
                                result =
                                    format!("{}{}", result.to_string(), numbers[i + 1].to_string())
                                        .parse()
                                        .unwrap()
                            }
                        }
                    }

                    if result == test_value {
                        return Some(test_value);
                    }
                }
                return None;
            })
            .sum::<usize>()
    }
}

fn main() {
    println!("Part 1: {}", part1::part1());
    println!("Part 2: {}", part2::part2());
}
