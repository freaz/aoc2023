use shared::{debug_print, load_file};

const CARDS: [char; 13] = [
    // '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A', // part one
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A', // part two
];

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    FiveOfKind = 7,
    FourOfKind = 6,
    FullHouse = 5,
    ThreeOfKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug, Eq, Ord)]
struct Hand {
    cards: [char; 5],
    hand_type: HandType,
    bid: usize,
}
impl Hand {
    fn new(cards: [char; 5], bid: usize) -> Self {
        let hand_type = Hand::get_hand_type(&cards);
        debug_print!("Hand: {:?} {:?}\n", cards, hand_type);
        Hand {
            cards,
            hand_type,
            bid,
        }
    }
    fn get_hand_type(cards: &[char; 5]) -> HandType {
        let mut counts = [0usize; 15];
        let mut jokers: usize = 0;

        for card in cards {
            if *card == 'J' {
                jokers += 1;
            } else {
                let i = CARDS.iter().position(|&c| c == *card).unwrap();
                counts[i] += 1;
            }
        }

        counts.sort();
        counts.reverse();
        counts[0] += jokers;

        let hand_type = match counts[0..2] {
            [a, _] if a == 5 => HandType::FiveOfKind,
            [a, _] if a == 4 => HandType::FourOfKind,
            [a, b] if a == 3 && b == 2 => HandType::FullHouse,
            [a, b] if a == 3 && b == 1 => HandType::ThreeOfKind,
            [a, b] if a == 2 && b == 2 => HandType::TwoPair,
            [a, b] if a == 2 && b == 1 => HandType::OnePair,
            _ => HandType::HighCard,
        };

        debug_print!("get_hand_type: {:?} {:?} {}", cards, counts, jokers);

        hand_type
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.hand_type != other.hand_type {
            return false;
        }

        for (i, card) in self.cards.iter().enumerate() {
            if card != &other.cards[i] {
                return false;
            }
        }

        true
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.hand_type != other.hand_type {
            return self.hand_type.partial_cmp(&other.hand_type);
        }

        for (i, card) in self.cards.iter().enumerate() {
            let a = CARDS.iter().position(|&c| c == *card).unwrap();
            let b = CARDS.iter().position(|&c| c == other.cards[i]).unwrap();
            if a != b {
                return a.partial_cmp(&b);
            }
        }

        Some(std::cmp::Ordering::Equal)
    }
}

fn main() {
    let data = load_file("input.txt");

    let mut hands: Vec<Hand> = Vec::new();

    for line in data.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let cards: [char; 5] = parts[0].chars().collect::<Vec<char>>()[0..5]
            .try_into()
            .unwrap();
        let bid: usize = parts[1].parse().unwrap();
        hands.push(Hand::new(cards, bid));
    }

    hands.sort();

    let winnings: Vec<usize> = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .collect();

    debug_print!("Final rankings: {:?}", hands);
    debug_print!("Winnings: {:?}", winnings);

    println!(
        "Result One: {}",
        winnings.iter().fold(0, |acc, winning| acc + winning)
    );
}
