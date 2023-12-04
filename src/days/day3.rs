use std::{collections::HashSet, io::BufRead};

const OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn run() {
    let reader = super::open_file("input/day3.txt");

    let mut first_ans = 0 as u64;
    let mut second_ans = 0 as u64;

    let lines = reader
        .lines()
        .map(|x| x.expect("failed to read line").chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for i in 0..lines.len() {
        let mut c = 0;
        while c < lines[i].len() {
            if is_number(lines[i][c]) {
                let (num, len) = parse_number(i, c, &lines);
                if in_bounds(i, c, len, &lines) {
                    first_ans += num;
                }
                c += len;
            } else {
                c += 1;
            }
        }
    }

    for i in 0..lines.len() {
        for c in 0..lines[i].len() {
            if lines[i][c] == '*' {
                let mut unique_nums = HashSet::new();
                let mut current_num = 1;
                for &(x, y) in OFFSETS.iter() {
                    let y = i as isize + y;
                    let x = c as isize + x;

                    if y >= 0
                        && x >= 0
                        && y < lines.len() as isize
                        && x < lines[y as usize].len() as isize
                        && is_number(lines[y as usize][x as usize])
                    {
                        let (start_idx, num, _len) =
                            parse_num_containing(y as usize, x as usize, &lines);
                        if !unique_nums.contains(&(y, start_idx)) {
                            unique_nums.insert((y, start_idx));
                            current_num *= num;
                        }
                    }
                }
                if unique_nums.len() == 2 {
                    second_ans += current_num;
                }
            }
        }
    }

    println!("{}", first_ans);
    println!("{}", second_ans);
}

fn is_symbol(c: char) -> bool {
    !is_number(c) && c != '.'
}

fn is_number(c: char) -> bool {
    "1234567890".contains(c)
}

// porses num starting at (c, i) returning the number and the length of the number
fn parse_number(i: usize, c: usize, lines: &Vec<Vec<char>>) -> (u64, usize) {
    let mut num = 0;
    let mut len = 0;
    while c + len < lines[i].len() && is_number(lines[i][c + len]) {
        num *= 10;
        num += lines[i][c + len].to_digit(10).unwrap() as u64;
        len += 1;
    }
    (num, len)
}

// parses the number containing this index and returns (start_idx, num, len)
fn parse_num_containing(i: usize, mut c: usize, lines: &Vec<Vec<char>>) -> (usize, u64, usize) {
    while c > 0 {
        c -= 1;
        if !is_number(lines[i][c]) {
            c += 1;
            break;
        }
    }

    let out = parse_number(i, c, lines);
    (c, out.0, out.1)
}

// checks if the number at (c, i) is in bounds of a symbol
fn in_bounds(i: usize, c: usize, len: usize, lines: &Vec<Vec<char>>) -> bool {
    for off in 0..len {
        for &(x, y) in OFFSETS.iter() {
            let y = i as isize + y;
            let x = c as isize + x + off as isize;
            if y > 0 && x > 0 && y < lines.len() as isize && x < lines[y as usize].len() as isize {
                if is_symbol(lines[y as usize][x as usize]) {
                    return true;
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_is_symbol() {
        assert_eq!(super::is_symbol('1'), false);
        assert_eq!(super::is_symbol('a'), true);
        assert_eq!(super::is_symbol('.'), false);
        assert_eq!(super::is_symbol('#'), true);
    }

    #[test]
    fn test_is_number() {
        assert_eq!(super::is_number('1'), true);
        assert_eq!(super::is_number('a'), false);
        assert_eq!(super::is_number('.'), false);
        assert_eq!(super::is_number('#'), false);
    }

    #[test]
    fn test_parse_number() {
        let lines = vec![vec!['1', '2', '3', '4', '0', '.']];
        assert_eq!(super::parse_number(0, 0, &lines), (12340, 5));
    }

    #[test]
    fn test_parse_num_containing() {
        let lines = vec![vec!['1', '2', '3', '4', '0', '.']];
        assert_eq!(super::parse_num_containing(0, 1, &lines), (0, 12340, 5));
    }

    #[test]
    fn test_in_bounds() {
        let lines = vec![vec!['1', '0', '.', '.'], vec!['.', '#', '.', '.']];

        assert_eq!(super::in_bounds(0, 0, 1, &lines), true);
        assert_eq!(super::in_bounds(0, 0, 2, &lines), true);
        assert_eq!(super::in_bounds(1, 3, 1, &lines), false);
    }
}
