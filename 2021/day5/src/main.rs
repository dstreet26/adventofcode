use std::collections::HashMap;
use std::fs;
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file :(");
    let vectors = contents
        .split("\n")
        .map(|x| {
            x.split(" -> ")
                .map(|y| {
                    let a = y
                        .split(",")
                        .map(|z| z.parse::<i32>().unwrap_or(0))
                        .collect::<Vec<i32>>();
                    (a[0], a[1])
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .collect::<Vec<Vec<(i32, i32)>>>();
    println!("part1: {:?}", part1(&vectors));
    println!("part2: {:?}", part2(&vectors));
}
fn sumup(dict: &HashMap<(i32, i32), i32>) -> i32 {
    let mut out = 0;
    for (_, value) in dict {
        if value > &1 {
            out += 1;
        }
    }
    return out;
}
fn part1(vectors: &Vec<Vec<(i32, i32)>>) -> i32 {
    let mut dict: HashMap<(i32, i32), i32> = HashMap::new();
    for vector in vectors {
        let x0 = vector[0].0;
        let y0 = vector[0].1;
        let x1 = vector[1].0;
        let y1 = vector[1].1;
        if x0 == x1 {
            let mut min = y0.min(y1);
            let max = y0.max(y1);
            while min <= max {
                dict.insert((x1, min), *dict.get(&(x1, min)).unwrap_or(&0) + 1);
                min += 1;
            }
        } else if y0 == y1 {
            let mut min = x0.min(x1);
            let max = x0.max(x1);
            while min <= max {
                dict.insert((min, y1), *dict.get(&(min, y1)).unwrap_or(&0) + 1);
                min += 1;
            }
        }
    }
    return sumup(&dict);
}
fn part2(vectors: &Vec<Vec<(i32, i32)>>) -> i32 {
    let mut dict: HashMap<(i32, i32), i32> = HashMap::new();
    for vector in vectors {
        let x0 = vector[0].0;
        let y0 = vector[0].1;
        let x1 = vector[1].0;
        let y1 = vector[1].1;

        if x0 == x1 {
            let mut min = y0.min(y1);
            let max = y0.max(y1);
            while min <= max {
                dict.insert((x1, min), *dict.get(&(x1, min)).unwrap_or(&0) + 1);
                min += 1;
            }
        } else if y0 == y1 {
            let mut min = x0.min(x1);
            let max = x0.max(x1);
            while min <= max {
                dict.insert((min, y1), *dict.get(&(min, y1)).unwrap_or(&0) + 1);
                min += 1;
            }
        } else {
            let x_diff = x1 - x0;
            let y_diff = y1 - y0;
            let mut x_inc = 1;
            let mut y_inc = 1;
            let mut x = x0;
            let mut y = y0;
            if x_diff < 0 {
                x_inc = -1;
            }
            if y_diff < 0 {
                y_inc = -1;
            }
            for _ in 0..x_diff.abs() + 1 {
                dict.insert((x, y), *dict.get(&(x, y)).unwrap_or(&0) + 1);
                x += x_inc;
                y += y_inc;
            }
        }
    }
    return sumup(&dict);
}
