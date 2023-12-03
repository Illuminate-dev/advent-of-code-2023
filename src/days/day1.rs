use std::io::BufRead;

use crate::days::open_file;

const DIGITS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

pub fn run() {
    let reader = open_file("input/day1.txt");

    let mut ans = 0 as i64;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut nums = Vec::new();

        let mut i = 0;
        while i < line.len() {
            match read_num(&line, &mut i) {
                Some(num) => nums.push(num),
                None => (),
            }
        }

        let num = nums.first().unwrap() * 10 + nums.last().unwrap();
        ans += num as i64;
    }

    println!("{}", ans);
}

fn read_num(line: &str, i: &mut usize) -> Option<u32> {
    match line.chars().nth(*i) {
        Some(c) => match c.to_digit(10) {
            Some(d) => {
                *i += 1;
                Some(d)
            }
            None => {
                for (s, d) in DIGITS.iter() {
                    if line[*i..].starts_with(s) {
                        *i += 1;
                        return Some(*d);
                    }
                }
                *i += 1;
                None
            }
        },
        None => None,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_read_num() {
        let line = "44onetwo2threefourfivesixseveneight8nine";

        let mut i = 0;

        let mut nums = Vec::new();

        while i < line.len() {
            match super::read_num(&line, &mut i) {
                Some(num) => nums.push(num),
                None => (),
            }
        }

        assert_eq!(nums, vec![4, 4, 1, 2, 2, 3, 4, 5, 6, 7, 8, 8, 9]);
    }

    #[test]
    fn test_unique() {
        let line = "oneight";

        let mut i = 0;

        let mut nums = Vec::new();

        while i < line.len() {
            match super::read_num(&line, &mut i) {
                Some(num) => nums.push(num),
                None => (),
            }
        }

        assert_eq!(nums, vec![1, 8]);
    }
}
