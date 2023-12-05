use std::io::Write;
use std::ops::RangeInclusive;

fn main() {
    let schematic = std::fs::read_to_string("../input.txt")
        .expect("Failed to read ../input.txt")
        .lines()
        .map(parse_tokens)
        .collect::<Vec<_>>();

    let schematic = Schematic::new(schematic);

    let mut sum = 0;
    for row in 0..schematic.height() {
        sum += sum_partnumbers(row, &schematic);
    }

    println!("Sum of part numbers: {}", sum);
}

fn sum_partnumbers(y: isize, schema: &Schematic) -> u32 {
    let mut partnumber_sum = 0;

    let mut x = 0isize;
    while x < schema.width() {
        let (number, digit_count) = match schema.number_at(x, y) {
            Some(r) => r,
            None => {
                x += 1;
                continue;
            }
        };

        let surrounding_x = (x - 1)..=(x + digit_count);
        let surrounding_y = (y - 1)..=(y + 1);

        if schema.has_symbol(surrounding_x, surrounding_y) {
            partnumber_sum += number;
            x += digit_count as isize + 1;
        } else {
            x += 1;
        }
    }

    partnumber_sum
}

#[derive(Clone, Copy)]
enum Token {
    Empty,
    Symbol,
    Digit(u8),
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            '.' => Token::Empty,
            '*' | '#' | '+' | '$' | '=' | '%' | '-' | '@' | '/' | '\\' | '&' => Token::Symbol,
            '0'..='9' => Token::Digit(c.to_digit(10).unwrap() as u8),
            _ => panic!("Invalid char: {}", c),
        }
    }
}

fn parse_tokens(line: &str) -> Vec<Token> {
    line.chars().into_iter().map(Token::from).collect()
}

struct Schematic {
    tokens: Vec<Vec<Token>>,
}

impl Schematic {
    fn new(tokens: Vec<Vec<Token>>) -> Self {
        Self { tokens }
    }

    fn width(&self) -> isize {
        self.tokens[0].len() as isize
    }

    fn height(&self) -> isize {
        self.tokens.len() as isize
    }

    fn get(&self, x: isize, y: isize) -> Option<Token> {
        let valid_x = 0..self.width();
        let valid_y = 0..self.height();

        if !valid_x.contains(&x) || !valid_y.contains(&y) {
            return None;
        }

        return Some(self.tokens[y as usize][x as usize]);
    }

    fn has_symbol(&self, rx: RangeInclusive<isize>, ry: RangeInclusive<isize>) -> bool {
        for y in ry {
            for x in rx.clone() {
                if let Some(Token::Symbol) = self.get(x, y) {
                    return true;
                }
            }
        }

        false
    }

    fn number_at(&self, x: isize, y: isize) -> Option<(u32, isize)> {
        let mut count = 0;
        let mut sum = 0;
        loop {
            match self.get(x + count, y) {
                Some(Token::Digit(d)) => {
                    count += 1;
                    sum *= 10;
                    sum += d as u32;
                }
                _ => break,
            }
        }

        match count {
            0 => None,
            _ => Some((sum, count)),
        }
    }
}

#[test]
fn test_example_input_1() {
    #[rustfmt::skip]
    let expected = [
        (467,  "467..114.."),
        (0,    "...*......"),
        (668,  "..35..633."),
        (0,    "......#..."),
        (617,  "617*......"),
        (0,    ".....+.58."),
        (592,  "..592....."),
        (755,  "......755."),
        (0,    "...$.*...."),
        (1262, ".664.598.."),
    ];

    assert_eq!(expected.iter().map(|(s, _)| *s).sum::<u32>(), 4361);

    let schematic = Schematic::new(
        expected
            .iter()
            .map(|(_, line)| parse_tokens(line))
            .collect(),
    );

    for y in 0..schematic.height() {
        let res = sum_partnumbers(y, &schematic);
        assert_eq!(res, expected[y as usize].0);
    }
}
