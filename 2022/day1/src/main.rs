use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file :(");

    let totals_sorted = get_totals_sorted(&contents);

    let first = totals_sorted[0];
    println!("Part 1: {:?}", first);

    let mut sum = 0;
    for i in 0..3 {
        sum += totals_sorted[i];
    }
    println!("Part 2: {:?}", sum);
}

fn get_totals_sorted(input: &str) -> Vec<i32> {
    let mut groups: Vec<i32> = Vec::new();
    let mut group: Vec<i32> = Vec::new();
    for line in input.lines() {
        if line == "" {
            groups.push(get_group_total(&group));
            group = Vec::new();
        } else {
            group.push(line.parse::<i32>().unwrap());
        }
    }
    groups.push(get_group_total(&group));

    groups.sort_by(|a, b| b.cmp(a));
    groups
}

fn get_group_total(group: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for num in group {
        sum += num;
    }
    sum
}
