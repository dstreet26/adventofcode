// use itertools::Itertools;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file :(");
    let list: Vec<&str> = contents.split("\n").collect();
    let mut count = 0;

    for i in &list {
        let list2: Vec<&str> = i.split(": ").collect();
        let list3: Vec<&str> = list2[0].split(" ").collect();
        let letter = list3[1].chars().nth(0).unwrap();
        let list4: Vec<&str> = list3[0].split("-").collect();
        let position1: usize = list4[0].parse().unwrap();
        let position2: usize = list4[1].parse().unwrap();
        let the_string = list2[1];
        let position1_contains = the_string.chars().nth(position1 - 1).unwrap() == letter;
        let position2_contains = the_string.chars().nth(position2 - 1).unwrap() == letter;
        let both = position1_contains && position2_contains;
        let pass = !both && (position1_contains || position2_contains);
        if pass {
            count = count + 1
        }
    }
    println!("count: {}", count);
}
