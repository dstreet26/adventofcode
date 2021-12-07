use std::fs;
extern crate nalgebra;
use nalgebra::DMatrix;

fn main() {
    let contents = fs::read_to_string("input0.txt").expect("Couldn't read input file :(");
    let lines = contents
        .split("\n")
        .map(|x| {
            x.chars()
                .map(|y| y.to_digit(10).unwrap_or(0))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let num_rows = lines.len();
    let num_cols = lines[0].len();
    let flat = lines.into_iter().flatten().collect::<Vec<u32>>();
    let dm = DMatrix::from_row_slice(num_rows, num_cols, &flat); //TODO: do this without flattening first
    println!("part1: {:?}", part1(&dm));
    println!("part2: {:?}", part2(&dm));
}
fn part1(dm: &DMatrix<u32>) -> u32 {
    let t = dm
        .row_sum()
        .map(|x| x > (dm.nrows().try_into().unwrap_or(0) / 2));
    let f = t.map(|x| !x);
    return vec_of_bool_to_value(t.as_slice()) * vec_of_bool_to_value(f.as_slice());
}
fn part2(dm: &DMatrix<u32>) -> u32 {
    let oxygen_rating = helper(&dm, 0, false);
    let co2_rating = helper(&dm, 0, true);
    return oxygen_rating * co2_rating;
}
fn helper(dm: &DMatrix<u32>, position: usize, flip: bool) -> u32 {
    if dm.nrows() == 1 {
        let vec = dm
            .as_slice()
            .iter()
            .map(|x| if *x == 1u32 { true } else { false })
            .collect::<Vec<bool>>();
        return vec_of_bool_to_value(&vec);
    } else {
        let column_sum = dm.column(position).sum();
        let majority = column_sum as f32 >= dm.nrows().try_into().unwrap_or(0) as f32 / 2.0;
        let majoritydec = if majority {
            if flip {
                0
            } else {
                1
            }
        } else {
            if flip {
                1
            } else {
                0
            }
        };
        let mut removalindices = vec![];
        for (i, elem) in dm.row_iter().enumerate() {
            if elem[position] != majoritydec {
                removalindices.push(i);
            }
        }
        let filtered = dm.clone().remove_rows_at(&removalindices);
        return helper(&filtered, position + 1, flip);
    }
}
//[true, false, true, true, false] = 22
fn vec_of_bool_to_value(vec: &[bool]) -> u32 {
    let mut sum: u32 = 0;
    for (i, x) in vec.iter().rev().enumerate() {
        if *x {
            sum += 2u32.pow(i as u32);
        }
    }
    sum
}
