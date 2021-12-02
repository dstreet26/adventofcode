use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file :(");

    let lines: Vec<&str> = contents.split("\n").collect();
    println!("part1: {:?}", part1(&lines));
    println!("part2: {:?}", part2(&lines));
}
fn part1(lines: &Vec<&str>) -> i32 {
    let mut horizontal_position = 0;
    let mut depth = 0;
    for line in lines {
        let split = line.split(" ").collect::<Vec<&str>>();
        let val = split[1].parse::<i32>().unwrap();
        match split[0] {
            "forward" => {
                horizontal_position += val;
            }
            "down" => {
                depth += val;
            }
            "up" => {
                depth -= val;
            }
            _ => {}
        }
    }
    return horizontal_position * depth;
}
fn part2(lines: &Vec<&str>) -> i32 {
    let mut aim = 0;
    let mut horizontal_position = 0;
    let mut depth = 0;
    for line in lines {
        let split = line.split(" ").collect::<Vec<&str>>();
        let val = split[1].parse::<i32>().unwrap();
        match split[0] {
            "forward" => {
                horizontal_position += val;
                depth += aim * val
            }
            "down" => {
                aim += val;
            }
            "up" => {
                aim -= val;
            }
            _ => {}
        }
    }
    return horizontal_position * depth;
}
