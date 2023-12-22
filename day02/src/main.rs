use log::debug;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();

    let contents = include_str!("../input");

    let sum: usize = contents
        .lines()
        .enumerate()
        .filter(|line| is_game_ok(line.1))
        .map(|line| line.0 + 1)
        .sum();

    let power_sum: i32 = contents.lines().map(power).sum();

    println!("The sum of lines of possible games is {sum}");
    println!("The sum of powers is {power_sum}");
}

fn is_game_ok(game: &str) -> bool {
    debug!("=== FUNCTION is_game_ok ===");
    debug!("{game}");
    let mut game_ok = true;
    let i = game.find(':').unwrap();
    for draw in game[i + 2..].split("; ") {
        debug!("{draw}");
        for balls in draw.split(", ") {
            debug!("{balls}");
            let mut it = balls.split_ascii_whitespace();
            let n: i32 = it.next().unwrap().parse().unwrap();
            let color = it.next().unwrap();
            match color {
                "red" => {
                    if n > 12 {
                        game_ok = false;
                        break;
                    }
                }
                "green" => {
                    if n > 13 {
                        game_ok = false;
                        break;
                    }
                }
                "blue" => {
                    if n > 14 {
                        game_ok = false;
                        break;
                    }
                }
                _ => panic!("Unexpected color"),
            }
        }
    }
    game_ok
}

fn power(game: &str) -> i32 {
    let mut min_balls = [0, 0, 0];
    let i = game.find(':').unwrap();
    for draw in game[i + 2..].split("; ") {
        for balls in draw.split(", ") {
            let mut it = balls.split_ascii_whitespace();
            let n: i32 = it.next().unwrap().parse().unwrap();
            let color = it.next().unwrap();
            match color {
                "red" => {
                    if n > min_balls[0] {
                        min_balls[0] = n;
                    }
                }
                "green" => {
                    if n > min_balls[1] {
                        min_balls[1] = n;
                    }
                }
                "blue" => {
                    if n > min_balls[2] {
                        min_balls[2] = n;
                    }
                }
                _ => panic!("Unexpected color"),
            }
        }
    }
    min_balls[0] * min_balls[1] * min_balls[2]
}
