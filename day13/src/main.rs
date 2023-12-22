use log::debug;
use simple_logger::SimpleLogger;
use std::cmp::min;

type Pattern = Vec<Vec<char>>;

fn main() {
    SimpleLogger::new().init().unwrap();

    let contents = include_str!("../input");

    let patterns: Vec<Pattern> = contents
        .split("\n\n")
        .map(|s| s.lines().map(|l| l.chars().collect()).collect())
        .collect();

    let total: usize = patterns.iter().map(|p| summarize(p, 0, 0)).sum();

    println!("The total when summarizing all notes is {total}");

    let mut total2 = 0;

    for pattern in patterns {
        let prev_s = summarize(&pattern, 0, 0);
        // compute the vertical/horizontal reflection to avoid when correcting patterns
        let bad_col = prev_s % 100;
        let bad_row = prev_s / 100;
        'search_loop: for i in 0..pattern.len() {
            for j in 0..pattern[0].len() {
                let mut new_pattern = pattern.clone();
                if new_pattern[i][j] == '.' {
                    new_pattern[i][j] = '#';
                } else {
                    new_pattern[i][j] = '.';
                }
                let s = summarize(&new_pattern, bad_col, bad_row);
                if s != 0 {
                    total2 += s;
                    break 'search_loop;
                }
            }
        }
    }

    println!("The total when summarizing all corrected notes is {total2}");
}

fn summarize(pattern: &Pattern, bad_col: usize, bad_row: usize) -> usize {
    debug!("Pattern: {:?}", pattern);

    let mut sum = 0;

    for i in 1..pattern[0].len() {
        let mut reflection = true;
        for j in 0..min(i, pattern[0].len() - i) {
            if get_column(pattern, i - 1 - j) != get_column(pattern, i + j) {
                reflection = false;
            }
        }
        if reflection && i != bad_col {
            debug!("Column reflection = {i}");
            sum += i;
            break;
        }
    }

    for i in 1..pattern.len() {
        let mut reflection = true;
        for j in 0..min(i, pattern.len() - i) {
            if pattern[i - 1 - j] != pattern[i + j] {
                reflection = false;
            }
        }
        if reflection && i != bad_row {
            debug!("Row reflection = {i}");
            sum += 100 * i;
            break;
        }
    }
    sum
}

fn get_column(pattern: &Pattern, i: usize) -> Vec<char> {
    pattern.iter().map(|row| *row.get(i).unwrap()).collect()
}
