use crate::cards::card::{rank_symbol, suit_symbol, Suit};
use crate::cards::deck::Deck;
use crate::cards::hand::Hand;
use std::io::{self, BufRead, Write};
use std::thread;
use std::time::Duration;

pub struct Game<I, O> {
    input: I,
    output: O,
}

impl<I: BufRead, O: Write> Game<I, O> {
    /// construct a new game with the i/o provided
    pub fn new(input: I, output: O) -> Self {
        Self { input, output }
    }

    /// main game loop
    pub fn run_game(&mut self, deck: &mut Deck, hand: &mut Hand) -> io::Result<()> {
        self.display_welcome_msg()?;
        writeln!(self.output, "Press enter to begin")?;

        // process any line, as long as player presses enter
        let mut input = String::new();
        self.input.read_line(&mut input)?;

        writeln!(self.output)?;
        writeln!(self.output, "Drawing cards...")?;
        // brief pause, for dramatic effect
        thread::sleep(Duration::from_millis(500));
        for _ in 0..hand.max_size() {
            if let Some(card) = deck.draw() {
                hand.add_card(card).map_err(io::Error::other)?;
            }
        }

        writeln!(self.output)?;

        // run the main game loop, processing inputs and totaling score.
        // TODO add a limit to number of draws/discards allowed and number of hands playable
        // TODO add logic to handle "end of game" when last hand is played, and display final score
        let mut total_points: u32 = 0;
        loop {
            // display game info and main command prompt
            writeln!(self.output)?;
            writeln!(self.output, "-----------------------------------------")?;
            writeln!(self.output)?;
            writeln!(self.output, "Your hand: (current points: {total_points})")?;
            self.display_hand(hand)?;

            writeln!(self.output)?;
            self.display_command_prompt(deck.len())?;
            write!(self.output, "> ")?;
            self.output.flush()?;

            input.clear();
            if self.input.read_line(&mut input)? == 0 {
                break;
            }

            // TODO should figure out a way to add args to each command,
            // to do things like "discard 1 3 5" instead of "discard" then "1 3 5"
            match input.trim() {
                "draw" => self.process_draw_command(deck, hand)?,
                "discard" => self.process_discard_command(hand)?,
                "play" => total_points += self.process_play_command(hand)?,
                "quit" => break,
                "" => {}
                command => {
                    writeln!(self.output, "Unknown command: {command}")?;

                    // brief delay to allow player to see error better
                    thread::sleep(Duration::from_secs(1));
                }
            }

        }

        writeln!(self.output, "Exiting - thanks for playing!")?;
        Ok(())
    }

    /// handles player input for discarding cards
    fn process_discard_command(&mut self, hand: &mut Hand) -> io::Result<()> {
        if hand.current_size() == 0 {
            writeln!(self.output)?;
            writeln!(self.output, "You have no cards to discard!")?;

            // brief delay to allow player to see error better
            thread::sleep(Duration::from_secs(1));
            return Ok(());
        }

        writeln!(
            self.output,
            "Type the index of the cards you want to discard (with spaces between them): "
        )?;
        write!(self.output, "> ")?;
        self.output.flush()?;

        // TODO processing input into list of indices is the same as play command,
        // consider making this a separate function
        // get input line from player
        let mut input = String::new();
        if self.input.read_line(&mut input)? == 0 {
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
        for index in indices {
            // displayed indices are 1-indexed, so reduce by 1 (safely)
            match index.checked_sub(1) {
                Some(hand_index) => match hand.discard_card(hand_index) {
                    Ok(card) => writeln!(self.output, "Discarding {card}")?,
                    Err(err) => {
                        writeln!(self.output, "{err}")?;
                        // brief delay to allow player to see error better
                        thread::sleep(Duration::from_secs(1));
                    }
                },
                None => {
                    writeln!(self.output, "Invalid index: {index}")?;
                    // brief delay to allow player to see error better
                    thread::sleep(Duration::from_secs(1));
                }
            }
        }

        Ok(())
    }

    /// process a draw command
    fn process_draw_command(&mut self, deck: &mut Deck, hand: &mut Hand) -> io::Result<()> {
        if !hand.is_full() {
            if let Some(card) = deck.draw() {
                hand.add_card(card).map_err(io::Error::other)?;
            }
        } else {
            writeln!(self.output)?;
            writeln!(self.output, "Your hand is full!")?;

            // brief delay to allow player to see error better
            thread::sleep(Duration::from_secs(1));
        }
        Ok(())
    }

    /// handle player input for playing a poker hand
    fn process_play_command(&mut self, hand: &mut Hand) -> io::Result<u32> {
        if hand.current_size() == 0 {
            writeln!(self.output)?;
            writeln!(self.output, "You have no cards to play!")?;

            // brief delay to allow player to see error better
            thread::sleep(Duration::from_secs(1));
            return Ok(0);
        }

        writeln!(
            self.output,
            "Type the index of up to 5 cards you want to play (with spaces between them): "
        )?;
        write!(self.output, "> ")?;
        self.output.flush()?;

        // get input line from player
        let mut input = String::new();
        if self.input.read_line(&mut input)? == 0 {
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

        // ensure valid number of cards played
        if indices.len() > 5 {
            writeln!(self.output)?;
            writeln!(self.output, "You can only play up to 5 cards!")?;

            // brief delay to allow player to see error better
            thread::sleep(Duration::from_secs(1));
            return Ok(0);
        } else if indices.is_empty() {
            writeln!(self.output)?;
            writeln!(self.output, "You must play at least one card!")?;

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
                        writeln!(self.output, "Playing {card}")?;
                        played_cards.push(card);
                    }
                    Err(err) => {
                        writeln!(self.output, "{err}")?;
                        // brief delay to allow player to see error better
                        thread::sleep(Duration::from_secs(1));
                    }
                },
                None => {
                    writeln!(self.output, "Invalid index: {index}")?;
                    // brief delay to allow player to see error better
                    thread::sleep(Duration::from_secs(1));
                }
            }
        }

        // create hand object for cards to be played and score it
        let played_hand = Hand::new_from_cards(played_cards.clone());

        writeln!(self.output)?;
        writeln!(self.output, "Playing hand:")?;
        self.display_hand(&played_hand)?;
        writeln!(self.output)?;

        let (hand_type, hand_value) = played_hand.score_hand();
        writeln!(
            self.output,
            "You played a {} worth {} points!",
            hand_type, hand_value
        )?;

        // brief delay to allow player to see it better
        thread::sleep(Duration::from_secs(2));
        Ok(hand_value)
    }

    /// displays a basic welcome message
    fn display_welcome_msg(&mut self) -> io::Result<()> {
        writeln!(self.output)?;
        writeln!(
            self.output,
            "~-------------------------------------------------------------------------~"
        )?;
        writeln!(self.output)?;
        writeln!(self.output, "Welcome to Mike's Rust-y version of Balatro!")?;
        writeln!(self.output)?;
        writeln!(
            self.output,
            "~-------------------------------------------------------------------------~"
        )?;
        writeln!(
            self.output,
            "Goal: create and play poker hands to score the most points!"
        )?;
        writeln!(
            self.output,
            "~-------------------------------------------------------------------------~"
        )?;
        writeln!(
            self.output,
            "For reference, using the following symbols for each suit: "
        )?;
        for suit in [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
            writeln!(self.output, "   {} - {}", suit_symbol(suit), suit)?;
        }
        writeln!(
            self.output,
            "~-------------------------------------------------------------------------~"
        )?;
        writeln!(self.output)?;
        Ok(())
    }

    /// displays the player's active hand.
    /// needs to loop over each card for every line, so it doesn't assume a fixed size
    fn display_hand(&mut self, hand: &Hand) -> io::Result<()> {
        // top of card
        for _ in hand.cards() {
            write!(self.output, " .-----. ")?;
        }
        writeln!(self.output)?;

        // second line, includes card rank
        for card in hand.cards() {
            write!(self.output, " |{:<2}   | ", rank_symbol(card.rank))?;
        }
        writeln!(self.output)?;

        // third line, includes suit symbol
        for card in hand.cards() {
            write!(self.output, " |  {}  | ", suit_symbol(card.suit))?;
        }
        writeln!(self.output)?;

        // fourth line, includes card rank
        for card in hand.cards() {
            write!(self.output, " |   {:>2}| ", rank_symbol(card.rank))?;
        }
        writeln!(self.output)?;

        // bottom of card
        for _ in hand.cards() {
            write!(self.output, " '-----' ")?;
        }
        writeln!(self.output)?;

        // index, for player to reference
        // assumes the hand will always have less than 10 cards, or numbers get thrown off.
        // need to enforce this in main or on game construction
        for idx in 0..hand.current_size() {
            write!(self.output, "   ({})   ", idx + 1)?;
        }
        writeln!(self.output)?;
        Ok(())
    }

    // displays the available commands
    fn display_command_prompt(&mut self, cards_remaining: usize) -> io::Result<()> {
        writeln!(self.output, "Commands:")?;
        writeln!(
            self.output,
            "   draw ({cards_remaining} cards left in deck)"
        )?;
        writeln!(self.output, "   discard")?;
        writeln!(self.output, "   play")?;
        writeln!(self.output, "   quit")?;
        Ok(())
    }
}
