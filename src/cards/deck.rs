use super::card::{rank_value, Card, Rank, Suit};

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    // construct a deck from the provided set of cards
    // this isn't actually used right now
    #[allow(dead_code)]
    pub fn new(cards: Vec<Card>) -> Self {
        Self { cards }
    }

    // construct a deck of the standard 52 cards
    pub fn standard_deck() -> Self {
        let suits = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
        let ranks = [
            (Rank::Two, rank_value(Rank::Two)),
            (Rank::Three, rank_value(Rank::Three)),
            (Rank::Four, rank_value(Rank::Four)),
            (Rank::Five, rank_value(Rank::Five)),
            (Rank::Six, rank_value(Rank::Six)),
            (Rank::Seven, rank_value(Rank::Seven)),
            (Rank::Eight, rank_value(Rank::Eight)),
            (Rank::Nine, rank_value(Rank::Nine)),
            (Rank::Ten, rank_value(Rank::Ten)),
            (Rank::Jack, rank_value(Rank::Jack)),
            (Rank::Queen, rank_value(Rank::Queen)),
            (Rank::King, rank_value(Rank::King)),
            (Rank::Ace, rank_value(Rank::Ace)),
        ];
        let cards = suits
            .into_iter()
            .flat_map(|suit| ranks.map(|(rank, value)| Card::new(suit, rank, value)))
            .collect();

        Self { cards }
    }

    // gets length of deck
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    // draws a card, removing it from the top of the deck
    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    // shuffles the cards in this deck
    pub fn shuffle(&mut self) {
        // easy way to implement this, use built-in rand shuffle function:
        // let mut rng = rand::rng();
        // self.cards.shuffle(&mut rng);

        // but I'd rather write one myself for the learning experience.
        // using Fisher-Yates shuffle algorithm, which ended up also being easy to implement
        use rand::RngExt;
        let mut rng = rand::rng();

        // reverse through vector and swap each element with a random card in the unshuffled portion
        for i in (1..self.cards.len()).rev() {
            let j = rng.random_range(0..=i);
            self.cards.swap(i, j);
        }
    }
}

// AI-generated tests for Deck (kind of overkill for how simple this class is...)
#[cfg(test)]
mod tests {
    use super::super::card::Rank;
    use super::Deck;

    #[test]
    fn standard_deck_deck_has_52_cards() {
        assert_eq!(Deck::standard_deck().len(), 52);
    }

    #[test]
    fn drawing_removes_a_card() {
        let mut deck = Deck::standard_deck();

        assert!(deck.draw().is_some());
        assert_eq!(deck.len(), 51);
    }

    #[test]
    fn card_values_match_ranks() {
        let mut deck = Deck::standard_deck();
        let cards: Vec<_> = std::iter::from_fn(|| deck.draw()).collect();

        for card in cards {
            let expected_value = match card.rank {
                Rank::Two => 2,
                Rank::Three => 3,
                Rank::Four => 4,
                Rank::Five => 5,
                Rank::Six => 6,
                Rank::Seven => 7,
                Rank::Eight => 8,
                Rank::Nine => 9,
                Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
                Rank::Ace => 11,
            };
            assert_eq!(card.value, expected_value);
        }
    }
}
