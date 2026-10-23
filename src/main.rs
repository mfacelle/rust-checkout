mod card;
mod deck;

use deck::Deck;

fn main() {
    let mut deck = Deck::standard_deck();
    println!("Created a standard deck with {} cards.", deck.len());

    println!("Shuffling the deck and drawing 10 cards");
    
    deck.shuffle();

    for _ in 0..10 {
        if let Some(card) = deck.draw() {
            println!("Drew: {card}.");
        }
    }

    println!("{} cards remain.", deck.len());
}
