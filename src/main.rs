mod cards;
mod game;

use cards::deck::Deck;
use cards::hand::Hand;
use std::io;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut game = game::Game::new(stdin.lock(), stdout.lock());

    let mut deck = Deck::standard_deck();

    deck.shuffle();

    let mut hand = Hand::new(Hand::DEFAULT_BALATRO_HAND_SIZE);

    game.run_game(&mut deck, &mut hand)
}