use std::collections::HashSet;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn r#move(&self, start: (isize, isize)) -> (isize, isize) {
        let (d_0, d_1) = match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };

        (start.0 + d_0, start.1 + d_1)
    }
}
fn get_input() -> Vec<Vec<usize>> {
    std::fs::read_to_string("input")
        .unwrap()
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn part1() -> usize {
    let map = get_input();

    fn find_tops(
        map: &Vec<Vec<usize>>,
        pos: (isize, isize),
        value: usize,
    ) -> HashSet<(isize, isize)> {
        if pos.0 < 0 || pos.1 < 0 || pos.0 >= map.len() as isize || pos.1 >= map[0].len() as isize {
            return Default::default();
        }

        if map[pos.0 as usize][pos.1 as usize] != value {
            return Default::default();
        }

        if value == 9 {
            return HashSet::from([pos]);
        }

        vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .into_iter()
        .flat_map(|direction| find_tops(map, direction.r#move(pos), value + 1))
        .collect()
    }

    map.iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .map(|(col_idx, tile)| {
                    if *tile == 0 {
                        find_tops(&map, (row_idx as isize, col_idx as isize), 0).len()
                    } else {
                        Default::default()
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn part2() -> usize {
    let map = get_input();

    fn find_tops(map: &Vec<Vec<usize>>, pos: (isize, isize), value: usize) -> usize {
        if pos.0 < 0 || pos.1 < 0 || pos.0 >= map.len() as isize || pos.1 >= map[0].len() as isize {
            return 0;
        }

        if map[pos.0 as usize][pos.1 as usize] != value {
            return 0;
        }

        if value == 9 {
            return 1;
        }

        vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .into_iter()
        .map(|direction| find_tops(map, direction.r#move(pos), value + 1))
        .sum()
    }

    map.iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .map(|(col_idx, tile)| {
                    if *tile == 0 {
                        find_tops(&map, (row_idx as isize, col_idx as isize), 0)
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
