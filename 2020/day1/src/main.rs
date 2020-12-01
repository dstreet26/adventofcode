use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file :(");
    let list: Vec<&str> = contents.split("\n").collect();
    // let theMatch = getMatch(list);
    // let (a, b) = getMatch(list);
    let (a, b, c) = getMatch3(list);
    // let (println!("{:?}", theMatch);
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
}

fn getMatch(list: Vec<&str>) -> (i32, i32) {
    for i in &list {
        for j in &list {
            let a = i.parse::<i32>().unwrap();
            let b = j.parse::<i32>().unwrap();

            if (a + b == 2020) {
                return (a, b);
            }
        }
    }
    (0, 0)
}
fn getMatch3(list: Vec<&str>) -> (i32, i32, i32) {
    for i in &list {
        for j in &list {
            for k in &list {
                let a = i.parse::<i32>().unwrap();
                let b = j.parse::<i32>().unwrap();
                let c = k.parse::<i32>().unwrap();

                if (a + b + c == 2020) {
                    return (a, b, c);
                }
            }
        }
    }
    (0, 0, 0)
}
