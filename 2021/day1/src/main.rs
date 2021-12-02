use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file :(");

    let nums: Vec<i32> = contents
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    println!("part1: {:?}", part1(&nums));
    println!("part2: {:?}", part2(&nums));
}
fn part1(nums: &Vec<i32>) -> i32 {
    return nums
        .windows(2)
        .map(|x| if x[1] > x[0] { 1 } else { 0 })
        .sum();
}
fn part2(nums: &Vec<i32>) -> i32 {
    return nums
        .windows(3)
        .map(|x| x[0] + x[1] + x[2])
        .collect::<Vec<i32>>()
        .windows(2)
        .map(|x| if x[1] > x[0] { 1 } else { 0 })
        .sum();

    // Improvement: use part1 to help with part2
    // return part1(
    //     &nums
    //         .windows(3)
    //         .map(|x| x[0] + x[1] + x[2])
    //         .collect::<Vec<i32>>(),
    // );
}
