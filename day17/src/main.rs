use log::debug;
use simple_logger::SimpleLogger;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

// stores information about each "node"
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Node {
    // position
    pos: (usize, usize),
    // incoming direction
    incoming_dir: Direction,
    // number of moves in the same direction
    same_dir: usize,
}

type Grid = Vec<Vec<u32>>;

fn main() {
    SimpleLogger::new().init().unwrap();

    let contents = include_str!("../input");

    let grid: Grid = contents
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let n_rows = grid.len();
    let n_cols = grid[0].len();

    let mut nodes_to_visit = Vec::new();
    // hashmap mapping nodes to minimal loss
    let mut hm: HashMap<Node, u32> = HashMap::new();

    // handling first two moves Down and Right
    let node_down = Node {
        pos: (1, 0),
        incoming_dir: Direction::Down,
        same_dir: 1,
    };
    hm.insert(node_down, grid[1][0]);
    nodes_to_visit.push(Node {
        pos: (1, 0),
        incoming_dir: Direction::Down,
        same_dir: 1,
    });

    let node_right = Node {
        pos: (0, 1),
        incoming_dir: Direction::Right,
        same_dir: 1,
    };
    hm.insert(node_right, grid[0][1]);
    nodes_to_visit.push(Node {
        pos: (0, 1),
        incoming_dir: Direction::Right,
        same_dir: 1,
    });

    let mut current_node = next_node(&nodes_to_visit, &hm);

    while current_node.pos != (n_rows - 1, n_cols - 1) {
        let (i, j) = current_node.pos;
        debug!("Current node: {:?}", current_node);
        debug!("Loss: {}", hm.get(&current_node).unwrap());

        // visit node turning left
        let left_dir = current_node.incoming_dir.turn_left();
        let (ii, jj) = move_pos((i, j), left_dir);
        if 0 <= ii && ii < n_rows as isize && 0 <= jj && jj < n_cols as isize {
            let ii: usize = ii.try_into().unwrap();
            let jj: usize = jj.try_into().unwrap();
            let tmp_loss = hm.get(&current_node).unwrap() + grid[ii][jj];
            let neighbour = Node {
                pos: (ii, jj),
                incoming_dir: left_dir,
                same_dir: 1,
            };
            if hm.contains_key(&neighbour) {
                let loss = hm.get(&neighbour).unwrap();
                if tmp_loss < *loss {
                    hm.insert(neighbour, tmp_loss);
                }
            } else {
                hm.insert(neighbour, tmp_loss);
                nodes_to_visit.push(neighbour);
            }
        }

        // visit node turning right
        let right_dir = current_node.incoming_dir.turn_right();
        let (ii, jj) = move_pos((i, j), right_dir);
        if 0 <= ii && ii < n_rows as isize && 0 <= jj && jj < n_cols as isize {
            let ii: usize = ii.try_into().unwrap();
            let jj: usize = jj.try_into().unwrap();
            let tmp_loss = hm.get(&current_node).unwrap() + grid[ii][jj];
            let neighbour = Node {
                pos: (ii, jj),
                incoming_dir: right_dir,
                same_dir: 1,
            };
            if hm.contains_key(&neighbour) {
                let loss = hm.get(&neighbour).unwrap();
                if tmp_loss < *loss {
                    hm.insert(neighbour, tmp_loss);
                }
            } else {
                hm.insert(neighbour, tmp_loss);
                nodes_to_visit.push(neighbour);
            }
        }

        // visit node forward if possible
        if current_node.same_dir < 3 {
            let forward_dir = current_node.incoming_dir;
            let (ii, jj) = move_pos((i, j), forward_dir);
            if 0 <= ii && ii < n_rows as isize && 0 <= jj && jj < n_cols as isize {
                let ii: usize = ii.try_into().unwrap();
                let jj: usize = jj.try_into().unwrap();
                let tmp_loss = hm.get(&current_node).unwrap() + grid[ii][jj];
                let neighbour = Node {
                    pos: (ii, jj),
                    incoming_dir: forward_dir,
                    same_dir: current_node.same_dir + 1,
                };
                if hm.contains_key(&neighbour) {
                    let loss = hm.get(&neighbour).unwrap();
                    if tmp_loss < *loss {
                        hm.insert(neighbour, tmp_loss);
                        debug! {"Updating node: {:?}", neighbour};
                        debug! {"New loss: {}", tmp_loss};
                    }
                } else {
                    hm.insert(neighbour, tmp_loss);
                    nodes_to_visit.push(neighbour);
                    debug! {"Creating node: {:?}", neighbour};
                    debug! {"Loss: {}", tmp_loss};
                }
            }
        }

        nodes_to_visit.retain(|&n| n != current_node);
        current_node = next_node(&nodes_to_visit, &hm);
        debug! {"Node handled!\n"};
    }

    println!(
        "The minimal heat loss is {:?}",
        hm.get(&current_node).unwrap()
    );

    // SECOND PART
    // we must arrive at the final node with same_dir >= 4

    let mut min_loss = u32::MAX;
    for i in 4..11 {
        let final_node = Node {
            pos: (n_rows - 1, n_cols - 1),
            incoming_dir: Direction::Down,
            same_dir: i,
        };
        let tmp_loss = dijsktra_second_part(&grid, final_node);
        if tmp_loss < min_loss {
            min_loss = tmp_loss;
        }
        let final_node = Node {
            pos: (n_rows - 1, n_cols - 1),
            incoming_dir: Direction::Right,
            same_dir: i,
        };
        let tmp_loss = dijsktra_second_part(&grid, final_node);
        if tmp_loss < min_loss {
            min_loss = tmp_loss;
        }
    }

    println!("The minimal heat loss for the second part is {min_loss}");
}

fn next_node(nodes_to_visit: &[Node], hm: &HashMap<Node, u32>) -> Node {
    let mut next_node = nodes_to_visit[0];
    for node in &nodes_to_visit[1..] {
        if hm.get(node).unwrap() < hm.get(&next_node).unwrap() {
            next_node = *node;
        }
    }
    next_node
}

fn move_pos(pos: (usize, usize), dir: Direction) -> (isize, isize) {
    let (i, j) = pos;
    match dir {
        Direction::Up => (i as isize - 1, j as isize),
        Direction::Down => (i as isize + 1, j as isize),
        Direction::Left => (i as isize, j as isize - 1),
        Direction::Right => (i as isize, j as isize + 1),
    }
}

// returns minimal loss depending on final node
fn dijsktra_second_part(grid: &Grid, final_node: Node) -> u32 {
    println!("Final node: {:?}", final_node);
    let n_rows = grid.len();
    let n_cols = grid[0].len();

    let mut nodes_to_visit = Vec::new();
    // hashmap mapping nodes to minimal loss
    let mut hm: HashMap<Node, u32> = HashMap::new();

    // handling first two moves Down and Right
    let node_down = Node {
        pos: (1, 0),
        incoming_dir: Direction::Down,
        same_dir: 1,
    };
    hm.insert(node_down, grid[1][0]);
    nodes_to_visit.push(Node {
        pos: (1, 0),
        incoming_dir: Direction::Down,
        same_dir: 1,
    });

    let node_right = Node {
        pos: (0, 1),
        incoming_dir: Direction::Right,
        same_dir: 1,
    };
    hm.insert(node_right, grid[0][1]);
    nodes_to_visit.push(Node {
        pos: (0, 1),
        incoming_dir: Direction::Right,
        same_dir: 1,
    });

    let mut current_node = next_node(&nodes_to_visit, &hm);

    while current_node != final_node {
        let (i, j) = current_node.pos;
        debug!("Current node: {:?}", current_node);
        debug!("Loss: {}", hm.get(&current_node).unwrap());

        // visit node turning left if possible
        if current_node.same_dir >= 4 {
            let left_dir = current_node.incoming_dir.turn_left();
            let (ii, jj) = move_pos((i, j), left_dir);
            if 0 <= ii && ii < n_rows as isize && 0 <= jj && jj < n_cols as isize {
                let ii: usize = ii.try_into().unwrap();
                let jj: usize = jj.try_into().unwrap();
                let tmp_loss = hm.get(&current_node).unwrap() + grid[ii][jj];
                let neighbour = Node {
                    pos: (ii, jj),
                    incoming_dir: left_dir,
                    same_dir: 1,
                };
                if hm.contains_key(&neighbour) {
                    let loss = hm.get(&neighbour).unwrap();
                    if tmp_loss < *loss {
                        hm.insert(neighbour, tmp_loss);
                    }
                } else {
                    hm.insert(neighbour, tmp_loss);
                    nodes_to_visit.push(neighbour);
                }
            }
        }

        // visit node turning right if possible
        if current_node.same_dir >= 4 {
            let right_dir = current_node.incoming_dir.turn_right();
            let (ii, jj) = move_pos((i, j), right_dir);
            if 0 <= ii && ii < n_rows as isize && 0 <= jj && jj < n_cols as isize {
                let ii: usize = ii.try_into().unwrap();
                let jj: usize = jj.try_into().unwrap();
                let tmp_loss = hm.get(&current_node).unwrap() + grid[ii][jj];
                let neighbour = Node {
                    pos: (ii, jj),
                    incoming_dir: right_dir,
                    same_dir: 1,
                };
                if hm.contains_key(&neighbour) {
                    let loss = hm.get(&neighbour).unwrap();
                    if tmp_loss < *loss {
                        hm.insert(neighbour, tmp_loss);
                    }
                } else {
                    hm.insert(neighbour, tmp_loss);
                    nodes_to_visit.push(neighbour);
                }
            }
        }

        // visit node forward if possible
        if current_node.same_dir < 10 {
            let forward_dir = current_node.incoming_dir;
            let (ii, jj) = move_pos((i, j), forward_dir);
            if 0 <= ii && ii < n_rows as isize && 0 <= jj && jj < n_cols as isize {
                let ii: usize = ii.try_into().unwrap();
                let jj: usize = jj.try_into().unwrap();
                let tmp_loss = hm.get(&current_node).unwrap() + grid[ii][jj];
                let neighbour = Node {
                    pos: (ii, jj),
                    incoming_dir: forward_dir,
                    same_dir: current_node.same_dir + 1,
                };
                if hm.contains_key(&neighbour) {
                    let loss = hm.get(&neighbour).unwrap();
                    if tmp_loss < *loss {
                        hm.insert(neighbour, tmp_loss);
                        debug! {"Updating node: {:?}", neighbour};
                        debug! {"New loss: {}", tmp_loss};
                    }
                } else {
                    hm.insert(neighbour, tmp_loss);
                    nodes_to_visit.push(neighbour);
                    debug! {"Creating node: {:?}", neighbour};
                    debug! {"Loss: {}", tmp_loss};
                }
            }
        }

        nodes_to_visit.retain(|&n| n != current_node);
        current_node = next_node(&nodes_to_visit, &hm);
        debug! {"Node handled!\n"};
    }
    *hm.get(&current_node).unwrap()
}
