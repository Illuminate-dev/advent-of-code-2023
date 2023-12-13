use std::io::BufRead;

pub fn run() {
    let reader = super::open_file("input/day6.txt");

    let mut first_ans = 1 as u64;

    let lines = reader
        .lines()
        .map(|s| s.expect("failed to read line"))
        .collect::<Vec<_>>();

    let times = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i32>().expect("failed to parse number"))
        .collect::<Vec<_>>();
    let distances = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i32>().expect("failed to parse number"))
        .collect::<Vec<_>>();

    for (time, distance) in times.iter().zip(distances.iter()) {
        let mut num_ways = 0;
        for i in 0..=*time {
            let distance_traveled = i * (time - i);
            if distance_traveled >= *distance {
                num_ways += 1;
            }
        }
        first_ans *= num_ways as u64;
    }
    println!("{first_ans}");

    let mut second_ans = 0 as u64;

    let time = lines[0]
        .split_whitespace()
        .skip(1)
        .fold(String::new(), |acc, e| acc + e)
        .parse::<u128>()
        .expect("failed to parse number");

    let distance = lines[1]
        .split_whitespace()
        .skip(1)
        .fold(String::new(), |acc, e| acc + e)
        .parse::<u128>()
        .expect("failed to parse number");

    for i in 0..=time {
        let distance_traveled = i * (time - i);
        if distance_traveled >= distance {
            second_ans += 1;
        }
    }
    println!("{second_ans}");
}
