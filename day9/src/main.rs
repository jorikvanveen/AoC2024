#[derive(Debug, Clone)]
enum Sector {
    Empty,
    Block(usize),
}

fn get_input() -> (Vec<Sector>, usize) {
    let input = std::fs::read_to_string("input").unwrap();
    let input = input.trim();
    let highest_id = input.len() / 2;
    let drive = input
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            let c: u32 = c.to_digit(10).unwrap();
            if i % 2 == 0 {
                (0..c).map(|_| Sector::Block(i / 2)).collect::<Vec<_>>()
            } else {
                (0..c).map(|_| Sector::Empty).collect::<Vec<_>>()
            }
        })
        .collect();
    (drive, highest_id)
}

fn part1() -> usize {
    let (mut drive, _) = get_input();

    let mut first = 0;
    loop {
        match drive.get(first) {
            Some(Sector::Empty) => {}
            Some(_) => {
                first += 1;
                continue;
            }
            None => break,
        }

        match drive.last().unwrap() {
            Sector::Empty => {
                drive.pop().unwrap();
                continue;
            }
            Sector::Block(id) => {
                drive[first] = Sector::Block(*id);
                drive.pop().unwrap();
                first += 1;
            }
        }
    }
    drive
        .iter()
        .enumerate()
        .map(|(i, block)| match block {
            Sector::Empty => 0,
            Sector::Block(id) => i * id,
        })
        .sum()
}

fn part2() -> usize {
    let (mut drive, highest_id) = get_input();

    let mut current_id = highest_id;
    let mut file_end_idx = if let Some(Sector::Block(_)) = drive.last() {
        drive.len() - 1
    } else {
        0
    };
    let mut file_start_idx = 0;
    for i in (0..drive.len()).rev() {
        if i == 0 {
            continue;
        }
        let (block_left, block) = (drive[i - 1].clone(), drive[i].clone());
        match (block_left, block) {
            (Sector::Empty, Sector::Block(id)) => {
                if id != current_id {
                    continue;
                }
                file_start_idx = i;
                move_fragment(&mut drive, file_end_idx, file_start_idx);
            }
            (Sector::Block(id), Sector::Empty) => {
                if id > current_id {
                    continue;
                }
                file_end_idx = i - 1;
                current_id = id;
            }
            (Sector::Block(id1), Sector::Block(id2)) => {
                if id1 == id2 {
                    continue;
                };
                if id2 == current_id {
                    file_start_idx = i;
                    move_fragment(&mut drive, file_end_idx, file_start_idx);
                }
                if id1 < current_id {
                    file_end_idx = i - 1;
                    current_id = id1;
                }
            }
            _ => continue,
        };
    }
    drive
        .iter()
        .enumerate()
        .map(|(i, block)| match block {
            Sector::Empty => 0,
            Sector::Block(id) => i * id,
        })
        .sum()
}

fn move_fragment(drive: &mut Vec<Sector>, file_end: usize, file_start: usize) {
    // Linear search for large enough block
    let required_space = file_end - file_start + 1;
    let mut empty_start = 0;
    let mut empty_end = 0;
    let mut in_empty_block = false;
    let mut did_find = false;
    for j in 0..drive.len() {
        if j >= file_start {
            break;
        }
        let mut do_check_length = true;
        match (&drive[j], in_empty_block) {
            (Sector::Empty, true) => {
                empty_end = j;
            }
            (Sector::Empty, false) => {
                empty_start = j;
                empty_end = j;
                in_empty_block = true;
            }
            (Sector::Block(_), true) => {
                in_empty_block = false;
            }
            (Sector::Block(_), false) => {
                do_check_length = false;
            }
        };

        if do_check_length && empty_end - empty_start + 1 == required_space {
            did_find = true;
            break;
        }
    }

    if did_find {
        // Perform the swap
        for i in 0..required_space {
            //dbg!(&drive[empty_start + i..file_start_idx + i]);
            drive.swap(empty_start + i, file_start + i);
        }
    }
}
fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
