fn main() {
    println!("Hello, world!");
}

struct WinningNumbers(u32, [u8; 5]);

struct ScratchCard([u8; 8]);

impl ScratchCard {
    fn score(&self, card: &WinningNumbers) -> u8 {
        let mut score = 0;
        for number in &self.0 {
            if card.1.contains(number) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }
        score
    }
}

fn parse_input(input: &str) -> (WinningNumbers, ScratchCard) {
    // Example input: "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
    //                     id  winning numbers| card numbers
    let Some((id, numbers)) = input.split_once(": ") else {
        panic!("Invalid winning numbers input");
    };

    let Some((_, id)) = id.split_once(" ") else {
        panic!("Invalid card id");
    };

    let Some((winningnumbers, cardnumbers)) = numbers.split_once(" | ") else {
        panic!("Invalid input");
    };

    let id = id.parse().expect("Invalid card id");

    let winningnumbers = winningnumbers
        .split_ascii_whitespace()
        .map(|n| n.parse().expect("Invalid winning number"))
        .collect::<Vec<u8>>()
        .try_into()
        .expect("Count of winning numbers must be 8");

    let cardnumbers = cardnumbers
        .split_ascii_whitespace()
        .map(|n| n.parse().expect("Invalid card number"))
        .collect::<Vec<u8>>()
        .try_into()
        .expect("Count of card numbers must be 5");

    (WinningNumbers(id, winningnumbers), ScratchCard(cardnumbers))
}

#[test]
fn test_example_input_1() {
    #[rustfmt::skip]
    let expected = [
        (8, "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
        (2, "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
        (2, "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
        (1, "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
        (0, "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
        (0, "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
    ];

    for (expected, input) in expected {
        let (card, winning) = parse_input(input);
        assert_eq!(expected, winning.score(&card), "Input: {}", input);
    }
}
