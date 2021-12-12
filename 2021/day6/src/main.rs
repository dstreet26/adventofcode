use std::fs;
fn main() {
    let contents = fs::read_to_string("input0.txt").expect("Couldn't read input file :(");
    let vectors = contents
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    println!("part1: {:?}", part1(&vectors));

    // part2 is too slow, perhaps there is an arithmatic way to solve it
    println!("part2: {:?}", part2(&vectors));
}
fn part1(vectors: &Vec<i64>) -> i64 {
    let mut out = vectors.clone();
    for _ in 0..80 {
        out = recalculate(out.clone());
    }
    return out.len() as i64;
}
fn part2(vectors: &Vec<i64>) -> i64 {
    let mut out = vectors.clone();
    for i in 0..256 {
        out = recalculate(out.clone());
        println!("{:?},{:?}", i, out.len());
    }
    return out.len() as i64;
}
fn recalculate(vectors: Vec<i64>) -> Vec<i64> {
    let mut out = vec![];
    for val in vectors {
        if val == 0 {
            out.push(6);
            out.push(8);
        } else {
            out.push(val - 1);
        }
    }
    return out;
}
