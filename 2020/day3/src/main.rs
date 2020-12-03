use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file :(");
    let lines: Vec<&str> = contents.split("\n").collect();
    let first_line = lines[0];
    let first_line_length = first_line.len();

    let run1 = get_count(lines.clone(), 1, 1, first_line_length);
    let run2 = get_count(lines.clone(), 1, 3, first_line_length);
    let run3 = get_count(lines.clone(), 1, 5, first_line_length);
    let run4 = get_count(lines.clone(), 1, 7, first_line_length);
    let run5 = get_count(lines.clone(), 2, 1, first_line_length);

    println!("Right 1, down 1. {}", run1);
    println!("Right 3, down 1. {}", run2);
    println!("Right 5, down 1. {}", run3);
    println!("Right 7, down 1. {}", run4);
    println!("Right 1, down 2. {}", run5);

    println!("Final {}", run1 * run2 * run3 * run4 * run5);
}

fn get_count(lines: Vec<&str>, down: usize, right: usize, width: usize) -> usize {
    let mut count = 0;
    let mut is_first = true;
    let mut x = 0;

    for line in (0..lines.len()).step_by(down) {
        if is_first {
            is_first = false;
        } else {
            let val = &lines[line].chars().nth(x).unwrap();
            if *val == '#' {
                count = count + 1;
            }
        }
        x = x + right;
        if x >= width {
            x = x - width
        }
    }
    count
}
