use log::debug;
use simple_logger::SimpleLogger;
use std::cmp;

fn main() {
    SimpleLogger::new().init().unwrap();

    let contents = include_str!("../input");

    let total_points: usize = contents.lines().map(number_of_points).sum();

    println!("The total number of points is {total_points}");

    let mut count = [1; 214];
    for line in contents.lines().enumerate() {
        let n = number_of_matches(line.1);
        for i in line.0 + 1..cmp::min(line.0 + n + 1, 214) {
            count[i] += count[line.0];
        }
    }
    let total: usize = count.iter().sum();

    debug! {"{:#?}", count};

    println!("The total number of cards is {total}");
}

fn number_of_points(card: &str) -> usize {
    let mut points: usize = 0;
    let mut it = card[9..].split('|');
    let winning_numbers: Vec<_> = it.next().unwrap().split_ascii_whitespace().collect();
    let my_numbers = it.next().unwrap().split_ascii_whitespace();
    for n in my_numbers {
        if winning_numbers.contains(&n) {
            if points == 0 {
                points = 1
            } else {
                points *= 2;
            }
        }
    }
    debug!("{card}");
    debug!("{points}");
    points
}

fn number_of_matches(card: &str) -> usize {
    let mut matches: usize = 0;
    let mut it = card[9..].split('|');
    let winning_numbers: Vec<_> = it.next().unwrap().split_ascii_whitespace().collect();
    let my_numbers = it.next().unwrap().split_ascii_whitespace();
    for n in my_numbers {
        if winning_numbers.contains(&n) {
            matches += 1;
        }
    }
    matches
}
