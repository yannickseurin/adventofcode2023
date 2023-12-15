use log::debug;
use simple_logger::SimpleLogger;
use std::collections::HashSet;

#[macro_export]
macro_rules! pause {
    () => {
        println!(
            "[{}:{}] Pausing! Press enter to continue...",
            file!(),
            line!()
        );

        let mut buffer = String::new();

        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");
    };
}

const N: usize = 140;

type Point = (usize, usize);

type Grid = [[char; N]; N];

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy)]
struct Node {
    point: Point,
    out_dir: Direction,
}

fn main() {
    SimpleLogger::new().init().unwrap();

    let contents = include_str!("../input");

    let mut grid: Grid = contents
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>().try_into().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    // debug!("Grid: {:?}", grid);

    // find the animal
    let mut start: Point = (0, 0);
    'row_loop: for (i, row) in grid.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if *tile == 'S' {
                start = (i, j);
                break 'row_loop;
            }
        }
    }

    debug!("Starting point: {:?}", start);

    // the loop goes north and east
    let mut wnode = Some(Node {
        point: start,
        out_dir: Direction::North,
    });
    let mut length = 0;
    // store points on the curve in a HashSet
    let mut curve: HashSet<Point> = HashSet::new();
    debug!("Start: {:?}", wnode);
    while let Some(node) = wnode {
        curve.insert(node.point);
        length += 1;
        if let Some(next_point) = next_point(&node) {
            if grid[next_point.0][next_point.1] == 'S' {
                debug!("Loop! length {length}");
                break;
            }
        }
        wnode = next_node(&grid, &node);
    }

    println!("Half-length of the loop: {}", length / 2);

    // compute area
    let mut area = 0;

    // replace S by L
    grid[start.0][start.1] = 'L';

    for (i, row) in grid.iter().enumerate().skip(1).take(N - 2) {
        let mut inside = false;
        let mut on_l_line = false;
        let mut on_f_line = false;
        for (j, tile) in row.iter().enumerate() {
            // one must switch in/out if one crosses '|' or 'L---7' or 'F---J', but not on 'L---J' or 'F---7'
            if curve.contains(&(i, j)) && *tile == '|' {
                inside = !inside;
            }
            if curve.contains(&(i, j)) && *tile == 'L' {
                on_l_line = true;
            }
            if curve.contains(&(i, j)) && *tile == 'F' {
                on_f_line = true;
            }
            if curve.contains(&(i, j)) && *tile == '7' {
                if on_l_line {
                    inside = !inside;
                    on_l_line = false;
                } else {
                    // one must be on an F line
                    on_f_line = false;
                }
            }
            if curve.contains(&(i, j)) && *tile == 'J' {
                if on_f_line {
                    inside = !inside;
                    on_f_line = false;
                } else {
                    // one must be on an L line
                    on_l_line = false;
                }
            }
            if !curve.contains(&(i, j)) && inside {
                area += 1;
            }
        }
        debug!("Line {i} ends inside: {inside}");
    }

    println!("Area: {area}");
}

fn next_point(&node: &Node) -> Option<Point> {
    match node.out_dir {
        Direction::North => {
            if node.point.0 == 0 {
                return None;
            }
            Some((node.point.0 - 1, node.point.1))
        }
        Direction::South => {
            if node.point.0 == N - 1 {
                return None;
            }
            Some((node.point.0 + 1, node.point.1))
        }
        Direction::East => {
            if node.point.1 == N - 1 {
                return None;
            }
            Some((node.point.0, node.point.1 + 1))
        }
        Direction::West => {
            if node.point.1 == 0 {
                return None;
            }
            Some((node.point.0, node.point.1 - 1))
        }
    }
}

// given a node (point, out_dir), returns Some(Node) if the adjacent node in direction out_dir is connected
// or None if it is not connected (border of the grid or incompatible pipe)
fn next_node(grid: &Grid, node: &Node) -> Option<Node> {
    if let Some(next_point) = next_point(node) {
        match node.out_dir {
            Direction::North => {
                match grid[next_point.0][next_point.1] {
                    '|' => Some(Node {
                        point: next_point,
                        out_dir: Direction::North,
                    }),
                    'F' => Some(Node {
                        point: next_point,
                        out_dir: Direction::East,
                    }),
                    '7' => Some(Node {
                        point: next_point,
                        out_dir: Direction::West,
                    }),
                    _ => None,
                }
            }
            Direction::South => {
                match grid[next_point.0][next_point.1] {
                    '|' => Some(Node {
                        point: next_point,
                        out_dir: Direction::South,
                    }),
                    'L' => Some(Node {
                        point: next_point,
                        out_dir: Direction::East,
                    }),
                    'J' => Some(Node {
                        point: next_point,
                        out_dir: Direction::West,
                    }),
                    _ => None,
                }
            }
            Direction::East => {
                match grid[next_point.0][next_point.1] {
                    '-' => Some(Node {
                        point: next_point,
                        out_dir: Direction::East,
                    }),
                    'J' => Some(Node {
                        point: next_point,
                        out_dir: Direction::North,
                    }),
                    '7' => Some(Node {
                        point: next_point,
                        out_dir: Direction::South,
                    }),
                    _ => None,
                }
            }
            Direction::West => {
                match grid[next_point.0][next_point.1] {
                    '-' => Some(Node {
                        point: next_point,
                        out_dir: Direction::West,
                    }),
                    'L' => Some(Node {
                        point: next_point,
                        out_dir: Direction::North,
                    }),
                    'F' => Some(Node {
                        point: next_point,
                        out_dir: Direction::South,
                    }),
                    _ => None,
                }
            }
        }
    }
    else {
        None
    }
}
