fn main() {
    let total = std::fs::read_to_string("../input.txt")
        .expect("Unable to read file")
        .lines()
        .filter_map(extract_digit_first_last)
        .filter_map(combine_digits)
        .fold(0, |acc, x| acc + x as u32);

    println!("Total: {}", total);
}

fn extract_digit_first_last(text: &str) -> Option<(u8, u8)> {
    let mut first: Option<u8> = None;
    let mut last: Option<u8> = None;

    let mut iter = text.chars();

    while let Some(ch) = iter.next() {
        if let Some(di) = ch.to_digit(10) {
            first = Some(di as u8);
            last = Some(di as u8);
            break;
        }
    }

    while let Some(ch) = iter.next_back() {
        if let Some(di) = ch.to_digit(10) {
            last = Some(di as u8);
            break;
        }
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

        assert_eq!(combined, expected);
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

        assert_eq!(combined, expected);
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
