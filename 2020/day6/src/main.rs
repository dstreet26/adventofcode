use itertools::Itertools;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file :(");
    let list: Vec<Vec<&str>> = contents.split("\n\n").map(|x: &str| x.split("\n").collect()).collect();

    let mut count = 0;

    for i in &list {
        println!("i: {:?}", i);
        let unique_chars = i.iter().flat_map(|s| s.chars()).unique().collect::<Vec<char>>().len();
        println!("unique_chars: {:?}", unique_chars);
        count = count + unique_chars;
    }
    println!("count: {}", count);
}
