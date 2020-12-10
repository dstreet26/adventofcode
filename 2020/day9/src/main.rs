use itertools::Itertools;
use std::fs;
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file :(");

    let input: Vec<usize> = contents
        .split("\n")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let preamble_length = 25;

    for i in preamble_length..input.len() {
        let element = input[i];
        let before: Vec<usize> = input[i - preamble_length..i].to_vec();
        let combs: Vec<Vec<usize>> = before.into_iter().combinations(2).collect();
        let mut has_sum = false;
        for j in combs {
            let sum: usize = j.iter().sum();
            println!("i:{:?}, el:{:?}, comb:{:?}, sum:{:?}", i, element, j, sum);
            if sum == element {
                has_sum = true;
                break;
            }
        }
        if !has_sum {
            println!("{:?}", element);
            break;
        }
    }
}
