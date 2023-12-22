fn main() {
    let contents = include_str!("../input");

    let sum: isize = contents
        .lines()
        .map(|l| {
            compute_next_value(
                &l.split_ascii_whitespace()
                    .map(|s| s.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>(),
            )
        })
        .sum();

    println!("The sum of next values is {sum}");

    let sum2: isize = contents
        .lines()
        .map(|l| {
            compute_prev_value(
                &l.split_ascii_whitespace()
                    .map(|s| s.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>(),
            )
        })
        .sum();

    println!("The sum of previous values is {sum2}");
}

fn compute_next_value(v: &[isize]) -> isize {
    let mut w = v.to_owned();
    // vector to collect integers to add
    let mut to_add: Vec<isize> = Vec::new();
    while !is_all_zero(&w) {
        let mut last = w.pop().unwrap();
        to_add.push(last);
        let len = w.len();
        for i in (0..len).rev() {
            let tmp = w[i];
            w[i] = last - tmp;
            last = tmp;
        }
    }
    to_add.iter().sum()
}

fn compute_prev_value(v: &[isize]) -> isize {
    let mut w = v.to_owned();
    // vector to collect integers to add
    let mut first_vals: Vec<isize> = Vec::new();
    while !is_all_zero(&w) {
        first_vals.push(*w.first().unwrap());
        let len = w.len();
        for i in 0..len - 1 {
            w[i] = w[i + 1] - w[i];
        }
        let _ = w.pop();
    }
    first_vals
        .iter()
        .enumerate()
        .map(|(i, x)| isize::pow(-1, i.try_into().unwrap()) * x)
        .sum()
}

fn is_all_zero(v: &[isize]) -> bool {
    for i in v.iter() {
        if *i != 0 {
            return false;
        }
    }
    true
}
