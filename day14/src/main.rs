use log::debug;
use simple_logger::SimpleLogger;
use std::collections::HashMap;

type Grid = Vec<Vec<char>>;

fn main() {
    SimpleLogger::new().init().unwrap();

    let contents = include_str!("../input");

    let mut grid: Grid = contents.lines().map(|l| l.chars().collect()).collect();

    tilt_north(&mut grid);

    debug!("{:?}", grid);

    let load: usize = grid
        .iter()
        .enumerate()
        .map(|row| {
            let n = row.1.iter().filter(|c| **c == 'O').count();
            n * (100 - row.0)
        })
        .sum();

    println!("The total load is {load}");

    // SECOND PART

    const N: usize = 1000000000;

    let mut grid: Grid = contents.lines().map(|l| l.chars().collect()).collect();

    let mut hm = HashMap::new();
    hm.insert(grid.clone(), 0);

    let mut ctr = 0;
    loop {
        ctr += 1;
        cycle(&mut grid);
        if hm.contains_key(&grid) {
            println!("cycle! {}, {}", hm.get(&grid).unwrap(), ctr);
            break;
        }
        hm.insert(grid.clone(), ctr);
    }
    let cycle_length = ctr - hm.get(&grid).unwrap();
    debug! {"cycle length: {cycle_length}"};

    for _i in 0..(N - ctr) % cycle_length {
        cycle(&mut grid);
    }

    let new_load: usize = grid
        .iter()
        .enumerate()
        .map(|row| {
            let n = row.1.iter().filter(|c| **c == 'O').count();
            n * (100 - row.0)
        })
        .sum();

    println!("The new total load is {new_load}");
}

fn tilt_north(grid: &mut Grid) {
    for j in 0..grid[0].len() {
        for i in 0..grid.len() {
            if grid[i][j] == 'O' {
                let mut stop = i;
                while stop > 0 && grid[stop - 1][j] == '.' {
                    stop -= 1;
                }
                grid[i][j] = '.';
                grid[stop][j] = 'O';
            }
        }
    }
}

fn tilt_south(grid: &mut Grid) {
    for j in 0..grid[0].len() {
        for i in (0..grid.len()).rev() {
            if grid[i][j] == 'O' {
                let mut stop = i;
                while stop < grid.len() - 1 && grid[stop + 1][j] == '.' {
                    stop += 1;
                }
                grid[i][j] = '.';
                grid[stop][j] = 'O';
            }
        }
    }
}

fn tilt_west(grid: &mut Grid) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'O' {
                let mut stop = j;
                while stop > 0 && grid[i][stop - 1] == '.' {
                    stop -= 1;
                }
                grid[i][j] = '.';
                grid[i][stop] = 'O';
            }
        }
    }
}

fn tilt_east(grid: &mut Grid) {
    for i in 0..grid.len() {
        for j in (0..grid[0].len()).rev() {
            if grid[i][j] == 'O' {
                let mut stop = j;
                while stop < grid[0].len() - 1 && grid[i][stop + 1] == '.' {
                    stop += 1;
                }
                grid[i][j] = '.';
                grid[i][stop] = 'O';
            }
        }
    }
}

fn cycle(grid: &mut Grid) {
    tilt_north(grid);
    tilt_west(grid);
    tilt_south(grid);
    tilt_east(grid);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn cycle() {
        let test_input = include_str!("../test");
        let result = include_str!("../result");
        let mut input_grid: Grid = test_input.lines().map(|l| l.chars().collect()).collect();
        let mut result_grid: Grid = result.lines().map(|l| l.chars().collect()).collect();

        println!("initial state");
        for row in input_grid.iter() {
            println!("{:?}", row);
        }
        tilt_north(&mut input_grid);
        println!("after tilt north");
        for row in input_grid.iter() {
            println!("{:?}", row);
        }
        tilt_west(&mut input_grid);
        println!("after tilt west");
        for row in input_grid.iter() {
            println!("{:?}", row);
        }
        tilt_south(&mut input_grid);
        println!("after tilt south");
        for row in input_grid.iter() {
            println!("{:?}", row);
        }
        tilt_east(&mut input_grid);
        println!("after tilt east");
        for row in input_grid.iter() {
            println!("{:?}", row);
        }
        assert_eq!(input_grid, result_grid);
    }

    #[test]
    fn north_south() {
        let test_input = include_str!("../test");
        let mut input_grid: Grid = test_input.lines().map(|l| l.chars().collect()).collect();
        let mut result_grid: Grid = test_input.lines().map(|l| l.chars().collect()).collect();

        tilt_north(&mut result_grid);
        tilt_north(&mut input_grid);

        tilt_south(&mut input_grid);
        tilt_north(&mut input_grid);
        assert_eq!(input_grid, result_grid);
    }

    #[test]
    fn east_west() {
        let test_input = include_str!("../test");
        let mut input_grid: Grid = test_input.lines().map(|l| l.chars().collect()).collect();
        let mut result_grid: Grid = test_input.lines().map(|l| l.chars().collect()).collect();

        tilt_west(&mut result_grid);
        tilt_west(&mut input_grid);

        tilt_east(&mut input_grid);
        tilt_west(&mut input_grid);
        assert_eq!(input_grid, result_grid);
    }
}
