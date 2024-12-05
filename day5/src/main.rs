use std::collections::{HashMap, HashSet};

fn get_input() -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let input = std::fs::read_to_string("input").unwrap();
    let mut split = input.split("\n\n");
    let rules_input = split.next().unwrap();
    let orderings_input = split.next().unwrap();

    let rules: Vec<(usize, usize)> = rules_input
        .split("\n")
        .map(|rule_line| {
            let mut split = rule_line.split("|");
            let first: usize = split.next().unwrap().parse().unwrap();
            let second: usize = split.next().unwrap().parse().unwrap();
            (first, second)
        })
        .collect();

    let orderings: Vec<Vec<usize>> = orderings_input
        .split("\n")
        .filter(|l| l.trim() != "")
        .map(|ordering_line| {
            ordering_line
                .split(",")
                .map(|elem| elem.parse().unwrap())
                .collect()
        })
        .collect();

    (rules, orderings)
}

fn part1() {
    let (rules, orderings) = get_input();

    let after_map: HashMap<usize, HashSet<usize>> =
        rules
            .iter()
            .fold(HashMap::new(), |mut map, (before, after)| {
                match map.get_mut(before) {
                    Some(afters) => {
                        afters.insert(*after);
                    }
                    None => {
                        map.insert(*before, HashSet::from([*after]));
                    }
                }
                map
            });

    let correctly_ordered: Vec<Vec<usize>> = orderings
        .iter()
        .filter(|ordering| {
            let mut seen: HashSet<usize> = HashSet::new();
            for i in 0..ordering.len() {
                let afters = match after_map.get(&ordering[i]) {
                    Some(afters) => afters,
                    None => {
                        seen.insert(ordering[i]);
                        continue;
                    }
                };

                if afters.intersection(&seen).count() != 0 {
                    return false;
                }

                seen.insert(ordering[i]);
            }
            true
        })
        .cloned()
        .collect();

    let middles_sum: usize = correctly_ordered
        .iter()
        .map(|ordering| ordering[ordering.len() / 2])
        .sum();

    dbg!(middles_sum);
}

fn part2() {
    let (rules, orderings) = get_input();

    let after_map: HashMap<usize, HashSet<usize>> =
        rules
            .iter()
            .fold(HashMap::new(), |mut map, (before, after)| {
                match map.get_mut(before) {
                    Some(afters) => {
                        afters.insert(*after);
                    }
                    None => {
                        map.insert(*before, HashSet::from([*after]));
                    }
                }
                map
            });

    let incorrectly_ordered: Vec<Vec<usize>> = orderings
        .iter()
        .filter(|ordering| {
            let mut seen: HashSet<usize> = HashSet::new();
            for i in 0..ordering.len() {
                let afters = match after_map.get(&ordering[i]) {
                    Some(afters) => afters,
                    None => {
                        seen.insert(ordering[i]);
                        continue;
                    }
                };

                if afters.intersection(&seen).count() != 0 {
                    return true;
                }

                seen.insert(ordering[i]);
            }
            false
        })
        .cloned()
        .collect();

    let correctly_ordered: Vec<Vec<usize>> = incorrectly_ordered
        .into_iter()
        .map(|mut ordering| {
            // Basically bubble sort
            loop {
                let mut seen: HashSet<usize> = HashSet::new();
                let mut i_toswap: usize = 0;
                let mut should_swap = false;
                for i in 0..ordering.len() {
                    let afters = match after_map.get(&ordering[i]) {
                        Some(afters) => afters,
                        None => {
                            seen.insert(ordering[i]);
                            continue;
                        }
                    };

                    if afters.intersection(&seen).count() != 0 {
                        i_toswap = i;
                        should_swap = true;
                        break;
                    }

                    seen.insert(ordering[i]);
                }

                if !should_swap {
                    break;
                }

                ordering.swap(i_toswap, i_toswap - 1);
            }
            ordering
        })
        .collect();

    let middles_sum: usize = correctly_ordered
        .iter()
        .map(|ordering| ordering[ordering.len() / 2])
        .sum();
    dbg!(middles_sum);
}

fn main() {
    part1();
    part2();
}
