use std::collections::HashMap;

fn get_input() -> (Vec<i32>, Vec<i32>) {
    let input = std::fs::read_to_string("input").unwrap();

    let input_pairs: Vec<(i32, i32)> = input
        .split("\n")
        .filter(|pair_str| *pair_str != "")
        .map(|pair_str| {
            let mut split = pair_str.split("   ");
            (
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let mut left_list: Vec<i32> = input_pairs.iter().map(|(num, _)| *num).collect();
    let mut right_list: Vec<i32> = input_pairs.iter().map(|(_, num)| *num).collect();

    left_list.sort();
    right_list.sort();

    (left_list, right_list)
}
fn part1() {
    let (left_list, right_list) = get_input();

    let sorted_pairs: Vec<(i32, i32)> = left_list.into_iter().zip(right_list.into_iter()).collect();

    let difference_sum: i32 = sorted_pairs
        .into_iter()
        .map(|(left, right)| (left - right).abs())
        .sum();

    println!("{}", difference_sum);
}

fn part2() {
    let (left, right) = get_input();

    let right_occur = get_occurrences(right);

    let similarity_score: i32 = left
        .into_iter()
        .map(|left_num| right_occur.get(&left_num).unwrap_or(&0) * left_num)
        .sum();

    println!("{}", similarity_score);
}

fn get_occurrences(list: Vec<i32>) -> HashMap<i32, i32> {
    list.into_iter().fold(HashMap::new(), |mut map, num| {
        match map.get(&num) {
            Some(existing_count) => map.insert(num, existing_count + 1),
            None => map.insert(num, 1),
        };
        map
    })
}

fn main() {
    part1();
    part2();
}
