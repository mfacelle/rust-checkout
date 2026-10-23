use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

// get the value of a specific card rank
pub(crate) fn rank_value(rank: Rank) -> u8 {
    match rank {
        Rank::Two => 2,
        Rank::Three => 3,
        Rank::Four => 4,
        Rank::Five => 5,
        Rank::Six => 6,
        Rank::Seven => 7,
        Rank::Eight => 8,
        Rank::Nine => 9,
        Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
        Rank::Ace => 11, // assuming aces always high for now
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
    pub value: u8,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank, value: u8) -> Self {
        Self { suit, rank, value }
    }
}

// display formatter for suit
impl fmt::Display for Suit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Suit::Clubs => "Clubs",
            Suit::Diamonds => "Diamonds",
            Suit::Hearts => "Hearts",
            Suit::Spades => "Spades",
        };
        formatter.write_str(name)
    }
}

// display formatter for rank
impl fmt::Display for Rank {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "Jack",
            Rank::Queen => "Queen",
            Rank::King => "King",
            Rank::Ace => "Ace",
        };
        formatter.write_str(name)
    }
}

// display formatter for card, prints rank/suit/value
impl fmt::Display for Card {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} of {} ({})", self.rank, self.suit, self.value)
    }
}
