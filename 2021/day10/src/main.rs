use std::fs;
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file :(");
    let lines = contents
        .split("\n")
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    println!("part1: {:?}", part1(&lines));
    println!("part2: {:?}", part2(&lines));
}

fn is_valid(line: &Vec<char>) -> bool {
    let mut stack = vec![];
    for character in line {
        match character {
            '(' => stack.push(character),
            '[' => stack.push(character),
            '{' => stack.push(character),
            '<' => stack.push(character),
            ')' => match stack.pop() {
                Some('(') => (),
                _ => return false,
            },
            ']' => match stack.pop() {
                Some('[') => (),
                _ => return false,
            },
            '}' => match stack.pop() {
                Some('{') => (),
                _ => return false,
            },
            '>' => match stack.pop() {
                Some('<') => (),
                _ => return false,
            },
            _ => (),
        }
    }
    return true;
}

fn part2(lines: &Vec<Vec<char>>) -> u64 {
    let mut completion_scores = vec![];
    let lines_filtered = lines
        .iter()
        .filter(|x| is_valid(x))
        .collect::<Vec<&Vec<char>>>();
    for line in lines_filtered {
        let mut stack = vec![];
        for character in line {
            match character {
                '(' => stack.push(character),
                '[' => stack.push(character),
                '{' => stack.push(character),
                '<' => stack.push(character),
                ')' => match stack.pop() {
                    Some('(') => (),
                    _ => (),
                },
                ']' => match stack.pop() {
                    Some('[') => (),
                    _ => (),
                },
                '}' => match stack.pop() {
                    Some('{') => (),
                    _ => (),
                },
                '>' => match stack.pop() {
                    Some('<') => (),
                    _ => (),
                },
                _ => (),
            }
        }
        stack.reverse();

        let mut completion_score: i64 = 0;
        for character in stack {
            completion_score *= 5;
            match character {
                '(' => completion_score += 1,
                '[' => completion_score += 2,
                '{' => completion_score += 3,
                '<' => completion_score += 4,
                _ => (),
            }
        }
        completion_scores.push(completion_score);
    }
    completion_scores.sort();
    let middle = completion_scores.len() / 2;
    let middle_score = completion_scores[middle];

    return middle_score as u64;
}

fn part1(lines: &Vec<Vec<char>>) -> u64 {
    let mut sum = 0;
    for line in lines {
        let mut out = vec![];
        for character in line {
            match character {
                '(' => out.push(character),
                '[' => out.push(character),
                '{' => out.push(character),
                '<' => out.push(character),
                ')' => match out.pop() {
                    Some('(') => (),
                    Some(_) => {
                        sum += 3;
                        break;
                    }
                    None => {
                        break;
                    }
                },
                ']' => match out.pop() {
                    Some('[') => (),
                    Some(_) => {
                        sum += 57;
                        break;
                    }
                    None => {
                        break;
                    }
                },
                '}' => match out.pop() {
                    Some('{') => (),
                    Some(_) => {
                        sum += 1197;
                        break;
                    }
                    None => {
                        break;
                    }
                },
                '>' => match out.pop() {
                    Some('<') => (),
                    Some(_) => {
                        sum += 25137;
                        break;
                    }
                    None => {
                        break;
                    }
                },
                _ => (),
            }
        }
    }
    return sum;
}
