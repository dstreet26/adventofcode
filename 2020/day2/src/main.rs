use itertools::Itertools;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file :(");
    let list: Vec<&str> = contents.split("\n").collect();
    let mut count = 0;

    for i in &list {
        let list2: Vec<&str> = i.split(": ").collect();
        let list3: Vec<&str> = list2[0].split(" ").collect();
        let letter = list3[1];
        let list4: Vec<&str> = list3[0].split("-").collect();
        let min = list4[0].parse().unwrap();
        let max = list4[1].parse().unwrap();
        let string = list2[1];
        let countOfNumber = string.split("").filter(|x| x == &letter).count();
        let pass = countOfNumber <= max && countOfNumber >= min;
        if (pass) {
            count = count + 1
        }
    }
    println!("count: {}", count);
}
