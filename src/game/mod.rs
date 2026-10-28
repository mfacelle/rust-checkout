use crate::cards::card::{rank_symbol, suit_symbol, Suit};
use crate::cards::deck::Deck;
use crate::cards::hand::Hand;
use std::io::{self, Write};
// use std::thread;
// use std::time::Duration;

#[derive(Clone, Copy)]
enum GameState {
    NotStarted,
    WaitingForStart,
    WaitingForCommand,
    WaitingForDiscardIndices,
    WaitingForPlayIndices,
    Finished,
}

pub struct Game<O> {
    output: O,
    deck: Deck,
    hand: Hand,
    state: GameState,
    total_points: u32,
}

// TODO had to remove any sleep calls for this to run properly as a wasm.
// they were mainly there for dramatic effect, or to see errors better before
// the next turn prompt appears.  Instead, added "!!!" to the beginning so they're
// at least a bit more noticeable.  It works for now.

impl<O: Write> Game<O> {
    /// construct a new game with the deck, hand, and output provided
    pub fn new(deck: Deck, hand: Hand, output: O) -> Self {
        Self {
            output,
            deck,
            hand,
            state: GameState::NotStarted,
            total_points: 0,
        }
    }

    /// main game loop
    /// advanced by externally supplied input
    pub fn start_game(&mut self) -> io::Result<()> {
        if !matches!(self.state, GameState::NotStarted) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "game has already been started",
            ));
        }

        self.display_welcome_msg()?;
        writeln!(self.output, "Press enter to begin")?;
        // process any line, as long as player presses enter
        self.state = GameState::WaitingForStart;
        Ok(())
    }

    /// run the main game loop, processing inputs and totaling score.
    /// TODO add a limit to number of draws/discards allowed and number of hands playable
    /// TODO add logic to handle "end of game" when last hand is played, and display final score
    /// process one line supplied by an external caller
    pub fn process_input(&mut self, input: &str) -> io::Result<()> {
        match self.state {
            // game not started, no input to process
            GameState::NotStarted => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "game has not been started",
                ));
            }
            // process any input to draw cards and start the game.
            // just doing this as a way to let player see welcome message and 
            // hit enter to actually start.
            GameState::WaitingForStart => {
                writeln!(self.output)?;
                writeln!(self.output, "Drawing cards...")?;

                // brief pause, for dramatic effect
                // thread::sleep(Duration::from_millis(500));

                for _ in 0..self.hand.max_size() {
                    if let Some(card) = self.deck.draw() {
                        self.hand.add_card(card).map_err(io::Error::other)?;
                    }
                }

                // prompt for next command
                self.state = GameState::WaitingForCommand;
                self.display_turn()?;
            }
            // process a regular command
            GameState::WaitingForCommand => self.process_command(input)?,
            // process a list of indices, to be discarded
            GameState::WaitingForDiscardIndices => {
                self.process_discard_command(input)?;

                // prompt for next command
                self.state = GameState::WaitingForCommand;
                self.display_turn()?;
            }
            // process a list of indices, to be played
            GameState::WaitingForPlayIndices => {
                self.total_points += self.process_play_command(input)?;

                // prompt for next command
                self.state = GameState::WaitingForCommand;
                self.display_turn()?;
            }
            // game ended, no input to process
            GameState::Finished => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "game has already finished",
                ));
            }
        }
        Ok(())
    }

    /// check if game is finished
    pub fn is_finished(&self) -> bool {
        matches!(self.state, GameState::Finished)
    }

    /// end the game
    /// TODO need to actually add win/lose conditions
    pub fn quit_game(&mut self) -> io::Result<()> {
        if !self.is_finished() {
            writeln!(self.output, "Exiting - thanks for playing!")?;
            self.state = GameState::Finished;
        }
        Ok(())
    }

    /// display player info and prompt for command input
    fn display_turn(&mut self) -> io::Result<()> {
        // display game info
        writeln!(self.output)?;
        writeln!(self.output, "-----------------------------------------")?;
        writeln!(self.output)?;
        writeln!(
            self.output,
            "Your hand: (current points: {})",
            self.total_points
        )?;
        Self::display_hand(&mut self.output, &self.hand)?;
        writeln!(self.output)?;
        
        Self::display_command_prompt(&mut self.output, self.deck.len())?;

        // no newline, so inputs are appended like a regular terminal
        write!(self.output, "> ")?;
        self.output.flush()?;
        Ok(())
    }

    /// process a command, as a line of text input
    fn process_command(&mut self, input: &str) -> io::Result<()> {
        // TODO should figure out a way to add args to each command,
        // to do things like "discard 1 3 5" instead of "discard" then "1 3 5"
        match input.trim() {
            "draw" => {
                self.process_draw_command()?;
                self.display_turn()?;
            }
            "discard" => {
                if self.hand.current_size() == 0 {
                    writeln!(self.output)?;
                    writeln!(self.output, "!!! You have no cards to discard!")?;

                    // brief delay to allow player to see error better
                    // thread::sleep(Duration::from_secs(1));
                    self.display_turn()?;
                } else {
                    writeln!(
                        self.output,
                        "Type the index of the cards you want to discard (with spaces between them): "
                    )?;
                    write!(self.output, "> ")?;
                    self.output.flush()?;
                    self.state = GameState::WaitingForDiscardIndices;
                }
            }
            "play" => {
                if self.hand.current_size() == 0 {
                    writeln!(self.output)?;
                    writeln!(self.output, "!!! You have no cards to play!")?;

                    // brief delay to allow player to see error better
                    // thread::sleep(Duration::from_secs(1));
                    self.display_turn()?;
                } else {
                    writeln!(
                        self.output,
                        "Type the index of up to 5 cards you want to play (with spaces between them): "
                    )?;
                    write!(self.output, "> ")?;
                    self.output.flush()?;
                    self.state = GameState::WaitingForPlayIndices;
                }
            }
            "quit" => {
                self.quit_game()?;
            }
            "" => self.display_turn()?,
            command => {
                writeln!(self.output)?;
                writeln!(self.output, "!!! Unknown command: {command}")?;

                // brief delay to allow player to see error better
                // thread::sleep(Duration::from_secs(1));
                self.display_turn()?;
            }
        }
        Ok(())
    }

    /// handles player input for discarding cards
    fn process_discard_command(&mut self, input: &str) -> io::Result<()> {
        // TODO processing input into list of indices is the same as play command,
        // consider making this a separate function
        // get input line from player
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
                Some(hand_index) => match self.hand.discard_card(hand_index) {
                    Ok(card) => writeln!(self.output, "Discarding {card}")?,
                    Err(err) => {
                        writeln!(self.output)?;
                        writeln!(self.output, "!!! {err}")?;
                        // brief delay to allow player to see error better
                        // thread::sleep(Duration::from_secs(1));
                    }
                },
                None => {
                    writeln!(self.output)?;
                    writeln!(self.output, "!!! Invalid index: {index}")?;
                    // brief delay to allow player to see error better
                    // thread::sleep(Duration::from_secs(1));
                }
            }
        }

        Ok(())
    }

    /// process a draw command
    fn process_draw_command(&mut self) -> io::Result<()> {
        if !self.hand.is_full() {
            if let Some(card) = self.deck.draw() {
                self.hand.add_card(card).map_err(io::Error::other)?;
            }
        } else {
            writeln!(self.output)?;
            writeln!(self.output, "!!! Your hand is full!")?;

            // brief delay to allow player to see error better
            // thread::sleep(Duration::from_secs(1));
        }
        Ok(())
    }

    /// handle player input for playing a poker hand
    fn process_play_command(&mut self, input: &str) -> io::Result<u32> {
        // get input line from player
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
            writeln!(self.output, "!!! You can only play up to 5 cards!")?;

            // brief delay to allow player to see error better
            // thread::sleep(Duration::from_secs(1));
            return Ok(0);
        } else if indices.is_empty() {
            writeln!(self.output)?;
            writeln!(self.output, "!!! You must play at least one card!")?;

            // brief delay to allow player to see error better
            // thread::sleep(Duration::from_secs(1));
            return Ok(0);
        }

        // remove cards from hand and place them into a temporary hand to be played
        let mut played_cards = Vec::with_capacity(indices.len());
        for index in indices {
            // displayed indices are 1-indexed, so reduce by 1
            match index.checked_sub(1) {
                Some(hand_index) => match self.hand.discard_card(hand_index) {
                    Ok(card) => {
                        writeln!(self.output, "Playing {card}")?;
                        played_cards.push(card);
                    }
                    Err(err) => {
                        writeln!(self.output)?;
                        writeln!(self.output, "!!! {err}")?;
                        // brief delay to allow player to see error better
                        // thread::sleep(Duration::from_secs(1));
                    }
                },
                None => {
                    writeln!(self.output)?;
                    writeln!(self.output, "!!! Invalid index: {index}")?;
                    // brief delay to allow player to see error better
                    // thread::sleep(Duration::from_secs(1));
                }
            }
        }

        // create hand object for cards to be played and score it
        let played_hand = Hand::new_from_cards(played_cards.clone());

        writeln!(self.output)?;
        writeln!(self.output, "Playing hand:")?;
        Self::display_hand(&mut self.output, &played_hand)?;
        writeln!(self.output)?;

        let (hand_type, hand_value) = played_hand.score_hand();
        writeln!(self.output)?;
        writeln!(
            self.output,
            "You played a {} worth {} points!",
            hand_type, hand_value
        )?;
        writeln!(self.output)?;

        // brief delay to allow player to see it better
        // thread::sleep(Duration::from_secs(2));
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
    fn display_hand(output: &mut O, hand: &Hand) -> io::Result<()> {
        // top of card
        for _ in hand.cards() {
            write!(output, " .-----. ")?;
        }
        writeln!(output)?;

        // second line, includes card rank
        for card in hand.cards() {
            write!(output, " |{:<2}   | ", rank_symbol(card.rank))?;
        }
        writeln!(output)?;

        // third line, includes suit symbol
        for card in hand.cards() {
            write!(output, " |  {}  | ", suit_symbol(card.suit))?;
        }
        writeln!(output)?;

        // fourth line, includes card rank
        for card in hand.cards() {
            write!(output, " |   {:>2}| ", rank_symbol(card.rank))?;
        }
        writeln!(output)?;

        // bottom of card
        for _ in hand.cards() {
            write!(output, " '-----' ")?;
        }
        writeln!(output)?;

        // index, for player to reference
        // assumes the hand will always have less than 10 cards, or numbers get thrown off.
        // need to enforce this in main or on game construction
        for idx in 0..hand.current_size() {
            write!(output, "   ({})   ", idx + 1)?;
        }
        writeln!(output)?;
        Ok(())
    }

    /// displays the available commands
    /// TODO will likely want to display how many draws/discards/plays are left
    fn display_command_prompt(output: &mut O, cards_remaining: usize) -> io::Result<()> {
        writeln!(output, "Commands:")?;
        writeln!(output, "   draw ({cards_remaining} cards left in deck)")?;
        writeln!(output, "   discard")?;
        writeln!(output, "   play")?;
        writeln!(output, "   quit")?;
        Ok(())
    }
}
