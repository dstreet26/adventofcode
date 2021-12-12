use std::fs;
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file :(");
    let line_pairs = contents
        .split("\n")
        .map(|s| s.split(" | ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    println!("part1: {:?}", part1(&line_pairs));
}
fn part1(line_pairs: &Vec<Vec<&str>>) -> i64 {
    let mut sum = 0;
    for line in line_pairs {
        let right = line[1];
        let right_words = right.split(" ").collect::<Vec<&str>>();
        for word in right_words {
            match word.len() {
                2 => sum += 1,
                3 => sum += 1,
                4 => sum += 1,
                7 => sum += 1,
                _ => {}
            }
        }
    }
    return sum;
}
