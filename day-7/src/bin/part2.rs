use std::cmp::Ordering;
use std::fmt;
use std::time::Instant;

const CARDS: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];
const HAND_SIZE: usize = 5;

#[derive(Debug, Eq, PartialEq)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn get_value(&self) -> u32 {
        match self {
            HandType::HighCard => 0,
            HandType::Pair => 1,
            HandType::TwoPair => 2,
            HandType::ThreeOfAKind => 3,
            HandType::FullHouse => 4,
            HandType::FourOfAKind => 5,
            HandType::FiveOfAKind => 6,
        }
    }
}

#[derive(Eq)]
struct Hand {
    cards: [u32; HAND_SIZE],
    bid: u32,
}

impl Hand {
    fn get_type(&self) -> HandType {
        let mut card_counts: [u32; 13] = [0; 13];
        for c in self.cards.iter() {
            card_counts[*c as usize] += 1;
        }
        // Add the joker count to the highest card count
        let joker_count = card_counts[0];
        card_counts[0] = 0;
        // Get index of highest card count
        let mut highest_card_count = 0;
        for i in 0..13 {
            if card_counts[i] > card_counts[highest_card_count] {
                highest_card_count = i;
            }
        }
        card_counts[highest_card_count] += joker_count;

        if card_counts.contains(&5) || joker_count == 5 {
            HandType::FiveOfAKind
        } else if card_counts.contains(&4) {
            HandType::FourOfAKind
        } else if card_counts.contains(&3) {
            if card_counts.contains(&2) {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        } else if card_counts.contains(&2) {
            if card_counts.iter().filter(|c| *c >= &2).count() >= 2 {
                HandType::TwoPair
            } else {
                HandType::Pair
            }
        } else {
            HandType::HighCard
        }
    }
}

impl fmt::Debug for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for c in self.cards.iter() {
            s.push(CARDS[*c as usize]);
        }
        write!(f, "{}: {}", s, self.bid)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.get_type().get_value() < other.get_type().get_value() {
            return Ordering::Less;
        } else if self.get_type().get_value() > other.get_type().get_value() {
            return Ordering::Greater;
        }

        for i in 0..HAND_SIZE {
            if self.cards[i] < other.cards[i] {
                return Ordering::Less;
            } else if self.cards[i] > other.cards[i] {
                return Ordering::Greater;
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

fn parse_hand(line: &str) -> Hand {
    let mut cards = [0; HAND_SIZE];
    let parts = line.split_whitespace().collect::<Vec<_>>();
    let hand = parts[0];
    let bid_str = parts[1];
    for (i, c) in hand.chars().enumerate() {
        cards[i] = CARDS.iter().position(|&x| x == c).unwrap() as u32;
    }
    let bid = bid_str.parse::<u32>().unwrap();
    Hand { cards, bid }
}

fn solve(file: &str) -> i64 {
    let mut hands = file.lines().map(|x| parse_hand(x)).collect::<Vec<_>>();
    hands.sort();
    // println!("{:?}", hands);
    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, h)| acc + (i + 1) as i64 * h.bid as i64)
}

fn main() {
    let input = include_str!("input.txt");
    println!("Starting solution");
    let t0 = Instant::now();
    let result = solve(input);
    let duration = t0.elapsed();
    println!("Result: {}", result);
    println!("Time: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whole_part() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(solve(input), 5905);
    }

    #[test]
    fn test_joker() {
        let input = "2345J 1\n2344J 2";
        assert_eq!(solve(input), 5);
    }
    #[test]
    fn test_joker_pair() {
        let input = "2345J 1";
        assert_eq!(parse_hand(input).get_type(), HandType::Pair);
    }

    #[test]
    fn test_joker_five() {
        let input = "22JJJ 1";
        assert_eq!(parse_hand(input).get_type(), HandType::FiveOfAKind);
        let input = "JJJJJ 1";
        assert_eq!(parse_hand(input).get_type(), HandType::FiveOfAKind);
    }

    #[test]
    fn test_joker_four() {
        let input = "233JJ 1";
        assert_eq!(parse_hand(input).get_type(), HandType::FourOfAKind);
    }
    #[test]
    fn test_joker_full() {
        let input = "2332J 1";
        assert_eq!(parse_hand(input).get_type(), HandType::FullHouse);
    }
    #[test]
    fn test_joker_three() {
        let input = "2334J 1";
        assert_eq!(parse_hand(input).get_type(), HandType::ThreeOfAKind);
    }

    #[test]
    fn test_joker_comparison() {
        let input = "23J44 1\n2344J 2";
        let mut hands = input.lines().map(|x| parse_hand(x)).collect::<Vec<_>>();
        hands.sort();
        assert_eq!(hands[0].bid, 1);
        assert_eq!(hands[1].bid, 2);
    }

    #[test]
    fn test_joker_comparison2() {
        let input = "23J44 1\n2344J 2\n2342J 3";
        let mut hands = input.lines().map(|x| parse_hand(x)).collect::<Vec<_>>();
        hands.sort();
        assert_eq!(hands[0].bid, 1);
        assert_eq!(hands[1].bid, 3);
        assert_eq!(hands[2].bid, 2);
    }

    #[test]
    fn test_joker_comparison3() {
        let input = "";
        let mut hands = input.lines().map(|x| parse_hand(x)).collect::<Vec<_>>();
        hands.sort();
    }
}
