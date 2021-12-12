use std::fs;
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file :(");
    let vector = contents
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let mut max = vector[0];
    let mut min = vector[0];
    for i in &vector {
        if i > &max {
            max = *i;
        }
        if i < &min {
            min = *i;
        }
    }
    println!("part1: {:?}", part1(&vector, min, max));
    println!("part2: {:?}", part2(&vector, min, max));
}
fn part1(vector: &Vec<i64>, min: i64, max: i64) -> i64 {
    let mut fuel_min = std::i64::MAX;
    let mut cheapest = 0;
    for i in min..=max {
        let mut fuel = 0;
        for v in vector {
            fuel += (v - i).abs() as i64;
        }
        if fuel < fuel_min {
            fuel_min = fuel;
            cheapest = fuel;
        }
    }

    return cheapest;
}
fn part2(vector: &Vec<i64>, min: i64, max: i64) -> i64 {
    let mut fuel_min = std::i64::MAX;
    let mut cheapest = 0;
    for i in min..=max {
        let mut fuel = 0;
        for v in vector {
            let diff = (v - i).abs();
            fuel += ((diff * (diff + 1)) / 2) as i64;
        }
        if fuel < fuel_min {
            fuel_min = fuel;
            cheapest = fuel;
        }
    }

    return cheapest;
}
