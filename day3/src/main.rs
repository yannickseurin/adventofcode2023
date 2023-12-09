use regex::Regex;

fn main() {
    let contents = include_str!("../input");

    let n_lines = contents.lines().count();
    let n_rows = contents.lines().next().unwrap().len();

    let mut is_special_char = vec![vec![false; n_rows]; n_lines];

    for line in contents.lines().enumerate() {
        for c in line.1.chars().enumerate() {
            if !c.1.is_ascii_digit() && c.1 != '.' {
                is_special_char[line.0][c.0] = true;
            }
        }
    }

    let re = Regex::new(r"[0-9]+").unwrap();
    let mut sum: usize = 0;
    for line in contents.lines().enumerate() {
        for cap in re.captures_iter(line.1) {
            let m = cap.get(0).unwrap();
            let i = m.start();
            let j = m.end();
            let num = m.as_str();
            if is_part_number(line.0, i, j, &is_special_char, n_lines, n_rows) {
                sum += num.parse::<usize>().unwrap();
            }
        }
    }

    println!("The sum of part numbers is {sum}");

    let mut gear_tab = vec![vec![(false, 0, 1); n_rows]; n_lines];

    for line in contents.lines().enumerate() {
        for c in line.1.chars().enumerate() {
            if c.1 == '*' {
                gear_tab[line.0][c.0].0 = true;
            }
        }
    }

    for line in contents.lines().enumerate() {
        for cap in re.captures_iter(line.1) {
            let m = cap.get(0).unwrap();
            let i = m.start();
            let j = m.end();
            let num = m.as_str().parse().unwrap();
            update_gear_tab(line.0, i, j, num, &mut gear_tab, n_lines, n_rows)
        }
    }

    let gear_sum: usize = gear_tab
        .iter()
        .flat_map(|l| l.iter())
        .filter(|cell| cell.1 == 2)
        .map(|cell| cell.2)
        .sum();

    println!("The sum of gear numbers is {gear_sum}");
}

/// Takes the position of a number specified by the line index, the row start index, and the row end index
/// and a tab of booleans of size n_lines times n_rows indicating where special chars are locates
/// and returns true if the number is adjacent to a special char.
fn is_part_number(
    l: usize,
    i: usize,
    j: usize,
    tab: &[Vec<bool>],
    n_lines: usize,
    n_rows: usize,
) -> bool {
    // check previous line
    if l > 0 {
        if i > 0 && tab[l - 1][i - 1] {
            return true;
        }
        for k in i..j {
            if tab[l - 1][k] {
                return true;
            }
        }
        if j < n_rows && tab[l - 1][j] {
            return true;
        }
    }

    // check current line
    if i > 0 && tab[l][i - 1] {
        return true;
    }
    if j < n_rows && tab[l][j] {
        return true;
    }

    // check following line
    if l < n_lines - 1 {
        if i > 0 && tab[l + 1][i - 1] {
            return true;
        }
        for k in i..j {
            if tab[l + 1][k] {
                return true;
            }
        }
        if j < n_rows && tab[l + 1][j] {
            return true;
        }
    }

    false
}

fn update_gear_tab(
    l: usize,
    i: usize,
    j: usize,
    num: usize,
    tab: &mut [Vec<(bool, usize, usize)>],
    n_lines: usize,
    n_rows: usize,
) {
    // update previous line
    if l > 0 {
        if i > 0 {
            update_cell(&mut tab[l - 1][i - 1], num);
        }
        for k in i..j {
            update_cell(&mut tab[l - 1][k], num);
        }
        if j < n_rows {
            update_cell(&mut tab[l - 1][j], num);
        }
    }

    // update current line
    if i > 0 {
        update_cell(&mut tab[l][i - 1], num);
    }
    if j < n_rows {
        update_cell(&mut tab[l][j], num);
    }

    // update following line
    if l < n_lines - 1 {
        if i > 0 {
            update_cell(&mut tab[l + 1][i - 1], num);
        }
        for k in i..j {
            update_cell(&mut tab[l + 1][k], num);
        }
        if j < n_rows {
            update_cell(&mut tab[l + 1][j], num);
        }
    }
}

fn update_cell(cell: &mut (bool, usize, usize), num: usize) {
    if cell.0 {
        cell.1 += 1;
        cell.2 *= num;
    }
}
