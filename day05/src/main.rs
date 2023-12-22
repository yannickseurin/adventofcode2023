use log::debug;
use simple_logger::SimpleLogger;

type Maps = Vec<Vec<Vec<usize>>>;

fn main() {
    SimpleLogger::new().init().unwrap();

    let contents = include_str!("../input");

    let seeds: Vec<_> = contents
        .lines()
        .next()
        .unwrap()
        .strip_prefix("seeds:")
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    debug!("seeds: {:?}", seeds);

    let mut line_it = contents.lines();
    line_it.next(); // seeds
    line_it.next(); //empty line

    let mut maps: Maps = Vec::new();

    for i in 0..6 {
        let mut rmap = Vec::new();
        line_it.next(); // map name
        let mut line = line_it.next().unwrap(); //first range
        while !line.is_empty() {
            let range: Vec<_> = line
                .split_ascii_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            rmap.push(range);
            line = line_it.next().unwrap();
        }
        debug!("map {}: {:?}", i + 1, rmap);
        maps.push(rmap);
    }

    // last map
    let mut rmap = Vec::new();
    line_it.next(); // map name
    let mut wline = line_it.next(); //first range
    while wline.is_some() {
        let line = wline.unwrap();
        let range: Vec<_> = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        rmap.push(range);
        wline = line_it.next();
    }
    debug!("map 7: {:?}", rmap);
    maps.push(rmap);

    let locations: Vec<_> = seeds.iter().map(|s| compose_maps(&maps, s)).collect();
    debug!("locations: {:?}", locations);

    println!("The lowest location is {}", locations.iter().min().unwrap());

    // SECOND PART

    let mut loc_min = usize::MAX;
    for p in seeds.chunks_exact(2) {
        let start = p[0];
        let len = p[1];
        debug!("seed range {start}/{len}");
        let mut seed = start;
        while seed < start + len {
            debug!("iter");
            let (loc, jump) = compose_maps_and_jump(&maps, &seed);
            if loc < loc_min {
                loc_min = loc;
            }
            seed += jump;
        }
    }

    println!("The new lowest location is {}", loc_min);
}

fn compose_maps(maps: &Maps, seed: &usize) -> usize {
    let mut i = *seed;
    debug!("=== Compose maps ===");
    for map in maps.iter() {
        for range in map.iter() {
            if i >= range[1] && i < range[1] + range[2] {
                i = range[0] + i - range[1];
                debug!("range {}", range[0]);
                break;
            }
        }
    }
    i
}

fn compose_maps_and_jump(maps: &Maps, seed: &usize) -> (usize, usize) {
    let mut i = *seed;
    let mut range_skips = Vec::new();
    debug!("=== Compose maps and jump===");
    for map in maps.iter() {
        for range in map.iter() {
            if i >= range[1] && i < range[1] + range[2] {
                range_skips.push(range[1] + range[2] - i);
                i = range[0] + i - range[1];
                debug!("range {}", range[0]);
                break;
            }
        }
    }
    (i, *range_skips.iter().min().unwrap())
}
