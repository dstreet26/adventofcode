use itertools::Itertools;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file :(");
    let list: Vec<Vec<&str>> = contents.split("\n\n").map(|x: &str| x.split("\n").collect()).collect();
    let mut count = 0;
    for group in &list {
        let chars_iter = group.iter().flat_map(|s| s.chars());
        let chars_vec : Vec<char> = chars_iter.clone().collect();
        let unique_votes : Vec<char> = chars_iter.unique().collect();
        let group_count = group.len();
        for unique_vote in &unique_votes {
                let mut this_count = 0;
                for vote in &chars_vec {
                    if vote == unique_vote {
                        this_count = this_count + 1;
                    }
                }
                if this_count >= group_count {
                    count = count + 1;
                }
            }
    }
    println!("count: {}", count);
}
