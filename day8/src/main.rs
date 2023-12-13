use log::debug;
use simple_logger::SimpleLogger;
use std::collections::HashMap;

fn main() {
    SimpleLogger::new().init().unwrap();

    let contents = include_str!("../input");

    // build a hashmap mapping strings "XYZ" to pairs of strings ("RST", "UVW")
    let mut hm: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in contents.lines().skip(2) {
        let v: Vec<_> = line.split_ascii_whitespace().collect();
        hm.insert(v[0], (&v[2][1..4], &v[3][0..3]));
    }
    debug!("{:?}", hm);

    // retrieve LR sequence
    let lrseq: Vec<_> = contents.lines().next().unwrap().chars().collect();
    let seqlen = lrseq.len();

    debug!("LR seq length: {seqlen}");

    let mut node = "AAA";
    let mut ctr = 0;
    while node != "ZZZ" {
        let (lnode, rnode) = hm.get(node).unwrap();
        if lrseq[ctr % seqlen] == 'L' {
            node = lnode;
        } else {
            node = rnode;
        }
        ctr += 1;
    }

    println!("The number of steps is {ctr}");

    let nodes: Vec<&str> = contents
        .lines()
        .skip(2)
        .filter(|l| l.chars().nth(2).unwrap() == 'A')
        .map(|l| &l[0..3])
        .collect();
    debug!("=== STARTING NODES ===: {:?}", nodes);

    let mut cycle_lengths = Vec::new();

    for n in nodes.iter() {
        let mut node = n;
        debug!("===STARTING NODE: {node}");
        ctr = 0;
        // find first node and next move
        debug!("--- First trip ---");
        loop {
            let (lnode, rnode) = hm.get(node).unwrap();
            if lrseq[ctr % seqlen] == 'L' {
                node = lnode;
            } else {
                node = rnode;
            }
            ctr += 1;
            if ends_with_z(node) {
                let first_visit = (node, lrseq[ctr % seqlen]);
                debug!("First node/move reached: {:?}", first_visit);
                debug!("Number of steps: {ctr}");
                break;
            }
        }
        let tail = ctr;
        // find next node
        debug!("--- Next trip ---");
        loop {
            let (lnode, rnode) = hm.get(node).unwrap();
            if lrseq[ctr % seqlen] == 'L' {
                node = lnode;
            } else {
                node = rnode;
            }
            ctr += 1;
            if ends_with_z(node) {
                debug!("Next node/move reached: {:?}", (node, lrseq[ctr % seqlen]));
                debug!("Number of steps: {}", ctr);
                break;
            }
        }
        cycle_lengths.push(tail);
    }

    // Each starting node ends up on a unique node ending
    // with Z, and then it is on a cycle with the same number of steps
    // e.g. "AAA" -- 19637 steps --> "ZZZ" -- 19637 steps --> "ZZZ"
    // which makes things much simpler to solve

    println!("{:?}", cycle_lengths);
    // [19637, 12643, 14257, 15871, 11567, 19099]
    // finished with Sagemath :)
    // answer: 8811050362409
}

fn ends_with_z(node: &str) -> bool {
    node.chars().nth(2).unwrap() == 'Z'
}
