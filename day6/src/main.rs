fn main() {
    let n1 = n_ways_win(47, 207);
    let n2 = n_ways_win(84, 1394);
    let n3 = n_ways_win(74, 1209);
    let n4 = n_ways_win(67, 1014);
    let prod = n1*n2*n3*n4;

    println!("The product of n_i's is {prod}");

    let n = n_ways_win(47847467, 207139412091014);

    println!("The new n is {n}");
}

fn n_ways_win(time: usize, record: usize) -> usize {
    let mut n = 0;

    for i in 1..time {
        let d = i*(time-i);
        if d > record { n += 1; }
    }
    n
}