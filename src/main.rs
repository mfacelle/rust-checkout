mod cards;
mod game;

use cards::deck::Deck;
use cards::hand::Hand;
use std::io;

fn main() -> io::Result<()> {
    let mut deck = Deck::standard_deck();

    deck.shuffle();

    let mut hand = Hand::new(Hand::DEFAULT_BALATRO_HAND_SIZE);

    game::game_io::run_game(&mut deck, &mut hand)?;

    Ok(())
}