use std::fs;
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file :(");
    let mut matrix = contents
        .split("\n")
        .map(|x| {
            x.chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut flash_count = 0;

    for i in 0..1000 {
        let mut all_zero = true;
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] != 0 {
                    all_zero = false;
                }
            }
        }
        if all_zero {
            println!("step: {}", i);
            break;
        }
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                matrix[i][j] += 1;
            }
        }
        let mut flashed = vec![];
        let mut flashing = true;
        while flashing {
            flashing = false;
            for i in 0..matrix.len() {
                for j in 0..matrix[i].len() {
                    if matrix[i][j] > 9 {
                        matrix[i][j] = 0;
                        flash_count += 1;
                        flashed.push((i, j));
                        if i > 0 && j > 0 {
                            matrix[i - 1][j - 1] += 1;
                        }
                        if i > 0 {
                            matrix[i - 1][j] += 1;
                        }
                        if i > 0 && j < matrix[i].len() - 1 {
                            matrix[i - 1][j + 1] += 1;
                        }
                        if j > 0 {
                            matrix[i][j - 1] += 1;
                        }
                        if j < matrix[i].len() - 1 {
                            matrix[i][j + 1] += 1;
                        }
                        if i < matrix.len() - 1 && j > 0 {
                            matrix[i + 1][j - 1] += 1;
                        }
                        if i < matrix.len() - 1 {
                            matrix[i + 1][j] += 1;
                        }
                        if i < matrix.len() - 1 && j < matrix[i].len() - 1 {
                            matrix[i + 1][j + 1] += 1;
                        }

                        flashing = true;
                    }
                }
            }
        }
        for i in 0..flashed.len() {
            matrix[flashed[i].0][flashed[i].1] = 0;
        }
    }
    println!("flash_count: {:?}", flash_count);
}
