use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

enum Tile {
    Empty,
    Frequency(char),
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct Vec2(pub isize, pub isize);
impl Vec2 {
    pub fn is_valid(&self, dimensions: (usize, usize)) -> bool {
        self.0 >= 0
            && self.1 >= 0
            && self.0 <= dimensions.0 as isize
            && self.1 <= dimensions.1 as isize
    }
}
impl Add for &Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Sub for &Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

fn get_input() -> Vec<Vec<Tile>> {
    std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    freq => Tile::Frequency(freq),
                })
                .collect()
        })
        .collect()
}

fn get_frequency_locations() -> HashMap<char, HashSet<Vec2>> {
    let map = get_input();
    let mut frequency_locations: HashMap<char, HashSet<Vec2>> = HashMap::new();

    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, tile) in row.iter().enumerate() {
            match tile {
                Tile::Empty => continue,
                Tile::Frequency(f) => match frequency_locations.get_mut(f) {
                    Some(set) => {
                        set.insert(Vec2(row_idx as isize, col_idx as isize));
                    }
                    None => {
                        frequency_locations.insert(
                            *f,
                            HashSet::from([Vec2(row_idx as isize, col_idx as isize)]),
                        );
                    }
                },
            }
        }
    }

    frequency_locations
}

fn part1() -> usize {
    let map = get_input();
    let dimensions = (map.len() - 1, map[0].len() - 1);
    let frequency_locations = get_frequency_locations();

    let mut antinode_locations: HashSet<Vec2> = HashSet::new();

    for locations in frequency_locations.values() {
        for loc_a in locations {
            for loc_b in locations {
                if loc_a == loc_b {
                    continue;
                }

                let diff = loc_b - loc_a;
                let anti_a = loc_a - &diff;
                let anti_b = loc_b + &diff;

                if anti_a.is_valid(dimensions) {
                    antinode_locations.insert(anti_a);
                }

                if anti_b.is_valid(dimensions) {
                    antinode_locations.insert(anti_b);
                }
            }
        }
    }

    antinode_locations.len()
}

fn part2() -> usize {
    let map = get_input();
    let dimensions = (map.len() - 1, map[0].len() - 1);
    let frequency_locations = get_frequency_locations();

    let mut antinode_locations: HashSet<Vec2> = HashSet::new();

    for locations in frequency_locations.values() {
        for loc_a in locations {
            for loc_b in locations {
                if loc_a == loc_b {
                    continue;
                }

                let diff = loc_b - loc_a;
                let mut anti_a = loc_a.clone();
                loop {
                    if !anti_a.is_valid(dimensions) {
                        break;
                    }

                    antinode_locations.insert(anti_a.clone());
                    anti_a = &anti_a - &diff;
                }
                let mut anti_b = loc_b.clone();
                loop {
                    if !anti_b.is_valid(dimensions) {
                        break;
                    }

                    antinode_locations.insert(anti_b.clone());
                    anti_b = &anti_b + &diff;
                }
            }
        }
    }

    antinode_locations.len()
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
