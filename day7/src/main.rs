use log::debug;
use simple_logger::SimpleLogger;
use std::cmp::Ordering;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum HandType {
    Five,
    Four,
    Full,
    Three,
    Two,
    One,
    High,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Hand([Card; 5]);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct WeightedHand {
    hand: Hand,
    weight: usize,
}

impl Card {
    fn to_int(self) -> u8 {
        match self {
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::T => 10,
            Card::J => 11,
            Card::Q => 12,
            Card::K => 13,
            Card::A => 14,
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_int().cmp(&other.to_int())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl HandType {
    fn to_int(self) -> u8 {
        match self {
            HandType::Five => 6,
            HandType::Four => 5,
            HandType::Full => 4,
            HandType::Three => 3,
            HandType::Two => 2,
            HandType::One => 1,
            HandType::High => 0,
        }
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_int().cmp(&other.to_int())
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn to_handtype(mut self) -> HandType {
        self.0.sort();
        if self.0[0] == self.0[1] {
            // XX***
            if self.0[1] == self.0[2] {
                //XXX**
                if self.0[2] == self.0[3] {
                    // XXXX*
                    if self.0[3] == self.0[4] {
                        // XXXXX
                        HandType::Five
                    } else {
                        // XXXXY
                        HandType::Four
                    }
                } else {
                    // XXXY*
                    if self.0[3] == self.0[4] {
                        // XXXYY
                        HandType::Full
                    } else {
                        // XXXYZ
                        HandType::Three
                    }
                }
            } else {
                // XXY**
                if self.0[2] == self.0[3] {
                    // XXYY*
                    if self.0[3] == self.0[4] {
                        // XXYYY
                        HandType::Full
                    } else {
                        // XXYYZ
                        HandType::Two
                    }
                } else {
                    // XXYZ*
                    if self.0[3] == self.0[4] {
                        // XXYZZ
                        HandType::Two
                    } else {
                        // XXYZT
                        HandType::One
                    }
                }
            }
        } else {
            // XY***
            if self.0[1] == self.0[2] {
                //XYY**
                if self.0[2] == self.0[3] {
                    // XYYY*
                    if self.0[3] == self.0[4] {
                        // XYYYY
                        HandType::Four
                    } else {
                        // XYYYZ
                        HandType::Three
                    }
                } else {
                    // XYYZ*
                    if self.0[3] == self.0[4] {
                        // XYYZZ
                        HandType::Two
                    } else {
                        // XYYZT
                        HandType::One
                    }
                }
            } else {
                // XYZ**
                if self.0[2] == self.0[3] {
                    // XYZZ*
                    if self.0[3] == self.0[4] {
                        // XYZZZ
                        HandType::Three
                    } else {
                        // XYZZT
                        HandType::One
                    }
                } else {
                    // XYZT*
                    if self.0[3] == self.0[4] {
                        // XYZTT
                        HandType::One
                    } else {
                        // XYZTU
                        HandType::High
                    }
                }
            }
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            Ordering::Equal
        } else {
            let ht1 = self.to_handtype();
            let ht2 = other.to_handtype();
            if ht1 != ht2 {
                ht1.to_int().cmp(&ht2.to_int())
            } else {
                let mut i = 0;
                while self.0[i] == other.0[i] {
                    i += 1;
                }
                self.0[i].to_int().cmp(&other.0[i].to_int())
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for WeightedHand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl PartialOrd for WeightedHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

trait ToCard {
    fn to_card(&self) -> Card;
}

impl ToCard for char {
    fn to_card(&self) -> Card {
        match self {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!("Invalid character"),
        }
    }
}

trait ToHand {
    fn to_hand(&self) -> Hand;
}

impl ToHand for str {
    fn to_hand(&self) -> Hand {
        let hand: [Card; 5] = self
            .chars()
            .map(|c| c.to_card())
            .collect::<Vec<Card>>()
            .try_into()
            .unwrap_or_else(|v: Vec<Card>| {
                panic!("Expected a string of length {} but it was {}", 5, v.len())
            });
        Hand(hand)
    }
}

// SECOND PART

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct JokerHand(Hand);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct WeightedJokerHand {
    hand: JokerHand,
    weight: usize,
}

impl Card {
    fn to_joker_int(self) -> u8 {
        match self {
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::T => 10,
            Card::J => 1,
            Card::Q => 12,
            Card::K => 13,
            Card::A => 14,
        }
    }
}

impl JokerHand {
    fn to_handtype(self) -> HandType {
        let hand = self.0;
        let mut ht = hand.to_handtype();
        // if there are several jokers, the best hand is always reached when all jokers are
        // replaced by the same card
        for c in Card::iter() {
            let mut temp_hand = hand;
            for d in temp_hand.0.iter_mut() {
                if *d == Card::J {
                    *d = c;
                }
            }
            if temp_hand.to_handtype() > ht {
                ht = temp_hand.to_handtype();
            }
        }
        ht
    }
}

impl Ord for JokerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            Ordering::Equal
        } else {
            let ht1 = self.to_handtype();
            let ht2 = other.to_handtype();
            if ht1 != ht2 {
                ht1.to_int().cmp(&ht2.to_int())
            } else {
                let mut i = 0;
                while self.0 .0[i] == other.0 .0[i] {
                    i += 1;
                }
                // J is now the weakest card!
                self.0 .0[i]
                    .to_joker_int()
                    .cmp(&other.0 .0[i].to_joker_int())
            }
        }
    }
}

impl PartialOrd for JokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for WeightedJokerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl PartialOrd for WeightedJokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    SimpleLogger::new().init().unwrap();

    let contents = include_str!("../input");

    let mut wh_vec: Vec<_> = contents
        .lines()
        .map(|l| {
            let mut it = l.split_ascii_whitespace();
            WeightedHand {
                hand: it.next().unwrap().to_hand(),
                weight: it.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    wh_vec.sort();

    debug!("{:#?}", wh_vec);

    let total: usize = wh_vec
        .iter()
        .enumerate()
        .map(|t| (t.0 + 1) * t.1.weight)
        .sum();

    println!("The total winnings is {total}");

    let mut wh_vec: Vec<_> = contents
        .lines()
        .map(|l| {
            let mut it = l.split_ascii_whitespace();
            WeightedJokerHand {
                hand: JokerHand(it.next().unwrap().to_hand()),
                weight: it.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    wh_vec.sort();

    debug!("{:#?}", wh_vec);

    let total2: usize = wh_vec
        .iter()
        .enumerate()
        .map(|t| (t.0 + 1) * t.1.weight)
        .sum();

    println!("The second total winnings is {total2}");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_to_hand() {
        assert_eq!(
            "AA2J7".to_hand(),
            Hand([Card::A, Card::A, Card::Two, Card::J, Card::Seven])
        );
    }

    #[test]
    fn test_to_handtype() {
        assert_eq!("2259T".to_hand().to_handtype(), HandType::One);
        assert_eq!("2229T".to_hand().to_handtype(), HandType::Three);
        assert_eq!("2299T".to_hand().to_handtype(), HandType::Two);
        assert_eq!("23999".to_hand().to_handtype(), HandType::Three);
        assert_eq!("23399".to_hand().to_handtype(), HandType::Two);
        assert_eq!("TTJJJ".to_hand().to_handtype(), HandType::Full);
        assert_eq!("29944".to_hand().to_handtype(), HandType::Two);
    }

    #[test]
    fn test_compare_hand() {
        assert_eq!("2244T".to_hand().cmp(&"33TTJ".to_hand()), Ordering::Less);
        assert_eq!("2244T".to_hand().cmp(&"222JJ".to_hand()), Ordering::Less);
    }
}
