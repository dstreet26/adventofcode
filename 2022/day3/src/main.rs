use std::fs;
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file :(");
    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
}

fn part1(contents: &str) -> i32 {
    let mut sum = 0;

    for line in contents.lines() {
        let midpoint = line.len() / 2;
        let (first_half, second_half) = line.split_at(midpoint);

        let common_letter = find_common_letter(first_half, second_half);

        let value = lookup_value(common_letter);

        sum += value;
    }

    sum
}

fn find_common_letter(first_half: &str, second_half: &str) -> char {
    for (i, c) in first_half.chars().enumerate() {
        for (j, d) in second_half.chars().enumerate() {
            if c == d {
                return c;
            }
        }
    }
    '1'
}

fn part2(contents: &str) -> i32 {
    let mut sum = 0;

    let mut lines = contents.lines();

    while let Some(line1) = lines.next() {
        let line2 = lines.next().unwrap();
        let line3 = lines.next().unwrap();

        let common_letter = find_common_letter2(&line1, &line2, &line3);

        let value = lookup_value(common_letter);

        sum += value;
    }

    sum
}

fn find_common_letter2(a: &str, b: &str, c: &str) -> char {
    let mut common_letter = ' ';

    for letter in a.chars() {
        if b.contains(letter) && c.contains(letter) {
            common_letter = letter;
            break;
        }
    }

    common_letter
}

fn lookup_value(letter: char) -> i32 {
    let mut value = 0;
    if letter.is_lowercase() {
        value = letter as i32 - 96;
    } else if letter.is_uppercase() {
        value = letter as i32 - 64 + 26;
    }
    value
}
