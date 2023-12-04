use std::collections::binary_heap::PeekMut;

fn main() {
    let bag = Set::new(12, 13, 14);

    let game_id_sum = std::fs::read_to_string("../input.txt")
        .expect("Failed to read input.txt")
        .lines()
        .filter_map(|line| {
            let tokens = tokenize(line);
            let mut tokens = tokens.iter().peekable();
            let game = Game::from_tokens(&mut tokens);

            if game.is_possible(&bag) {
                Some(game.id)
            } else {
                None
            }
        })
        .sum::<u32>();

    println!("Game ID sum: {}", game_id_sum);
}

#[derive(Debug, PartialEq)]
enum Token {
    Text(String),
    Number(u32),
    Semicolon,
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '0'..='9' => {
                let mut number = c.to_digit(10).unwrap();
                while let Some('0'..='9') = chars.peek() {
                    number = number * 10 + chars.next().unwrap().to_digit(10).unwrap();
                }
                tokens.push(Token::Number(number));
            }
            'a'..='z' | 'A'..='Z' => {
                let mut text = c.to_string();
                while let Some('a'..='z' | 'A'..='Z') = chars.peek() {
                    text.push(chars.next().unwrap());
                }
                tokens.push(Token::Text(text));
            }
            ';' => tokens.push(Token::Semicolon),
            ' ' | ':' | ',' => {
                // Ignored characters
            }
            _ => {
                panic!("Unexpected character: {}", c);
            }
        }
    }

    tokens
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    fn new(id: u32, sets: Vec<Set>) -> Self {
        Self { id, sets }
    }

    fn from_tokens<'a>(tokens: &mut std::iter::Peekable<impl Iterator<Item = &'a Token>>) -> Self {
        match tokens.next().unwrap() {
            Token::Text(text) if text == "Game" => {}
            _ => panic!("Expected text 'Game'"),
        }
        let id = match tokens.next().unwrap() {
            Token::Number(n) => *n,
            _ => panic!("Expected number"),
        };

        let mut sets = Vec::new();

        while let Some(token) = tokens.peek() {
            match token {
                Token::Number(_) => sets.push(Set::from_tokens(tokens)),
                Token::Semicolon => break,
                _ => panic!("Unexpected token: {:?}", token),
            }
        }

        Self::new(id, sets)
    }

    fn is_possible(&self, bag: &Set) -> bool {
        self.sets.iter().all(|set| set.is_less_than(&bag))
    }
}

#[derive(Debug, PartialEq)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl Set {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    fn from_tokens<'a>(tokens: &mut std::iter::Peekable<impl Iterator<Item = &'a Token>>) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        while let Some(token) = tokens.next() {
            match token {
                Token::Number(n) => {
                    let color = tokens.next().unwrap();
                    match color {
                        Token::Text(color) => match color.as_str() {
                            "red" => red = *n,
                            "green" => green = *n,
                            "blue" => blue = *n,
                            _ => panic!("Unexpected color: {}", color),
                        },
                        _ => panic!("Unexpected token: {:?}", color),
                    }
                }
                Token::Semicolon => break,
                _ => panic!("Unexpected token: {:?}", token),
            }
        }

        Self::new(red, green, blue)
    }

    fn is_less_than(&self, other: &Self) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }
}

#[test]
fn test_example_input() {
    #[rustfmt::skip]
    let expected = [
        (true,  "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
        (true,  "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
        (false, "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
        (false, "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
        (true,  "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
    ];

    let bag = Set::new(12, 13, 14);

    for (expected, input) in expected {
        let tokens = tokenize(input);
        let mut tokens = tokens.iter().peekable();
        let game = Game::from_tokens(&mut tokens);
        assert_eq!(game.is_possible(&bag), expected);
    }
}

#[test]
fn test_tokenize_input() {
    let input = "Game 1: 1 blue, 3 red; 24 red, 60 green";
    let expected = [
        Token::Text("Game".to_string()),
        Token::Number(1),
        Token::Number(1),
        Token::Text("blue".to_string()),
        Token::Number(3),
        Token::Text("red".to_string()),
        Token::Semicolon,
        Token::Number(24),
        Token::Text("red".to_string()),
        Token::Number(60),
        Token::Text("green".to_string()),
    ];

    assert_eq!(tokenize(input), expected);
}

#[test]
fn test_tokenize_set() {
    let input = [
        Token::Number(1),
        Token::Text("blue".to_string()),
        Token::Number(3),
        Token::Text("red".to_string()),
    ];
    let expected = Set::new(3, 0, 1);

    let mut p = input.iter().peekable();
    assert_eq!(Set::from_tokens(&mut p), expected);
}

fn test_tokenize_game() {
    let input = [
        Token::Text("Game".to_string()),
        Token::Number(1),
        Token::Number(1),
        Token::Text("blue".to_string()),
        Token::Number(3),
        Token::Text("red".to_string()),
    ];
    let expected = Game::new(1, vec![Set::new(3, 0, 1)]);

    let mut p = input.iter().peekable();
    assert_eq!(Game::from_tokens(&mut p), expected);
}
