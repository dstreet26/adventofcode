use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file :(");
    let list: Vec<&str> = contents.split("\n").collect();
    let mut count = 0;
    let mut paths = vec![("shiny gold".to_string(), 1)];
    while paths.len() > 0 {
        let target2 = paths.pop().unwrap();
        let (target, target_multiplier) = target2;
        for i in &list {
            let remove_period = i.replace(".", "");
            let left_right: Vec<&str> = remove_period.split("contain").map(|x| x.trim()).collect();
            let left = left_right[0].replace(" bags", "").replace(" bag", "");
            let right = left_right[1];
            if right == "no other bags" {
                continue;
            }
            let right_split: Vec<&str> = right.split(",").map(|x| x.trim()).collect();
            if left == target {
                for innerbag in right_split {
                    let remove_bags = innerbag.replace(" bags", "").replace(" bag", "");
                    let split1: Vec<&str> = remove_bags.split(" ").collect();
                    let digit = split1[0].parse().unwrap_or(0);
                    let inner_color: String = split1[1..].join(" ");
                    count = count + (digit * target_multiplier);
                    paths.push((inner_color, (digit * target_multiplier)));
                }
                break;
            }
        }
    }
    println!("count: {}", count);
}
