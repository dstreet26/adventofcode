use std::fs;

struct Data {
    place: i32,
    input: i32,
    sum: i32,
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file :(");
    let lines = contents.split("\n").collect::<Vec<&str>>();
    //take the first line
    let inputline = lines[0];
    //split the line by comma into numbers
    let inputnumbers = inputline
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    //To parse bingotables, we rejoin the numbers and then re-split them by two newlines
    let recombined = lines.into_iter().skip(2).collect::<Vec<&str>>().join("\n");
    let bingolines = recombined.split("\n\n").collect::<Vec<&str>>();
    let bingotables = bingolines
        .into_iter()
        .map(|x| x.split("\n").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()
        .into_iter()
        .map(|x| {
            x.into_iter()
                .map(|y| {
                    y.replace("  ", " ")
                        .trim()
                        .split(" ")
                        .map(|z| z.trim().parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>()
        })
        .collect::<Vec<Vec<Vec<i32>>>>();

    //Make a copy and set all the values to 0
    let mut bingotables_zeroed = bingotables.clone();
    for i in 0..bingotables_zeroed.len() {
        for j in 0..bingotables_zeroed[i].len() {
            for k in 0..bingotables_zeroed[i][j].len() {
                bingotables_zeroed[i][j][k] = 0;
            }
        }
    }

    //For each table, go through each input value and set a value of 1 in the cloned table, and then check if it's a winner and push the results to an output.
    let mut output = vec![];
    for (i, bingotable) in bingotables.iter().enumerate() {
        for (z, inputnumber) in inputnumbers.iter().enumerate() {
            for (j, bingorow) in bingotable.iter().enumerate() {
                for (k, number) in bingorow.iter().enumerate() {
                    if &number == &inputnumber {
                        bingotables_zeroed[i][j][k] = 1;
                    }
                }
            }
            if check(&bingotables_zeroed[i]) {
                let mut sum = 0;
                for (j, bingorow) in bingotables[i].iter().enumerate() {
                    for (k, number) in bingorow.iter().enumerate() {
                        let asdf = bingotables_zeroed[i][j][k];
                        if asdf == 0 {
                            sum += number;
                        }
                    }
                }
                output.push(Data {
                    place: z as i32,
                    input: *inputnumber,
                    sum: sum,
                });
                break;
            }
        }
    }
    //sort output by place
    output.sort_by(|a, b| a.place.cmp(&b.place));
    //print the output, the answer is the first or last one depending on the part
    for item in output {
        println!(
            "place:{:?},input:{:?},sum:{:?},answer:{:?}",
            item.place,
            item.input,
            item.sum,
            item.sum * item.input
        );
    }
}
fn check(table: &Vec<Vec<i32>>) -> bool {
    for row in table {
        let mut rowsum = 0;
        for item in row {
            rowsum += item;
        }
        if rowsum == 5 {
            return true;
        }
    }
    for col in 0..table[0].len() {
        let mut colsum = 0;
        for row in 0..table.len() {
            colsum += table[row][col];
        }
        if colsum == 5 {
            return true;
        }
    }
    return false;
}
