use itertools::Itertools;
use std::fs;
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file :(");

    let combs: Vec<Vec<i32>> = contents
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .combinations(3)
        .collect();

    for i in combs {
        let sum: i32 = i.iter().sum();
        if sum == 2020 {
            let product: i32 = i.iter().product();
            println!("{:?}", i);
            println!("{:?}", product);
        }
    }
}
