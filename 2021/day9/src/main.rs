use std::fs;
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file :(");
    let matrix = contents
        .split("\n")
        .map(|x| {
            x.chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    println!("matrix: {:?}", matrix);
    let mut sum = 0;
    for (i, row) in matrix.iter().enumerate() {
        for (j, col) in matrix[i].iter().enumerate() {
            let thisvalue = matrix[i][j];
            let mut top_neighbor = 9;
            let mut bottom_neighbor = 9;
            let mut left_neighbor = 9;
            let mut right_neighbor = 9;
            if i > 0 {
                top_neighbor = matrix[i - 1][j];
            }
            if i < matrix.len() - 1 {
                bottom_neighbor = matrix[i + 1][j];
            }
            if j > 0 {
                left_neighbor = matrix[i][j - 1];
            }
            if j < matrix[i].len() - 1 {
                right_neighbor = matrix[i][j + 1];
            }
            if thisvalue < top_neighbor
                && thisvalue < bottom_neighbor
                && thisvalue < left_neighbor
                && thisvalue < right_neighbor
            {
                println!("yes {}", thisvalue);
                sum += thisvalue + 1;
            }
        }
    }
    println!("sum: {}", sum);
}
