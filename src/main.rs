mod cards;
mod game;

use cards::deck::Deck;
use cards::hand::Hand;
use std::io;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut deck = Deck::standard_deck();
    deck.shuffle();
    let hand = Hand::new(Hand::DEFAULT_BALATRO_HAND_SIZE);

    let mut game = game::Game::new(deck, hand, stdout.lock());

    game.start_game()?;

    // process lines as player inputs them, and pass them to the game.
    // this is done so that xterm.js, which will provide inputs asynchronously,
    // can be handled as well as stdin.
    let mut input = String::new();
    while !game.is_finished() {
        input.clear();
        if stdin.read_line(&mut input)? == 0 {
            // this means stdin was closed, so end the game and exit
            game.quit_game()?;
            break;
        }
        game.process_input(&input)?;
    }

    Ok(())
}
