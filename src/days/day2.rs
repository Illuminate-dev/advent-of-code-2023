use std::io::BufRead;

pub fn run() {
    let reader = super::open_file("input/day2.txt");

    let mut ans = 0 as u64;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let game = Game::parse(&line);

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for set in game.sets {
            max_red = max_red.max(set.red);

            max_green = max_green.max(set.green);

            max_blue = max_blue.max(set.blue);
        }

        ans += (max_red * max_green * max_blue) as u64;
    }

    println!("{}", ans);
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    fn parse(s: &str) -> Self {
        let s = s.trim_start_matches("Game ");

        let id = s
            .split_whitespace()
            .take(1)
            .next()
            .unwrap()
            .trim_end_matches(":")
            .parse::<u32>()
            .unwrap();

        let mut sets = Vec::new();

        let data = s.split_whitespace().skip(1).collect::<Vec<_>>().join(" ");

        for set in data.split("; ") {
            sets.push(Set::parse(set));
        }

        Game { id, sets }
    }
}

#[derive(Debug, PartialEq)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl Set {
    fn parse(s: &str) -> Self {
        let data = s.split(", ").collect::<Vec<_>>();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for info in data {
            let mut info = info.split_whitespace();

            let num = info.next().unwrap().parse::<u32>().unwrap();
            let color = info.next().unwrap();

            match color {
                "red" => red = num,
                "green" => green = num,
                "blue" => blue = num,
                _ => panic!("Invalid color"),
            }
        }

        Set { red, green, blue }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game() {
        let s = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";

        let game = Game::parse(s);

        assert_eq!(
            game,
            Game {
                id: 3,
                sets: vec![
                    Set {
                        red: 20,
                        green: 8,
                        blue: 6
                    },
                    Set {
                        blue: 5,
                        red: 4,
                        green: 13
                    },
                    Set {
                        green: 5,
                        red: 1,
                        blue: 0
                    }
                ]
            }
        )
    }
}
