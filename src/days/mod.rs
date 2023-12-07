pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

use std::{fs::File, io::BufReader};

fn open_file(filename: &str) -> BufReader<File> {
    let file = File::open(filename).expect("Failed to open input");
    BufReader::new(file)
}
