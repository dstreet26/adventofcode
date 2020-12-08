// use itertools::Itertools;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file :(");
    let list: Vec<&str> = contents.split("\n").collect();
    let mut count = 0;
    let mut paths = vec!["shiny gold".to_string()];
    let mut paths2 = vec!["shiny gold".to_string()];
    while paths.len() > 0 {
        let target = paths.pop().unwrap();
        println!("target: {:?}", target);
        for i in &list {
            let remove_period = i.replace(".", "");
            let left_right: Vec<&str> = remove_period.split("contain").map(|x| x.trim()).collect();
            let left = left_right[0].replace(" bags", "").replace(" bag", "");
            let right = left_right[1];
            if right == "no other bags" {
                continue;
            }
            let right_split: Vec<&str> = right.split(",").map(|x| x.trim()).collect();
            for innerbag in right_split {
                let remove_bags = innerbag.replace(" bags", "").replace(" bag", "");
                let split1: Vec<&str> = remove_bags.split(" ").collect();
                let digit = split1[0].parse().unwrap_or(0);
                let inner_color: String = split1[1..].join(" ");
                if inner_color == target {
                    println!("add: {:?}", left);
                    if !paths2.contains(&left.clone()) {
                        count = count + 1;
                        paths.push(left.clone());
                    }
                    paths2.push(left.clone());
                    break;
                }
            }
        }
    }
    println!("count: {}", count);
}
