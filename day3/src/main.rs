use std::collections::VecDeque;

use regex::Regex;

fn get_input() -> String {
    std::fs::read_to_string("input").unwrap()
}

fn part1() {
    let input = get_input();

    let regex = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();

    let sum: i32 = regex
        .captures_iter(&input)
        .map(|capture| capture.extract())
        .map(|(_, [fac1, fac2])| (fac1.parse::<i32>().unwrap(), fac2.parse::<i32>().unwrap()))
        .map(|(fac1, fac2)| fac1 * fac2)
        .sum();

    println!("{}", sum);
}

fn part2() {
    let input = get_input();

    let mut in_do_section = true;
    let mut i = 0;
    let mut filtered_input = String::from("");
    let mut prev_cache: VecDeque<char> = VecDeque::new();
    prev_cache.reserve(7);

    while i < input.len() {
        let char = input.chars().nth(i).unwrap();

        if in_do_section {
            filtered_input.push(char);
        }

        prev_cache.push_back(char);

        if prev_cache.len() > 7 {
            prev_cache.pop_front();
        }

        let cache_str: String = prev_cache.range(0..).collect();
        if cache_str == "don't()" {
            in_do_section = false;
        }

        if cache_str.get(3..7) == Some("do()") {
            in_do_section = true;
        }

        //if char == 'd' {
        //    let next_7 = input.get(i..(i+7));
        //    let next_4 = input.get(i..(i+4));
        //    
        //    if next_7 == Some("don't()") {
        //        in_do_section = false;
        //    }

        //    if next_4 == Some("do()") {
        //        in_do_section = true;
        //    }
        //}
        i += 1;
    }

    let regex = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();
    let sum: i32 = regex
        .captures_iter(&filtered_input)
        .map(|capture| capture.extract())
        .map(|(_, [fac1, fac2])| (fac1.parse::<i32>().unwrap(), fac2.parse::<i32>().unwrap()))
        .map(|(fac1, fac2)| fac1 * fac2)
        .sum();

    println!("{}", sum);

}

fn main() {
    part1();
    part2()
}