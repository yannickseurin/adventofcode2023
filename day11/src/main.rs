use log::debug;
use simple_logger::SimpleLogger;

const N: usize = 140;

fn main() {
    SimpleLogger::new().init().unwrap();

    let contents = include_str!("../input");

    let mut array: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();

    // insert empty rows
    let mut inserted_rows = 0;
    for i in 0..N {
        if is_empty(&array[i + inserted_rows]) {
            debug!("Row {i} is empty");
            array.insert(i + inserted_rows, vec!['.'; N]);
            inserted_rows += 1;
        }
    }

    // insert empty columns
    let mut inserted_cols = 0;
    for j in 0..N {
        if is_empty(
            &array
                .iter()
                .map(|row| *row.get(j + inserted_cols).unwrap())
                .collect::<Vec<_>>(),
        ) {
            debug!("Col {j} is empty");
            for row in array.iter_mut() {
                row.insert(j + inserted_cols, '.');
            }
            inserted_cols += 1;
        }
    }

    let mut galaxy_positions = Vec::new();
    for (i, row) in array.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if *ch == '#' {
                galaxy_positions.push((i, j));
            }
        }
    }

    debug!("Positions: {:?}", galaxy_positions);

    let mut sum = 0;

    for (k, (i1, j1)) in galaxy_positions.iter().enumerate() {
        for (i2, j2) in galaxy_positions.iter().skip(k + 1) {
            if j1 < j2 {
                sum += i2 - i1 + j2 - j1;
            } else {
                sum += i2 - i1 + j1 - j2;
            }
        }
    }

    println!("The sum of distances is {sum}");

    // SECOND PART

    let array: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();

    let empty_rows = array
        .iter()
        .enumerate()
        .filter(|e| is_empty(e.1))
        .map(|e| e.0)
        .collect::<Vec<_>>();
    debug! {"Empty rows: {:?}", empty_rows};
    let empty_cols = (0..N)
        .filter(|j| {
            is_empty(
                &array
                    .iter()
                    .map(|row| *row.get(*j).unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    debug! {"Empty cols: {:?}", empty_cols};

    let mut galaxy_positions = Vec::new();
    for (i, row) in array.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if *ch == '#' {
                galaxy_positions.push((i, j));
            }
        }
    }

    debug!("Positions: {:?}", galaxy_positions);

    let mut sum = 0;
    const EXP: usize = 1000000;

    for (k, (i1, j1)) in galaxy_positions.iter().enumerate() {
        for (i2, j2) in galaxy_positions.iter().skip(k + 1) {
            for i in i1 + 1..*i2 + 1 {
                if empty_rows.contains(&i) {
                    sum += EXP;
                } else {
                    sum += 1;
                }
            }
            if j1 < j2 {
                for j in j1 + 1..*j2 + 1 {
                    if empty_cols.contains(&j) {
                        sum += EXP;
                    } else {
                        sum += 1;
                    }
                }
            }
            if j2 < j1 {
                for j in j2 + 1..*j1 + 1 {
                    if empty_cols.contains(&j) {
                        sum += EXP;
                    } else {
                        sum += 1;
                    }
                }
            }
        }
    }

    println!("The new sum of distances is {sum}");
}

fn is_empty(vec: &[char]) -> bool {
    for c in vec.iter() {
        if *c != '.' {
            return false;
        }
    }
    true
}
