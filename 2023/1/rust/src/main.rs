use std::iter::Peekable;
use std::str::Chars;

fn main() {
    let total = std::fs::read_to_string("../input.txt")
        .expect("Unable to read file")
        .lines()
        .filter_map(extract_digit_first_last)
        .filter_map(combine_digits)
        .fold(0, |acc, x| acc + x as u32);

    println!("Total: {}", total);
}

const STR_NUMBER_MAP: [(&str, u8); 10] = [
    ("zero", 0),
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

enum Match {
    None,
    Partial,
    Full(u8),
}

struct Matcher {
    offset: usize,
    state: Vec<Option<(&'static str, u8)>>,
}

impl Matcher {
    pub fn new() -> Self {
        Self {
            offset: 0,
            state: STR_NUMBER_MAP
                .iter()
                .map(|(s, n)| Some((*s, *n)))
                .collect::<Vec<_>>(),
        }
    }

    pub fn advance(&mut self, ch: char) -> Match {
        let mut some = Match::None;

        for state in self.state.iter_mut() {
            if let Some(valid) = state {
                if valid.0.as_bytes()[self.offset] == ch as u8 {
                    if self.offset == valid.0.len() - 1 {
                        some = Match::Full(valid.1);
                        break;
                    } else {
                        some = Match::Partial;
                    }
                } else {
                    *state = None;
                }
            }
        }

        self.offset += 1;
        some
    }
}

fn peek_str_number(iter: &mut Peekable<Chars>) -> Option<u8> {
    println!("\npeek_str_number {:#?}", iter.clone().collect::<String>());
    let mut peekahead = iter.clone();

    let mut state = Matcher::new();
    while let Some(ch) = peekahead.peek() {
        print!("\n+{}", ch);
        match state.advance(*ch) {
            Match::None => {
                println!("!");
                return None;
            }
            Match::Partial => {
                print!("?{}", state.state.iter().filter(|x| x.is_some()).count());
            }
            Match::Full(num) => {
                println!("={}", num);
                return Some(num);
            }
        }
        peekahead.next();
    }
    None
}

fn extract_digit_first_last(text: &str) -> Option<(u8, u8)> {
    let mut first: Option<u8> = None;
    let mut last: Option<u8> = None;

    let mut iter = text.chars().peekable();

    while let Some(ch) = iter.peek() {
        if let Some(di) = ch.to_digit(10) {
            first = Some(di as u8);
            last = Some(di as u8);
            break;
        } else if let Some(num) = peek_str_number(&mut iter) {
            first = Some(num);
            last = Some(num);
            break;
        }
        iter.next();
    }

    while let Some(ch) = iter.peek() {
        if let Some(di) = ch.to_digit(10) {
            last = Some(di as u8);
        } else if let Some(num) = peek_str_number(&mut iter) {
            last = Some(num);
        }
        iter.next();
    }

    matches!(first, Some(_)).then(|| (first.unwrap(), last.unwrap()))
}

/// Second digit must be in range 0..=9
fn combine_digits((first, second): (u8, u8)) -> Option<u8> {
    matches!(second, 0..=9).then(|| first * 10 + second)
}

#[test]
fn test_example_input() {
    #[rustfmt::skip]
    let expected = [
        ("1abc2", 12),
        ("pqr3stu8vwx", 38),
        ("a1b2c3d4e5f", 15),
        ("treb7uchet", 77),
    ];

    for (input, expected) in expected {
        let first_last = extract_digit_first_last(input).unwrap();
        let combined = combine_digits(first_last).unwrap();

        assert_eq!(combined, expected, "Input: {:#?}", input);
    }
}

#[test]
fn test_example_input_2() {
    let expected = [
        ("two1nine", 29),
        ("eightwothree", 83),
        ("abcone2threexyz", 13),
        ("xtwone3four", 24),
        ("4nineeightseven2", 42),
        ("zoneight234", 14),
        ("7pqrstsixteen", 76),
    ];

    for (input, expected) in expected {
        let first_last = extract_digit_first_last(input).unwrap();
        let combined = combine_digits(first_last).unwrap();

        assert_eq!(combined, expected, "Input: {:#?}", input);
    }
}

#[test]
fn test_combine_digits() {
    #[ rustfmt::skip]
    let expected = [
        (1, 2, 12),
        (3, 8, 38),
        (1, 5, 15),
        (7, 7, 77),
    ];

    for (first, last, expected) in expected {
        let combined = combine_digits((first, last)).unwrap();
        assert_eq!(combined, expected);
    }
}

#[test]
fn test_peek() {
    #[rustfmt::skip]
    let expected = [
        ("one", 1), 
        (" one", 1), 
        ("twone", 2), 
        ("threone", 1),
    ];

    for (input, expected) in expected {
        let parsed = parse_str(input);
        assert_eq!(parsed, Some(expected), "Input: {:#?}", input);
    }

    fn parse_str(text: &str) -> Option<u8> {
        println!("parse_str: {:#?}", text);
        let mut iter = text.chars().peekable();
        while iter.peek().is_some() {
            if let Some(d) = peek_str_number(&mut iter) {
                return Some(d);
            }
            iter.next();
        }

        None
    }
}
