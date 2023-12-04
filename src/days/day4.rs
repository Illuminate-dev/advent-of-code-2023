use std::io::BufRead;

pub fn run() {
    let reader = super::open_file("input/day4.txt");

    let mut first_ans = 0 as u64;

    let lines = reader
        .lines()
        .map(|s| s.expect("failed to read line"))
        .collect::<Vec<_>>();

    for line in lines.iter() {
        let card = Card::parse(line);
        first_ans += card.score() as u64;
    }

    let mut copies = vec![1; lines.len()];

    for (i, line) in lines.iter().enumerate() {
        let card = Card::parse(line);
        let matches = card.num_matches();
        for idx in i + 1..i + matches as usize + 1 {
            if idx < lines.len() {
                copies[idx] += copies[i];
            }
        }
    }

    let second_ans = copies.into_iter().sum::<u32>();

    println!("{}", first_ans);
    println!("{}", second_ans);
}

struct Card {
    winning_numbers: Vec<u32>,
    current_numbers: Vec<u32>,
}

impl Card {
    fn parse(s: &str) -> Self {
        let two_lists = s
            .trim_start_matches("Card ")
            .split_whitespace()
            .skip(1)
            .collect::<Vec<_>>()
            .join(" ");
        let two_lists = two_lists.split(" | ").collect::<Vec<_>>();

        let winning_numbers = two_lists[0]
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let current_numbers = two_lists[1]
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        Self {
            winning_numbers,
            current_numbers,
        }
    }

    fn score(&self) -> u32 {
        let matches = self.num_matches();

        if matches > 0 {
            return (2 as u32).pow(matches - 1);
        }
        0
    }

    fn num_matches(&self) -> u32 {
        self.current_numbers.iter().fold(0, |acc, &n| {
            if self.winning_numbers.contains(&n) {
                acc + 1
            } else {
                acc
            }
        })
    }
}

#[cfg(test)]
mod tests {}
