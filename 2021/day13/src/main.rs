use std::collections::HashMap;
use std::fs;
fn main() {
    let contents = fs::read_to_string("input0.txt").expect("Couldn't read input file :(");
    let parts = contents.split("\n\n").collect::<Vec<&str>>();
    let points = parts[0]
        .split("\n")
        .map(|line| {
            let parts = line.split(",").collect::<Vec<&str>>();

            return (
                parts[0].parse::<u32>().unwrap(),
                parts[1].parse::<u32>().unwrap(),
            );
        })
        .collect::<Vec<(u32, u32)>>();
    println!("parts: {:?}", parts);
    println!("points: {:?}", points);
    let commands = parts[1]
        .split("\n")
        .map(|line| {
            let parts = line.split("=").collect::<Vec<&str>>();

            return (
                parts[0].replace("fold along ", ""),
                parts[1].parse::<u32>().unwrap(),
            );
        })
        .collect::<Vec<(String, u32)>>();

    println!("commands: {:?}", commands);


    // get the min and max x and y values from the points
    let mut min_x = points[0].0;
    let mut max_x = points[0].0;
    let mut min_y = points[0].1;
    let mut max_y = points[0].1;
    for point in &points {
        if point.0 < min_x {
            min_x = point.0;
        }
        if point.0 > max_x {
            max_x = point.0;
        }
        if point.1 < min_y {
            min_y = point.1;
        }
        if point.1 > max_y {
            max_y = point.1;
        }
    }
    //build an empty matrix from 0 to the max x and y values
    let mut matrix = vec![vec![0; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];

    println!("matrix: {:?}", matrix);
    // for each point, set the value in the matrix to the point's index
    for (i, point) in points.iter().enumerate() {
        matrix[(point.1 - min_y) as usize][(point.0 - min_x) as usize] = 1;
    }
    println!("matrix: {:?}", matrix);
    

    //get the first command
    let first_command = &commands[0].0;
    println!("first_command: {:?}", first_command);
}

//takes a matrix and a value and folds the matrix up onto itself along the horizontal axis
fn foldUp(matrix: Vec<Vec<&str>>, value: u32) -> Vec<Vec<&str>> {
    // let mut new_matrix = matrix.clone();
    // for row in matrix {
    //     let mut new_row = vec![];
    //     for i in 0..row.len() {
    //         if i == 0 {
    //             new_row.push(row[i]);
    //         } else {
    //             new_row.push(row[i].to_string());
    //         }
    //     }
    //     new_matrix.push(new_row);
    // }
    // return new_matrix;

    //split the matrix into two halves, splitting at the value
    let mut top_half = matrix.clone();
    let mut bottom_half = matrix.clone();
    
}



// //takes a matrix and a value and folds the matrix left onto itself along the vertical axis
// fn foldLeft(matrix: Vec<Vec<&str>>, value: u32) -> Vec<Vec<&str>> {
//     let mut new_matrix = vec![];
//     for i in 0..matrix.len() {
//         for j in 0..matrix[i].len() {
//             if matrix[i][j] == value.to_string() {
//             } else {
//             }
//         }
//     }
//     return new_matrix;
// }
