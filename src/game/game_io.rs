use crate::cards::card::{suit_symbol, rank_symbol, Suit};
use crate::cards::deck::Deck;
use crate::cards::hand::Hand;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

// main game loop
pub fn run_game(deck: &mut Deck, hand: &mut Hand) -> io::Result<()> {

    // initial prompts
    display_welcome_msg();
    println!("Press enter to begin");
    
    // process any line, as long as player presses enter
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    println!("");
    println!("Drawing cards...");
    // brief pause, for dramatic effect
    thread::sleep(Duration::from_secs(1));
    for _ in 0..hand.max_size() {
        if let Some(card) = deck.draw() {
            hand.add_card(card).unwrap();
        }
    }
    println!("");

    // run the main game loop, processing inputs and totaling score.
    // TODO add a limit to number of draws/discards allowed and number of hands playable
    // TODO add logic to handle "end of game" when last hand is played, and display final score
    let mut total_points: u32 = 0;
    loop {
        println!("");
        println!("-----------------------------------------");
        println!("");
        println!("Your hand: (current points: {total_points})");
        display_hand(hand);
    
        println!("");
        display_command_prompt(deck.len());
        print!("> ");
        io::stdout().flush()?;

        input.clear();
        if io::stdin().read_line(&mut input)? == 0 {
            break;
        }

        // TODO should figure out a way to add args to each command,
        // to do things like "discard 1 3 5" instead of "discard" then "1 3 5"
        match input.trim() {
            "draw" => process_draw_command(deck, hand),
            "discard" => process_discard_command(hand)?,
            "play" => total_points +=process_play_command(hand)?,
            "quit" => break,
            "" => {}
            command => {
                println!("Unknown command: {command}");

                // brief delay to allow player to see error better
                thread::sleep(Duration::from_secs(1));
            }
        }
    }

    println!("Exiting - thanks for playing!");

    Ok(())
}


// handles player input for discarding cards
fn process_discard_command(hand: &mut Hand) -> io::Result<()> {
    if hand.current_size() == 0 {
        println!("");
        println!("You have no cards to discard!");

        // brief delay to allow player to see error better
        thread::sleep(Duration::from_secs(1));

        return Ok(());
    }

    println!("Type the index of the cards you want to discard (with spaces between them): ");
    print!("> ");
    io::stdout().flush()?;

    // get input line from player
    let mut input = String::new();
    if io::stdin().read_line(&mut input)? == 0 {
        // this means stdin was closed, so return
        return Ok(());
    }
    
    // split into indices of cards to discard
    let indices: Vec<usize> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();
    
    // remove higher indices first so earlier removals don't change later selections.
    let mut indices = indices;
    indices.sort_unstable_by(|left, right| right.cmp(left));
    indices.dedup();

    // discard card at each index
    for &index in &indices {
        // displayed indices are 1-indexed, so reduce by 1
        match hand.discard_card(index - 1) {
            Ok(card) => println!("Discarding {card}"),
            Err(err) => println!("{}", err),
        }
    }

    Ok(())
}


// process a draw command
fn process_draw_command(deck: &mut Deck, hand: &mut Hand) {
    if !hand.is_full() {
        if let Some(card) = deck.draw() {
            hand.add_card(card).unwrap();
        }
    }
    else {
        println!("");
        println!("Your hand is full!");

        // brief delay to allow player to see error better
        thread::sleep(Duration::from_secs(1));
    }
}


// handle player input for playing a poker hand
fn process_play_command(hand: &mut Hand) -> io::Result<u32> {
    if hand.current_size() == 0 {
        println!("");
        println!("You have no cards to play!");

        // brief delay to allow player to see error better
        thread::sleep(Duration::from_secs(1));

        return Ok(0);
    }

    println!("Type the index of up to 5 cards you want to play (with spaces between them): ");
    print!("> ");
    io::stdout().flush()?;

    // get input line from player
    let mut input = String::new();
    if io::stdin().read_line(&mut input)? == 0 {
        // this means stdin was closed, so return
        return Ok(0);
    }
    
    // split into indices of cards to play, ensure no duplicates, and size is at most 5
    let indices: Vec<usize> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();
    
    // remove higher indices first so earlier removals don't change later selections.
    let mut indices = indices;
    indices.sort_unstable_by(|left, right| right.cmp(left));
    indices.dedup();
    if indices.len() > 5 {
        println!("");
        println!("You can only play up to 5 cards!");

        // brief delay to allow player to see error better
        thread::sleep(Duration::from_secs(1));

        return Ok(0);
    } else if indices.is_empty() {
        println!("");
        println!("You must play at least one card!");

        // brief delay to allow player to see error better
        thread::sleep(Duration::from_secs(1));

        return Ok(0);
    }

    // remove cards from hand and place them into a temporary hand to be played
    let mut played_cards = Vec::with_capacity(indices.len());
    for index in indices {
        // displayed indices are 1-indexed, so reduce by 1
        match index.checked_sub(1) {
            Some(hand_index) => match hand.discard_card(hand_index) {
                Ok(card) => {
                    println!("Playing {card}");
                    played_cards.push(card);
                }
                Err(err) => println!("{err}"),
            },
            None => println!("Invalid index: {index}"),
        }
    }

    // score the hand
    let played_hand = Hand::new_from_cards(played_cards.clone());

    println!("");
    println!("Playing hand:");
    display_hand(&played_hand);
    println!("");
    let (hand_type, hand_value) = played_hand.score_hand();
    println!("You played a {} worth {} points!", hand_type, &hand_value);

    // brief delay to allow player to see it better
    thread::sleep(Duration::from_secs(2));

    Ok(hand_value)
}


// displays a basic welcome message with some game info
pub fn display_welcome_msg() {
    println!("");
    println!("~-------------------------------------------------------------------------~");
    println!("");
    println!("Welcome to Mike's Rust-y version of Balatro!");
    println!("");
    println!("~-------------------------------------------------------------------------~");
    println!("Goal: create and play poker hands to score the most points!");
    println!("~-------------------------------------------------------------------------~");
    println!("For reference, using the following symbols for each suit: ");
    for suit in [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
        println!("   {} - {}", suit_symbol(suit), suit);
    }
    println!("~-------------------------------------------------------------------------~");
    println!("");
}


// displays the player's active hand
pub fn display_hand(hand: &Hand) {
    // top of card
    for _ in hand.cards() {
        print!(" .-----. ");
    }
    println!("");

    // second line, includes card rank
    for card in hand.cards() {
        print!(" |{:<2}   | ", rank_symbol(card.rank));
    }
    println!("");

    // third line, includes suit symbol
    for card in hand.cards() {
        print!(" |  {}  | ", suit_symbol(card.suit));
    }
    println!("");

    // fourth line, includes card rank
    for card in hand.cards() {
        print!(" |   {:>2}| ", rank_symbol(card.rank));
    }
    println!("");

    // bottom of card
    for _ in hand.cards() {
        print!(" '-----' ");
    }
    println!("");

    // index, for player to reference
    // assumes the hand will always have less than 10 cards, or numbers get thrown off.
    for idx in 0..hand.current_size() {
        print!("   ({})   ", idx + 1);
    }
    println!("");
}


// displays the available commands
fn display_command_prompt(cards_remaining: usize) {
    println!("Commands:");
    println!("   draw ({cards_remaining} cards left in deck)");
    println!("   discard");
    println!("   play");
    println!("   quit");
}
