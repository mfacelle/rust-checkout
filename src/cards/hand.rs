use super::card::{Card, Rank, Suit};
use std::{collections::HashMap, fmt};

// possible poker hands
#[derive(Debug, PartialEq, Eq)]
pub enum PokerHand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

// determine what type of poker hand the slice of cards provided is
impl PokerHand {
    fn from_cards(cards: &[Card]) -> Self {
        let mut rank_counts: HashMap<Rank, u8> = HashMap::new();
        let mut suit_counts: HashMap<Suit, u8> = HashMap::new();
        for card in cards {
            *rank_counts.entry(card.rank).or_insert(0) += 1;
            *suit_counts.entry(card.suit).or_insert(0) += 1;
        }

        let is_flush = suit_counts
            .values()
            .any(|&count| count as usize == Hand::POKER_HAND_SIZE);

        let mut sorted_ranks: Vec<u8> = cards
            .iter()
            .map(|card| match card.rank {
                Rank::Two => 2,
                Rank::Three => 3,
                Rank::Four => 4,
                Rank::Five => 5,
                Rank::Six => 6,
                Rank::Seven => 7,
                Rank::Eight => 8,
                Rank::Nine => 9,
                Rank::Ten => 10,
                Rank::Jack => 11,
                Rank::Queen => 12,
                Rank::King => 13,
                Rank::Ace => 14,
            })
            .collect();
        sorted_ranks.sort_unstable();
        sorted_ranks.dedup();

        let is_straight = sorted_ranks.len() == Hand::POKER_HAND_SIZE
            && (sorted_ranks.windows(2).all(|pair| pair[1] == pair[0] + 1)
                || sorted_ranks == [2, 3, 4, 5, 14]);

        let pairs = rank_counts.values().filter(|&&count| count == 2).count();

        if is_flush && is_straight {
            Self::StraightFlush
        } else if rank_counts.values().any(|&count| count == 4) {
            Self::FourOfAKind
        } else if rank_counts.values().any(|&count| count == 3)
            && rank_counts.values().any(|&count| count == 2)
        {
            Self::FullHouse
        } else if is_flush {
            Self::Flush
        } else if is_straight {
            Self::Straight
        } else if rank_counts.values().any(|&count| count == 3) {
            Self::ThreeOfAKind
        } else if pairs >= 2 {
            Self::TwoPair
        } else if pairs == 1 {
            Self::OnePair
        } else {
            Self::HighCard
        }
    }
}

// base points awarded for each hand type (from actual Balatro values)
fn get_hand_points(hand: &PokerHand) -> u32 {
    match hand {
        PokerHand::HighCard => 5,
        PokerHand::OnePair => 10,
        PokerHand::TwoPair => 20,
        PokerHand::ThreeOfAKind => 30,
        PokerHand::Straight => 30,
        PokerHand::Flush => 35,
        PokerHand::FullHouse => 40,
        PokerHand::FourOfAKind => 60,
        PokerHand::StraightFlush => 100,
    }
}

// base multiplier awarded for each hand type (from actual Balatro values)
fn get_hand_multiplier(hand: &PokerHand) -> u32 {
    match hand {
        PokerHand::HighCard => 1,
        PokerHand::OnePair => 2,
        PokerHand::TwoPair => 2,
        PokerHand::ThreeOfAKind => 3,
        PokerHand::Straight => 4,
        PokerHand::Flush => 4,
        PokerHand::FullHouse => 4,
        PokerHand::FourOfAKind => 7,
        PokerHand::StraightFlush => 8,
    }
}

// represents a hand of cards which can have a max size.
#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>,
    max_size: usize,
}

impl Hand {
    pub const POKER_HAND_SIZE: usize = 5;
    pub const DEFAULT_BALATRO_HAND_SIZE: usize = 8;

    // create a new hand with a set max size
    pub fn new(max_size: usize) -> Self {
        Self {
            cards: Vec::with_capacity(max_size),
            max_size,
        }
    }

    pub fn cards(&self) -> &Vec<Card> {
        &self.cards
    }

    pub fn max_size(&self) -> usize {
        self.max_size
    }

    pub fn current_size(&self) -> usize {
        self.cards.len()
    }

    pub fn is_full(&self) -> bool {
        self.cards.len() >= self.max_size
    }

    // create a new hand from the cards provided, with max size set to number of cards
    pub fn new_from_cards(cards: Vec<Card>) -> Self {
        let max_size = cards.len();
        Self { cards, max_size }
    }

    // add a card, if able
    pub fn add_card(&mut self, card: Card) -> Result<(), String> {
        if self.cards.len() >= self.max_size {
            return Err(format!(
                "Cannot add more cards to hand. Max size is {}.",
                self.max_size
            ));
        }
        self.cards.push(card);
        Ok(())
    }

    // take all cards from the hand and return them
    #[allow(dead_code)]
    pub fn discard_all(&mut self) -> Result<Vec<Card>, String> {
        Ok(std::mem::take(&mut self.cards))
    }

    // discard a card at a specific index
    pub fn discard_card(&mut self, index: usize) -> Result<Card, String> {
        if index >= self.cards.len() {
            return Err(format!("Invalid index: {}", index));
        }
        Ok(self.cards.remove(index))
    }

    // get the score value for this hand, returning both the type of hand and its value
    pub fn score_hand(&self) -> (PokerHand, u32) {
        let hand_type = PokerHand::from_cards(&self.cards);

        // sum values for all cards in the hand
        // TODO need to figure out how to know which cards actually applied to the hand type.
        // maybe a second return value, with indices or references?
        let hand_value_sum: u32 = self.cards.iter().map(|card| u32::from(card.value)).sum();
        let hand_point_value = get_hand_points(&hand_type);
        let hand_multiplier = get_hand_multiplier(&hand_type);

        // return both the hand type and value
        (
            hand_type,
            hand_value_sum * hand_point_value * hand_multiplier,
        )
    }
}

// hand display
impl fmt::Display for PokerHand {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            PokerHand::HighCard => "High Card",
            PokerHand::OnePair => "Pair",
            PokerHand::TwoPair => "Two Pair",
            PokerHand::ThreeOfAKind => "Three of a Kind",
            PokerHand::Straight => "Straight",
            PokerHand::Flush => "Flush",
            PokerHand::FullHouse => "Full House",
            PokerHand::FourOfAKind => "Four of a Kind",
            PokerHand::StraightFlush => "Straight Flush",
        };
        formatter.write_str(name)
    }
}

// AI-generated tests. could probably add better and more test cases
#[cfg(test)]
mod tests {
    use super::PokerHand;
    use crate::cards::card::{Card, Rank, Suit};

    fn card(rank: Rank, suit: Suit) -> Card {
        Card::new(suit, rank, 0)
    }

    #[test]
    fn from_cards_recognizes_ace_low_straight() {
        let cards = [
            card(Rank::Ace, Suit::Clubs),
            card(Rank::Two, Suit::Diamonds),
            card(Rank::Three, Suit::Hearts),
            card(Rank::Four, Suit::Spades),
            card(Rank::Five, Suit::Clubs),
        ];

        assert_eq!(PokerHand::from_cards(&cards), PokerHand::Straight);
    }

    #[test]
    fn from_cards_distinguishes_one_pair_and_two_pair() {
        let one_pair = [
            card(Rank::Two, Suit::Clubs),
            card(Rank::Two, Suit::Diamonds),
            card(Rank::Four, Suit::Hearts),
            card(Rank::Seven, Suit::Spades),
            card(Rank::Nine, Suit::Clubs),
        ];
        let two_pair = [
            card(Rank::Two, Suit::Clubs),
            card(Rank::Two, Suit::Diamonds),
            card(Rank::Four, Suit::Hearts),
            card(Rank::Four, Suit::Spades),
            card(Rank::Nine, Suit::Clubs),
        ];

        assert_eq!(PokerHand::from_cards(&one_pair), PokerHand::OnePair);
        assert_eq!(PokerHand::from_cards(&two_pair), PokerHand::TwoPair);
    }
}
