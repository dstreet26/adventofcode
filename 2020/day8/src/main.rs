use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file :(");
    let list: Vec<&str> = contents.split("\n").collect();
    let mut executed = HashSet::new();

    let mut accumulator = 0;
    let mut i = 0;
    loop {
        let element = list[i];
        if executed.contains(&i) {
            println!("HALT");
            println!("{:?}", accumulator);
            break;
        }
        executed.insert(i);
        // println!("{:?}", i);
        let leftright: Vec<&str> = element.split(" ").collect();
        // println!("{:?}", leftright);
        let opcode = leftright[0];
        let value: i32 = leftright[1].parse().unwrap_or(0);
        match opcode {
            "jmp" => {
                if value > 0 {
                    i = i + (value.abs() as usize)
                } else {
                    i = i - (value.abs() as usize)
                }
            }
            "nop" => {
                i = i + 1;
            }
            "acc" => {
                i = i + 1;
                accumulator = accumulator + value;
            }

            _ => println!("oopsie"),
        }
        // println!("acc{:?}", accumulator);
    }
}
