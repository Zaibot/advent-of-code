use std::collections::HashSet;
use std::io::Write;
use std::ops::RangeInclusive;

fn main() {
    let schematic = std::fs::read_to_string("../input.txt")
        .expect("Failed to read ../input.txt")
        .lines()
        .map(parse_tokens)
        .collect::<Vec<_>>();

    let schematic = Schematic::new(schematic);

    let mut sum_pn = 0;
    for row in 0..schematic.height() {
        sum_pn += sum_partnumbers(row, &schematic);
    }
    println!("Sum of part numbers: {}", sum_pn);

    let mut sum_gr = 0;
    for row in 0..schematic.height() {
        sum_gr += sum_gearratios(row, &schematic);
    }
    println!("Sum of gear ratios: {}", sum_gr);
}

/// Sum all numbers that have an symbol in any neighbouring cell.
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

/// Sum for each gear token the product of all neighbouring numbers; having atleast two numbers.
fn sum_gearratios(y: isize, schema: &Schematic) -> u32 {
    let mut power_sum = 0;

    let mut x = 0isize;
    while x < schema.width() {
        if !matches!(schema.get(x, y), Some(Token::Gear)) {
            x += 1;
            continue;
        }

        let mut num_coords = HashSet::new();
        for y in (y - 1)..=(y + 1) {
            for x in (x - 1)..=(x + 1) {
                if let Some(coord) = schema.start_of_number(x, y) {
                    num_coords.insert(coord);
                }
            }
        }

        if num_coords.len() >= 2 {
            let mut values = Vec::new();
            for (x, y) in num_coords {
                let (num, _) = schema.number_at(x, y).unwrap();
                values.push(num);
            }

            power_sum += values.iter().fold(1 as u32, |acc, x| acc * x);
        }

        x += 1;
    }

    power_sum
}

#[derive(Clone, Copy)]
enum Token {
    Empty,
    Symbol,
    Gear,
    Digit(u8),
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            '.' => Token::Empty,
            '#' | '+' | '$' | '=' | '%' | '-' | '@' | '/' | '\\' | '&' => Token::Symbol,
            '*' => Token::Gear,
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

                if let Some(Token::Gear) = self.get(x, y) {
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

    fn start_of_number(&self, x: isize, y: isize) -> Option<(isize, isize)> {
        let mut count = 0;
        loop {
            match self.get(x - count, y) {
                Some(Token::Digit(_)) => count += 1,
                _ => break,
            }
        }

        match count {
            0 => None,
            _ => Some((x - count + 1, y)),
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

#[test]
fn test_example_input_2() {
    #[rustfmt::skip]
    let expected = [
        (0,      "467..114.."),
        (16345,  "...*......"),
        (0,      "..35..633."),
        (0,      "......#..."),
        (0,      "617*......"),
        (0,      ".....+.58."),
        (0,      "..592....."),
        (0,      "......755."),
        (451490, "...$.*...."),
        (0,      ".664.598.."),
    ];

    assert_eq!(expected.iter().map(|(s, _)| *s).sum::<u32>(), 467835);

    let schematic = Schematic::new(
        expected
            .iter()
            .map(|(_, line)| parse_tokens(line))
            .collect(),
    );

    for y in 0..schematic.height() {
        let res = sum_gearratios(y, &schematic);
        assert_eq!(res, expected[y as usize].0);
    }
}

#[test]
fn test_cases_i_might_have_screwed_up() {
    fn sum_text(text: &str) -> u32 {
        let schematic = Schematic::new(vec![parse_tokens(text)]);

        let mut sum = 0;
        for row in 0..schematic.height() {
            sum += sum_partnumbers(row, &schematic);
        }
        sum
    }

    assert_eq!(sum_text("#1#2#"), 3);
    assert_eq!(sum_text("1.#2#3#"), 5); // sigh
    assert_eq!(sum_text("#1#2#.3"), 3);
    assert_eq!(sum_text("#1.2.3#"), 4);
}

fn write_debug_schematic(schematic: &Schematic) {
    let mut output = std::fs::File::create("../output-debug.txt")
        .expect("Failed to write to ../output-debug.txt");

    for row in 0..schematic.height() {
        let mut x = 0isize;
        while x < schematic.width() {
            match schematic.get(x, row) {
                Some(Token::Empty) => {
                    write!(output, " ").unwrap();
                    x += 1;
                    continue;
                }

                Some(Token::Symbol) => {
                    write!(output, "X").unwrap();
                    x += 1;
                    continue;
                }

                None => {
                    write!(output, "?").unwrap();
                    x += 1;
                    continue;
                }

                Some(Token::Gear) => {
                    write!(output, "*").unwrap();
                    x += 1;
                    continue;
                }

                Some(Token::Digit(d)) => {
                    let (number, digit_count) = match schematic.number_at(x, row) {
                        Some(r) => r,
                        None => {
                            write!(output, ".").unwrap();
                            x += 1;
                            continue;
                        }
                    };

                    let surrounding_x = (x - 1)..=(x + digit_count + 1);
                    let surrounding_y = (row - 1)..=(row + 1);

                    if schematic.has_symbol(surrounding_x, surrounding_y) {
                        write!(output, "{}", number).unwrap();
                        x += digit_count as isize;
                    } else {
                        for _ in 0..digit_count {
                            write!(output, "-").unwrap();
                        }
                        x += digit_count as isize;
                    }
                }
            }
        }
        writeln!(output).unwrap();
    }
}
