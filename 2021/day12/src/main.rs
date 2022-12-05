use std::collections::HashMap;
use std::fs;
fn main() {
    let contents = fs::read_to_string("input0.txt").expect("Couldn't read input file :(");
    // let mut connections = contents
    //     .split("\n")
    //     .map(|line| {
    //         let parts = line.split("-").collect::<Vec<&str>>();
    //         return (parts[0], parts[1]);
    //     })
    //     .collect::<Vec<(&str, &str)>>();
    let mut connections = contents
        .split("\n")
        .map(|line| {
            line.split("-").collect::<Vec<&str>>()
            // return (parts[0], parts[1]);
        })
        .collect::<Vec<Vec<&str>>>();

    // println!("lines: {:?}", lines);

    println!("lines: {:?}", connections);

    //create a digraph from the pairs in the lines
    let mut dict: HashMap<&str, Vec<&str>> = HashMap::new();

    //create a new empty hashmap
    // let dict: HashMap<String, String> = HashMap::new();
    // let mut dict: HashMap<&str, &str> = HashMap::new();
    let defaultarray :Vec<&str> = Vec::new();
    // let defaultarray2 :Vec<&str> = Vec::new();
    for c in connections {
        // dict.insert("k: K", "v: V");
        let left = dict.get(c[0]).unwrap_or(&defaultarray);
        let right = dict.get(c[1]).unwrap_or(&defaultarray);
        let mut new_left = left.clone();
        new_left.push(c[1]);
        let mut new_right = right.clone();
        new_right.push(c[0]);
        dict.insert(c[0], new_left);
        dict.insert(c[1], new_right);
        // left.push(c[1]);
        // right.push(c[0]);
        // dict.insert(c[0], &left);
        // dict.insert(c[1], right.clone());
    }
    println!("dict: {:?}", dict);

    // using the dict as a digraph, do a bfs to find all the valid paths from start to end
    let mut visited: HashMap<&str, bool> = HashMap::new();
    let mut queue: Vec<&str> = Vec::new();
    let mut path: Vec<&str> = Vec::new();
    let mut paths: Vec<Vec<&str>> = Vec::new();
    let mut path_count = 0;
    let mut path_length = 0;
    let mut max_path_length = 0;
    let mut max_path_count = 0;
    let mut max_path: Vec<&str> = Vec::new();
    let mut max_paths: Vec<Vec<&str>> = Vec::new();
    let mut max_path_lengths: Vec<usize> = Vec::new();
    let mut max_path_lengths_counts: Vec<usize> = Vec::new();
    let mut max_path_lengths_counts_max_index = 0;
    let mut max_path_lengths_counts_max_value = 0;
    let mut max_path_lengths_counts_max_value_index = 0;
    

}
