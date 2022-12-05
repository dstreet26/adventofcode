//definitions
// A means player1 played rock
// B means player1 played paper
// C means player1 played scissors
// X means player2 played rock
// Y means player2 played paper
// Z means player2 played scissors

// part1 mapping
// A Y
// B X
// C Z

// part2
// X means player2 needs to lose
// Y means player2 needs to win
// Z means player2 needs to draw

use std::fs;
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file :(");

    let mut contents_list = Vec::new();
    for line in contents.lines() {
        let mut line_list = Vec::new();
        for c in line.chars() {
            if c != ' ' {
                line_list.push(c);
            }
        }
        contents_list.push(line_list);
    }

    let part1a = part1(&contents_list);

    println!("part 1: {}", part1a);

    let mut contents_list2 = Vec::new();
    for line in contents_list {
        let mut line_list = Vec::new();
        let a = line[0];
        line_list.push(a);
        let b = line[1];

        match a {
            'A' => {
                if b == 'X' {
                    line_list.push('Z');
                } else if b == 'Y' {
                    line_list.push('X');
                } else if b == 'Z' {
                    line_list.push('Y');
                }
            }
            'B' => {
                if b == 'X' {
                    line_list.push('X');
                } else if b == 'Y' {
                    line_list.push('Y');
                } else if b == 'Z' {
                    line_list.push('Z');
                }
            }
            'C' => {
                if b == 'X' {
                    line_list.push('Y');
                } else if b == 'Y' {
                    line_list.push('Z');
                } else if b == 'Z' {
                    line_list.push('X');
                }
            }
            _ => {}
        }

        contents_list2.push(line_list);
    }

    println!("part 2: {}", part1(&contents_list2));
}

fn part1(input: &Vec<Vec<char>>) -> usize {
    let mut score = 0;
    for line in input {
        let a = line[0];
        let b = line[1];
        if b == 'X' {
            score += 1;
        } else if b == 'Y' {
            score += 2;
        } else if b == 'Z' {
            score += 3;
        }

        if a == 'A' {
            if b == 'X' {
                score += 3;
            } else if b == 'Y' {
                score += 6;
            } else {
                score += 0;
            }
        } else if a == 'B' {
            if b == 'X' {
                score += 0;
            } else if b == 'Y' {
                score += 3;
            } else {
                score += 6;
            }
        } else {
            if b == 'X' {
                score += 6;
            } else if b == 'Y' {
                score += 0;
            } else {
                score += 3;
            }
        }
    }
    score
}

