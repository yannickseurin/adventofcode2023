// use fn_cache::{FnCache, HashCache};
use log::debug;
use simple_logger::SimpleLogger;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Record {
    // whether each spring is operational '.', damaged '#', or unknown '?'
    condition: String,
    // list of the size of each contiguous group of damaged springs
    pattern: Vec<usize>,
    // total number of missing damaged springs
    missing: usize,
    // position of springs of unknown condition
    unknown_positions: Vec<usize>,
    // total number of springs of unknown condition
    unknown: usize,
}

impl Record {
    fn new(condition_str: &str, pattern_slice: &[usize]) -> Record {
        let condition = condition_str.to_string();
        let pattern = pattern_slice.to_vec();
        let missing =
            pattern.iter().sum::<usize>() - condition.chars().filter(|c| *c == '#').count();
        let unknown_positions = condition
            .chars()
            .enumerate()
            .filter(|c| c.1 == '?')
            .map(|c| c.0)
            .collect::<Vec<_>>();
        let unknown = unknown_positions.len();
        Record {
            condition,
            pattern,
            missing,
            unknown_positions,
            unknown,
        }
    }
}

fn main() {
    SimpleLogger::new().init().unwrap();

    let contents = include_str!("../input");

    let records: Vec<Record> = contents
        .lines()
        .map(|l| {
            let mut it = l.split_whitespace();
            let condition_str = it.next().unwrap();
            let pattern_slice = it
                .next()
                .unwrap()
                .split(',')
                .map(|c| c.parse().unwrap())
                .collect::<Vec<_>>();
            Record::new(condition_str, &pattern_slice)
        })
        .collect();

    // debug!("Records: {:?}", records);

    let total: usize = records.iter().map(count_arrangements).sum();

    println!("The total number of arrangements is {total}");

    let mut cache = HashMap::new();

    let total2: usize = records
        .iter()
        .map(|r| count_arrangements_recursive(r, &mut cache))
        .sum();

    println!("The total number of arrangements by the second method is {total2}");

    // SECOND PART

    let long_records = records
        .iter()
        .map(|rec| {
            let c = &rec.condition;
            let g = &rec.pattern;
            let condition_str = format!("{c}?{c}?{c}?{c}?{c}");
            let pattern_slice = [g.clone(), g.clone(), g.clone(), g.clone(), g.clone()].concat();
            Record::new(&condition_str, &pattern_slice)
        })
        .collect::<Vec<_>>();

    debug!("Long records: {:?}", long_records);

    let mut cache = HashMap::new();

    let long_total: usize = long_records
        .iter()
        .map(|r| count_arrangements_recursive(r, &mut cache))
        .sum();

    println!("The total number of arrangements for long records is {long_total}");
}

fn count_arrangements(record: &Record) -> usize {
    // debug!{"Handling record: {:?}", record};

    let mut count = 0;

    for combi in combinations(&record.unknown_positions, record.missing) {
        let mut cond = record.condition.clone();
        for i in record.unknown_positions.iter() {
            if combi.contains(i) {
                cond.replace_range(i..&(i + 1), "#");
            } else {
                cond.replace_range(i..&(i + 1), ".");
            }
        }
        // debug!("Testing {:?}", cond);
        let pattern = cond
            .split('.')
            .map(|s| s.len())
            .filter(|n| *n != 0)
            .collect::<Vec<_>>();
        // debug!("Pattern: {:?}", pattern);
        if pattern == record.pattern {
            count += 1;
        }
    }
    count
}

// given a vector of v, returns all possible combinations of n elements of v
fn combinations<T: Copy>(v: &[T], n: usize) -> Vec<Vec<T>> {
    if n > v.len() {
        panic!("n is larger than the number of elements in v.");
    }
    if n == 0 {
        vec![vec![]]
    } else {
        let mut list: Vec<Vec<T>> = Vec::new();
        for i in 0..v.len() - n + 1 {
            let a = v[i];
            for c in combinations(&v[i + 1..], n - 1).iter_mut() {
                c.insert(0, a);
                list.push(c.to_vec());
            }
        }
        list
    }
}

fn count_arrangements_recursive(record: &Record, cache: &mut HashMap<Record, usize>) -> usize {
    debug!("Recursive record: {:?}", record);
    if cache.contains_key(record) {
        *cache.get(record).unwrap()
    } else if record.missing == 0 {
        // no more damaged springs to dispatch
        // replace remaining '?' with '.'
        let cond = record.condition.replace('?', ".");
        // compute the resulting pattern
        let pattern = cond
            .split('.')
            .map(|s| s.len())
            .filter(|n| *n != 0)
            .collect::<Vec<_>>();
        // debug!("Pattern: {:?}", pattern);
        if pattern == record.pattern {
            1
        } else {
            0
        }
    } else if record.missing == record.unknown {
        // as many missing damaged springs as unknown positions
        // replace remaining '?' with '#'
        let cond = record.condition.replace('?', "#");
        // compute the resulting pattern
        let pattern = cond
            .split('.')
            .map(|s| s.len())
            .filter(|n| *n != 0)
            .collect::<Vec<_>>();
        // debug!("Pattern: {:?}", pattern);
        if pattern == record.pattern {
            1
        } else {
            0
        }
    } else {
        // if record.missing > 0, one must have record.condition != ""
        match record.condition.chars().next().unwrap() {
            '.' => {
                // simply remove the '.' and count recursively
                let trimmed_record = Record::new(&record.condition[1..], &record.pattern);
                let count = count_arrangements_recursive(&trimmed_record, cache);
                cache.insert(record.clone(), count);
                count
            }
            '?' => {
                // count recursively assuming either '.' or '#' and add the results
                let record_dot = Record::new(&record.condition[1..], &record.pattern);
                let record_sharp =
                    Record::new(&("#".to_owned() + &record.condition[1..]), &record.pattern);
                let count = count_arrangements_recursive(&record_dot, cache)
                    + count_arrangements_recursive(&record_sharp, cache);
                cache.insert(record.clone(), count);
                count
            }
            '#' => {
                // must be '#'
                // count number of trailing '#'
                let mut n = 0;
                let mut it = record.condition.chars();
                while it.next().unwrap() == '#' {
                    n += 1;
                }
                match record.condition.chars().nth(n).unwrap() {
                    '.' => {
                        if n != record.pattern[0] {
                            // incompatible
                            0
                        } else {
                            let trimmed_record =
                                Record::new(&record.condition[n + 1..], &record.pattern[1..]);
                            let count = count_arrangements_recursive(&trimmed_record, cache);
                            cache.insert(record.clone(), count);
                            count
                        }
                    }
                    '?' => {
                        let mut condition_dot = record.condition.clone();
                        condition_dot.replace_range(n..n + 1, ".");
                        let record_dot = Record::new(&condition_dot, &record.pattern);
                        let mut condition_sharp = record.condition.clone();
                        condition_sharp.replace_range(n..n + 1, "#");
                        let record_sharp = Record::new(&condition_sharp, &record.pattern);
                        let count = count_arrangements_recursive(&record_dot, cache)
                            + count_arrangements_recursive(&record_sharp, cache);
                        cache.insert(record.clone(), count);
                        count
                    }
                    _ => {
                        panic!("Unexpected char");
                    }
                }
            }
            _ => {
                panic!("Unexpected char");
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn combinations_zero_of_n() {
        assert_eq!(combinations(&[1, 2, 3, 4], 0), [[]]);
    }

    #[test]
    fn combinations_one_of_n() {
        assert_eq!(combinations(&[1, 2, 3, 4], 1), [[1], [2], [3], [4]]);
    }

    #[test]
    fn combinations_n_of_n() {
        assert_eq!(combinations(&[1, 2, 3, 4], 4), [[1, 2, 3, 4]]);
    }

    #[test]
    fn combinations_k_of_n() {
        assert_eq!(
            combinations(&[1, 2, 3, 4], 3),
            [[1, 2, 3], [1, 2, 4], [1, 3, 4], [2, 3, 4]]
        );
    }
}
