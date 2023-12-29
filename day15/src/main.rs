use log::debug;
use simple_logger::SimpleLogger;
use std::collections::HashMap;

fn main() {
    SimpleLogger::new().init().unwrap();

    let contents = include_str!("../input");

    let sum: usize = contents.split(',').map(hash).sum();

    println!("The sum of hashes is {sum}");

    // SECOND PART

    let mut hm: HashMap<usize, Vec<(String, usize)>> = HashMap::new();

    for s in contents.split(',') {
        handle_step(s, &mut hm);
    }

    let mut new_sum: usize = 0;

    for i in 0..256 {
        if hm.contains_key(&i) {
            let lens_box = hm.get(&i).unwrap();
            new_sum += (i + 1)
                * lens_box
                    .iter()
                    .enumerate()
                    .map(|lens| (lens.0 + 1) * lens.1 .1)
                    .sum::<usize>();
        }
    }

    println!("The new sum is {new_sum}");
}

fn hash(s: &str) -> usize {
    let mut h = 0;
    for c in s.chars() {
        if c != '\n' {
            h = (h + (c as usize)) * 17 % 256;
        }
    }
    h
}

fn handle_step(s: &str, hm: &mut HashMap<usize, Vec<(String, usize)>>) {
    let s = s.trim_end_matches('\n');
    debug!("step: {}", s);
    if !s.ends_with('-') {
        // '=' step
        let label = s.split('=').next().unwrap();
        let h = hash(label);
        let focal: usize = s
            .chars()
            .last()
            .unwrap()
            .to_digit(10)
            .unwrap()
            .try_into()
            .unwrap();
        let lens_box = hm.entry(h).or_default();
        debug!("box before: {:?}", lens_box);
        let mut present = false;
        for lens in lens_box.iter_mut() {
            if lens.0 == label {
                present = true;
                *lens = (label.to_string(), focal);
                break;
            }
        }
        if !present {
            lens_box.push((label.to_string(), focal));
        }
        debug!("box after: {:?}", lens_box);
    } else {
        // '-' step
        let label = s.split('-').next().unwrap();
        let h = hash(label);
        if hm.contains_key(&h) {
            let lens_box = hm.get_mut(&h).unwrap();
            debug!("box before: {:?}", lens_box);
            for i in 0..lens_box.len() {
                if lens_box[i].0 == label {
                    lens_box.remove(i);
                    break;
                }
            }
            debug!("box after: {:?}", lens_box);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn simple_test() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn less_simple_test() {
        let input = include_str!("../test");
        assert_eq!(input.split(',').map(hash).sum::<usize>(), 1320);
    }
}
