use fancy_regex::Regex;

fn main() {
    let contents = include_str!("../input");

    let re1 = r"(?<digit>[1-9])";
    let calib_sum1: i32 = contents.lines().map(|w| compute_calibration(re1, w)).sum();
    println!("The first sum of calibrations is {calib_sum1}");

    let re2 = r"(?=(?<digit>[1-9]|one|two|three|four|five|six|seven|eight|nine))";
    let calib_sum2: i32 = contents.lines().map(|w| compute_calibration(re2, w)).sum();
    println!("The second sum of calibrations is {calib_sum2}");
}

fn compute_calibration(re_str: &str, word: &str) -> i32 {
    let re = Regex::new(re_str).unwrap();
    let mut matches = re.captures_iter(word);
    let mut calib = String::new();

    // check first math
    calib.push_str(match matches.next() {
        None => return 0,
        Some(cap) => match cap.unwrap().name("digit").unwrap().as_str() {
            "1" | "one" => "1",
            "2" | "two" => "2",
            "3" | "three" => "3",
            "4" | "four" => "4",
            "5" | "five" => "5",
            "6" | "six" => "6",
            "7" | "seven" => "7",
            "8" | "eight" => "8",
            "9" | "nine" => "9",
            _ => panic!("Unexpected first match."),
        },
    });

    // check last match
    let first_digit = calib.clone();
    calib.push_str(match matches.last() {
        None => &first_digit,
        Some(cap) => match cap.unwrap().name("digit").unwrap().as_str() {
            "1" | "one" => "1",
            "2" | "two" => "2",
            "3" | "three" => "3",
            "4" | "four" => "4",
            "5" | "five" => "5",
            "6" | "six" => "6",
            "7" | "seven" => "7",
            "8" | "eight" => "8",
            "9" | "nine" => "9",
            _ => panic!("Unexpected last match."),
        },
    });

    calib.parse().unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let re1 = r"(?<digit>[1-9])";
        let c = compute_calibration(re1, "2zzdsijdsij1");
        assert_eq!(c, 21);
    }
}
