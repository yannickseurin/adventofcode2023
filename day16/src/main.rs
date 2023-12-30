use log::debug;
use simple_logger::SimpleLogger;

#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct Tile {
    // character at the tile position
    what: char,
    // whether the tile is energized or not
    energized: bool,
    // beams incident to this tile
    incident: Vec<Direction>,
}

type Grid = Vec<Vec<Tile>>;

fn main() {
    SimpleLogger::new().init().unwrap();

    let contents = include_str!("../input");

    let mut grid: Grid = contents
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| Tile {
                    what: c,
                    energized: false,
                    incident: Vec::new(),
                })
                .collect()
        })
        .collect();

    propagate((0, 0), Direction::Right, &mut grid);

    // let sum: usize = grid.iter().map(|row| row.iter().filter(|tile| tile.energized).count()).sum();

    println!(
        "The number of energized tiles is {}",
        count_energized(&grid)
    );

    // SECOND PART

    let init_grid: Grid = contents
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| Tile {
                    what: c,
                    energized: false,
                    incident: Vec::new(),
                })
                .collect()
        })
        .collect();

    let mut max = 0;

    for i in 0..init_grid.len() {
        let mut grid = init_grid.clone();
        propagate((i as isize, 0), Direction::Right, &mut grid);
        if count_energized(&grid) > max {
            max = count_energized(&grid);
        }
        let mut grid = init_grid.clone();
        propagate(
            (i as isize, grid[0].len() as isize - 1),
            Direction::Left,
            &mut grid,
        );
        if count_energized(&grid) > max {
            max = count_energized(&grid);
        }
    }

    for j in 0..init_grid[0].len() {
        let mut grid = init_grid.clone();
        propagate((0, j as isize), Direction::Down, &mut grid);
        if count_energized(&grid) > max {
            max = count_energized(&grid);
        }
        let mut grid = init_grid.clone();
        propagate(
            (grid.len() as isize - 1, j as isize),
            Direction::Up,
            &mut grid,
        );
        if count_energized(&grid) > max {
            max = count_energized(&grid);
        }
    }

    println!("The maximal number of energized tiles is {max}");
}

fn propagate(pos: (isize, isize), dir: Direction, grid: &mut Grid) {
    if pos.0 < 0
        || pos.0 >= grid.len().try_into().unwrap()
        || pos.1 < 0
        || pos.1 >= grid[0].len().try_into().unwrap()
    {
        // out of grid
    } else {
        let i = pos.0 as usize;
        let j = pos.1 as usize;

        if grid[i][j].incident.contains(&dir) {
            return;
        }

        grid[i][j].energized = true;
        grid[i][j].incident.push(dir.clone());

        match grid[i][j].what {
            '.' => match dir {
                Direction::Up => {
                    propagate((pos.0 - 1, pos.1), Direction::Up, grid);
                }
                Direction::Down => {
                    propagate((pos.0 + 1, pos.1), Direction::Down, grid);
                }
                Direction::Left => {
                    propagate((pos.0, pos.1 - 1), Direction::Left, grid);
                }
                Direction::Right => {
                    propagate((pos.0, pos.1 + 1), Direction::Right, grid);
                }
            },
            '/' => match dir {
                Direction::Up => {
                    propagate((pos.0, pos.1 + 1), Direction::Right, grid);
                }
                Direction::Down => {
                    propagate((pos.0, pos.1 - 1), Direction::Left, grid);
                }
                Direction::Left => {
                    propagate((pos.0 + 1, pos.1), Direction::Down, grid);
                }
                Direction::Right => {
                    propagate((pos.0 - 1, pos.1), Direction::Up, grid);
                }
            },
            '\\' => match dir {
                Direction::Up => {
                    propagate((pos.0, pos.1 - 1), Direction::Left, grid);
                }
                Direction::Down => {
                    propagate((pos.0, pos.1 + 1), Direction::Right, grid);
                }
                Direction::Left => {
                    propagate((pos.0 - 1, pos.1), Direction::Up, grid);
                }
                Direction::Right => {
                    propagate((pos.0 + 1, pos.1), Direction::Down, grid);
                }
            },
            '-' => match dir {
                Direction::Up | Direction::Down => {
                    propagate((pos.0, pos.1 + 1), Direction::Right, grid);
                    propagate((pos.0, pos.1 - 1), Direction::Left, grid);
                }
                Direction::Left => {
                    propagate((pos.0, pos.1 - 1), Direction::Left, grid);
                }
                Direction::Right => {
                    propagate((pos.0, pos.1 + 1), Direction::Right, grid);
                }
            },
            '|' => match dir {
                Direction::Up => {
                    propagate((pos.0 - 1, pos.1), Direction::Up, grid);
                }
                Direction::Down => {
                    propagate((pos.0 + 1, pos.1), Direction::Down, grid);
                }
                Direction::Left | Direction::Right => {
                    propagate((pos.0 - 1, pos.1), Direction::Up, grid);
                    propagate((pos.0 + 1, pos.1), Direction::Down, grid);
                }
            },
            _ => {
                panic!("Unexpected character");
            }
        }
    }
}

fn count_energized(grid: &Grid) -> usize {
    grid.iter()
        .map(|row| row.iter().filter(|tile| tile.energized).count())
        .sum::<usize>()
}
