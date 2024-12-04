fn get_input() -> (Vec<String>, Vec<String>, Vec<String>, Vec<String>) {
    let raw = std::fs::read_to_string("input").unwrap();
    let rows: Vec<String> = raw
        .split("\n")
        .filter(|r| r.trim() != "")
        .map(Into::into)
        .collect();

    let mut columns: Vec<String> = vec![];

    for col_idx in 0..rows[0].len() {
        columns.push("".into());
        for row in rows.iter() {
            columns
                .last_mut()
                .unwrap()
                .push(row.chars().nth(col_idx).unwrap());
        }
    }

    let lr_diagonals = get_diagonals(&rows);
    let rl_diagonals = get_diagonals(
        &rows
            .iter()
            .map(|row| row.chars().rev().collect::<String>())
            .collect(),
    );

    (rows, columns, lr_diagonals, rl_diagonals)
}

fn get_diagonals(rows: &Vec<String>) -> Vec<String> {
    let mut diagonals: Vec<String> = vec![];

    for col_start_idx in 0..rows[0].len() {
        let row_start_idx = 0;
        diagonals.push(get_diagonal_from_start(rows, row_start_idx, col_start_idx));
    }

    for row_start_idx in 1..rows.len() {
        let col_start_idx = 0;
        diagonals.push(get_diagonal_from_start(rows, row_start_idx, col_start_idx));
    }

    diagonals
}

fn get_diagonal_from_start(rows: &Vec<String>, row_start: usize, col_start: usize) -> String {
    let mut col_idx = col_start;
    let mut row_idx = row_start;
    let mut diagonal: String = "".into();

    loop {
        let value = match rows.get(row_idx).and_then(|row| row.chars().nth(col_idx)) {
            Some(v) => v,
            None => break,
        };
        diagonal.push(value);
        col_idx += 1;
        row_idx += 1;
    }

    diagonal
}

fn part1() {
    let (rows, columns, lr_diagonals, rl_diagonals) = get_input();

    let row_count: usize = rows.iter().map(|row| count_xmas(&row)).sum();
    let col_count: usize = columns.iter().map(|col| count_xmas(&col)).sum();
    let rl_diag_count: usize = rl_diagonals.iter().map(|diag| count_xmas(&diag)).sum();
    let lr_diag_count: usize = lr_diagonals.iter().map(|diag| count_xmas(&diag)).sum();

    let total_count = row_count + col_count + rl_diag_count + lr_diag_count;

    println!("{}", total_count);
}

fn part2() {
    let (rows, ..) = get_input();

    let mut xmas_count = 0;

    for i in 1..rows.len() - 1 {
        for j in 1..rows[0].len() - 1 {
            let current = rows[i].chars().nth(j).unwrap();

            if current != 'A' {
                continue;
            }

            let left_up = rows[i - 1].chars().nth(j - 1).unwrap();
            let left_down = rows[i + 1].chars().nth(j - 1).unwrap();
            let right_up = rows[i - 1].chars().nth(j + 1).unwrap();
            let right_down = rows[i + 1].chars().nth(j + 1).unwrap();

            let diag_a = (left_up, right_down);
            let diag_b = (left_down, right_up);

            if (diag_a == ('M', 'S') || diag_a == ('S', 'M'))
                && (diag_b == ('M', 'S') || diag_b == ('S', 'M'))
            {
                xmas_count += 1;
            }
        }
    }

    println!("{}", xmas_count);
}

fn count_xmas(slice: &str) -> usize {
    let count = slice.matches("XMAS").count();
    let reversed_count = slice
        .chars()
        .rev()
        .collect::<String>()
        .matches("XMAS")
        .count();
    count + reversed_count
}

fn main() {
    part1();
    part2();
}
