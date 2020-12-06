use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file :(");
    let list: Vec<&str> = contents.split("\n").collect();
    let mut highest = 0;

    for i in &list {
        let x = get_front_or_back(&i[..7]);
        let y = get_left_or_right(&i[7..]);
        let id = (x * 8) + y;
        println!("id: {:?}", id);
        if id > highest {
            highest = id;
        }
    }
    println!("hightest id: {}", highest);
}

fn get_front_or_back(input : &str) -> u32 {
    let mut min = 0;
    let mut max = 127;
    let mut out = 0;
    for i in input.chars() {
        if i == 'B' {
            min = subdivide_back(min,max);
            out = min;
        } else if i == 'F' {
            max = subdivide_front(min,max);
            out = max;
        }
    }
    out
}
fn get_left_or_right(input : &str) -> u32 {
    let mut min = 0;
    let mut max = 7;
    let mut out = 0;
    for i in input.chars() {
        if i == 'R' {
            min = subdivide_back(min,max);
            out = min;
        } else if i == 'L' {
            max = subdivide_front(min,max);
            out = max;
        }
    }
    out
}
fn subdivide_back(min: u32, max: u32) -> u32 {
    let diff = ((max - min) + 1)  / 2;
    if diff == 1 {
        max
    } else {
        min + diff
        
    }
}
fn subdivide_front(min: u32, max: u32) -> u32 {
    let diff = ((max - min) + 1)  / 2;
    if diff == 1 {
        min
    } else {
        max - diff

    }
}