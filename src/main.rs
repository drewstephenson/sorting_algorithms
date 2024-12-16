#![allow(dead_code)]

use std::env;

use sorting_algorithms::{Options, sort};


fn main() {
    let args: Vec<String> = env::args().collect();
    let size = match args[1].parse::<usize>() {  // Get the vector size from the first argument (args[0] is the name of the program) and convert it
        Ok(size) => size,
        Err(_) => {
            eprintln!("Please provide a positive integer for the size of the vector.");
            return;
        }
    };

    let algorithm = args[2].clone();  // Get the algorithm type from the second argument
    let options = Options::build(size, algorithm);
    println!("Options: {:?}", options);

    // Apply the sorting algorithm.
    sort(options);

}
