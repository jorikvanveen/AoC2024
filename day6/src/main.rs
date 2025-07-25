use std::collections::HashSet;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum MapTile {
    Empty,
    Obstacle,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn forward(&self, (row_idx, col_idx): &(isize, isize)) -> (isize, isize) {
        match self {
            Direction::Up => (row_idx - 1, *col_idx),
            Direction::Down => (row_idx + 1, *col_idx),
            Direction::Left => (*row_idx, col_idx - 1),
            Direction::Right => (*row_idx, col_idx + 1),
        }
    }

    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

fn get_input() -> (Vec<Vec<MapTile>>, (isize, isize), Direction) {
    let mut direction = Direction::Up;
    let mut location: (isize, isize) = (0, 0);

    let map = std::fs::read_to_string("input")
        .unwrap()
        .trim()
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '.' => MapTile::Empty,
                    '#' => MapTile::Obstacle,
                    d => {
                        match d {
                            '^' => direction = Direction::Up,
                            'v' => direction = Direction::Down,
                            '<' => direction = Direction::Left,
                            '>' => direction = Direction::Right,
                            _ => panic!("Invalid input"),
                        };
                        location = (row as isize, col as isize);
                        MapTile::Empty
                    }
                })
                .collect::<Vec<MapTile>>()
        })
        .collect::<Vec<Vec<MapTile>>>();
    (map, location, direction)
}

fn get_visited() -> HashSet<(isize, isize)> {
    let (map, mut location, mut direction) = get_input();

    let mut visited_locations: HashSet<(isize, isize)> = HashSet::new();
    loop {
        visited_locations.insert(location);
        let next_location = direction.forward(&location);

        if next_location.0 >= map.len() as isize || next_location.1 >= map[0].len() as isize {
            break;
        }

        let next_tile = map
            .get(next_location.0 as usize)
            .and_then(|row| row.get(next_location.1 as usize))
            .unwrap(); // Validity of location checked earlier

        match next_tile {
            MapTile::Empty => location = next_location,
            MapTile::Obstacle => direction = direction.turn_right(),
        }
    }
    return visited_locations;
}

fn part1() -> usize {
    return get_visited().len();
}

fn part2() -> usize {
    let (map, location, direction) = get_input();

    get_visited()
        .par_iter()
        .filter(|obstacle_candidate| {
            let (mut map, mut location, mut direction) =
                (map.clone(), location.clone(), direction.clone());
            map[obstacle_candidate.0 as usize][obstacle_candidate.1 as usize] = MapTile::Obstacle;
            let mut visited_locations: HashSet<((isize, isize), Direction)> = HashSet::new();

            loop {
                if !visited_locations.insert((location, direction.clone())) {
                    break true;
                };
                let next_location = direction.forward(&location);

                if next_location.0 < 0
                    || next_location.1 < 0
                    || next_location.0 >= map.len() as isize
                    || next_location.1 >= map[0].len() as isize
                {
                    break false;
                }

                let next_tile = map
                    .get(next_location.0 as usize)
                    .and_then(|row| row.get(next_location.1 as usize))
                    .unwrap(); // Validity of location checked earlier

                match next_tile {
                    MapTile::Empty => location = next_location,
                    MapTile::Obstacle => direction = direction.turn_right(),
                }
            }
        })
        .count()
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
