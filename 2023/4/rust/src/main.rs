use std::collections::HashMap;

fn main() {
    let cards = std::fs::read_to_string("../input.txt")
        .expect("Failed to read ../input.txt")
        .lines()
        .map(parse_input)
        .collect::<Vec<_>>();

    let sum_score = cards
        .iter()
        .map(|card| card.score())
        .fold(0, |acc, score| acc + score);
    println!("Sum of scores: {}", sum_score);

    let count_winnings = eval_cards_stage2(cards).len();
    println!("Count of winnings: {}", count_winnings);
}

type CardId = u32;

#[derive(Debug, Clone)]
struct WinningNumbers(Vec<u8>);

#[derive(Debug, Clone)]
struct ScratchCard(Vec<u8>);

#[derive(Debug, Clone)]
struct Card(CardId, WinningNumbers, ScratchCard, u32, u32);

impl Card {
    fn new(id: CardId, winning_numbers: WinningNumbers, scratch_card: ScratchCard) -> Self {
        let score = {
            // First number is worth 1 point, after that accumulate points by doubling
            winning_numbers
                .0
                .iter()
                .filter(|number| scratch_card.0.contains(number))
                .fold(0, |acc, _| (acc * 2).max(1))
        };

        let match_count = {
            winning_numbers
                .0
                .iter()
                .filter(|n| scratch_card.0.contains(n))
                .count() as u32
        };

        Self(id, winning_numbers, scratch_card, score, match_count)
    }

    fn score(&self) -> u32 {
        self.3
    }

    fn match_count(&self) -> u32 {
        self.4
    }
}

fn parse_input(input: &str) -> Card {
    // Example input: "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
    //                     id  winning numbers| card numbers
    let Some((id, numbers)) = input.split_once(": ") else {
        panic!("Invalid winning numbers input");
    };

    let Some((_, id)) = id.split_once(' ') else {
        panic!("Invalid card id");
    };

    let Some((winningnumbers, cardnumbers)) = numbers.split_once(" | ") else {
        panic!("Invalid input");
    };

    let id = id.trim().parse().expect("Invalid card id");

    let winningnumbers = winningnumbers
        .split_ascii_whitespace()
        .map(|n| n.parse().expect("Invalid winning number"))
        .collect::<Vec<u8>>();

    let cardnumbers = cardnumbers
        .split_ascii_whitespace()
        .map(|n| n.parse().expect("Invalid card number"))
        .collect::<Vec<u8>>();

    Card::new(id, WinningNumbers(winningnumbers), ScratchCard(cardnumbers))
}

/// Each card's match count determines how many copies of the cards it gets after.
/// This repeats until no new cards are won.
fn eval_cards_stage2(cards: Vec<Card>) -> Vec<CardId> {
    let index: HashMap<CardId, Card> =
        HashMap::from_iter(cards.into_iter().map(|card| (card.0, card)));
    let mut won_ids = Vec::from_iter(index.keys().cloned());

    let mut pending_ids = won_ids.clone();
    while let Some(current_id) = pending_ids.pop() {
        let match_count = index[&current_id].match_count();
        for card_id in current_id + 1..=current_id + match_count {
            won_ids.push(card_id);
            pending_ids.push(card_id);
        }
    }

    won_ids
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
        let card = parse_input(input);
        assert_eq!(expected, card.score(), "Input: {}", input);
    }
}

#[test]
fn test_stage_2() {
    #[rustfmt::skip]
    let expected = [
        (1, 1,  "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
        (2, 2,  "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
        (3, 4,  "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
        (4, 8,  "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
        (5, 14, "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
        (6, 1,  "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
    ];

    let won_ids = eval_cards_stage2(
        expected
            .iter()
            .map(|(_, _, input)| parse_input(input))
            .collect::<Vec<_>>(),
    );

    for (id, count, input) in expected {
        assert_eq!(
            count,
            won_ids.iter().filter(|&&card| card == id).count(),
            "Input: {}",
            input
        );
    }

    assert_eq!(30, won_ids.len());
}
