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
    let contents = fs::read_to_string("input0.txt").expect("Couldn't read input file :(");

    // turn the input into a list of tuples

    let data = contents
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let a = iter.next().unwrap();
            let b = iter.next().unwrap();
            (a, b)
        })
        .collect::<Vec<_>>();

    let part1a = part1(data.clone());

    println!("part 1: {}", part1a);

    // we can re-use the part1 function to do part2
    // maybe we can just pre-process the data to make it look like part1
    let newdata = data
        .iter()
        .map(|(a, b)| {
            // let newa = match a {
            //     // &"A" => "Y",
            //     &"A" => asdf("A", b),
            //     &"B" => "X",
            //     &"C" => "Z",
            //     _ => panic!("invalid input"),
            // };
            // (newa, b)
            asdf(a, b)
            
        })
        .collect::<Vec<_>>();
    
    let part2 = part1(newdata);

    println!("part 2: {}", part2);
}

fn asdf(a: &str, b: &str) -> bool {
    match (a, b) {
        ("A", "Y") => true,
        ("B", "X") => true,
        ("C", "Z") => true,
        _ => false,
    }
}

fn part1(input: Vec<(&str, &str)>) -> i32 {
    let mut score = 0;
    for (a, b) in input {
        if b == "X" {
            score += 1;
        } else if b == "Y" {
            score += 2;
        } else if b == "Z" {
            score += 3;
        }

        if a == "A" {
            if b == "X" {
                score += 3;
            } else if b == "Y" {
                score += 6;
            } else {
                score += 0;
            }
        } else if a == "B" {
            if b == "X" {
                score += 0;
            } else if b == "Y" {
                score += 3;
            } else {
                score += 6;
            }
        } else {
            if b == "X" {
                score += 6;
            } else if b == "Y" {
                score += 0;
            } else {
                score += 3;
            }
        }
    }
    score
}


