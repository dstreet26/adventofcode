use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file :(");
    let list: Vec<&str> = contents.split("\n\n").collect();
    let mut count = 0;

    for i in &list {
        let trimmed = i.replace("\n", " ");
        let list2: Vec<&str> = trimmed.split(" ").collect();
        let requirements: HashSet<&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .cloned()
            .collect();
        let mut lefters = HashSet::new();
        for j in &list2 {
            let split2: Vec<&str> = j.split(":").collect();
            let leftleft = split2[0];
            let right = split2[1];
            let right_i = right.parse::<i32>().unwrap_or(0);
            if leftleft == "byr" {
                if right_i >= 1920 && right_i <= 2002 {
                    lefters.insert(leftleft);
                }
            }
            // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
            if leftleft == "iyr" {
                if right_i >= 2010 && right_i <= 2020 {
                    lefters.insert(leftleft);
                }
            }
            // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
            if leftleft == "eyr" {
                if right_i >= 2020 && right_i <= 2030 {
                    lefters.insert(leftleft);
                }
            }
            // hgt (Height) - a number followed by either cm or in:
            if leftleft == "hgt" {
                let unit = right
                    .chars()
                    .rev()
                    .take(2)
                    .collect::<Vec<char>>()
                    .iter()
                    .rev()
                    .cloned()
                    .collect::<String>();

                let newheight = right
                    .replace("in", "")
                    .replace("cm", "")
                    .parse::<i32>()
                    .unwrap_or(0);
                //If in, the number must be at least 59 and at most 76.
                if unit == "in" {
                    if newheight >= 59 && newheight <= 76 {
                        lefters.insert(leftleft);
                    }
                } else if unit == "cm" {
                    //If cm, the number must be at least 150 and at most 193.
                    if newheight >= 150 && newheight <= 193 {
                        lefters.insert(leftleft);
                    }
                }
            }
            // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
            if leftleft == "hcl" {
                if right.chars().nth(0).unwrap() == '#' {
                    let asdf2 = &right[1..];
                    if asdf2.len() == 6 {
                        if asdf2.chars().all(|x| x.is_numeric() || x.is_alphabetic()) {
                            lefters.insert(leftleft);
                        }
                    }
                }
            }
            // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
            if leftleft == "ecl" {
                if right == "amb"
                    || right == "blu"
                    || right == "brn"
                    || right == "gry"
                    || right == "grn"
                    || right == "hzl"
                    || right == "oth"
                {
                    lefters.insert(leftleft);
                }
            }
            // pid (Passport ID) - a nine-digit number, including leading zeroes.
            if leftleft == "pid" {
                if right.len() == 9 {
                    if right.chars().all(|x| x.is_numeric()) {
                        lefters.insert(leftleft);
                    }
                }
            }
        }
        let is_good = lefters.is_superset(&requirements);

        if is_good {
            count = count + 1;
        }
    }
    println!("count: {}", count);
}
