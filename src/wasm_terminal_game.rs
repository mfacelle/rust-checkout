use crate::cards::deck::Deck;
use crate::cards::hand::Hand;
use crate::game::Game;
use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;
use wasm_bindgen::prelude::*;

// sets up and handles I/O from xterm.js.
// Does not use the main method that a normal CLI build would

struct TerminalOutput(Rc<RefCell<Vec<u8>>>);

impl Write for TerminalOutput {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0.borrow_mut().extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[wasm_bindgen]
pub struct WasmTerminalGame {
    game: Game<TerminalOutput>,
    output: Rc<RefCell<Vec<u8>>>,
}


#[wasm_bindgen]
impl WasmTerminalGame {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // set up inputs for the game and terminal, but don't start it yet
        let output = Rc::new(RefCell::new(Vec::new()));
        let mut deck = Deck::standard_deck();
        deck.shuffle();
        let hand = Hand::new(Hand::DEFAULT_BALATRO_HAND_SIZE);
        let game = Game::new(deck, hand, TerminalOutput(Rc::clone(&output)));

        Self { game, output }
    }

    /// start the game (callable via javascript)
    pub fn start(&mut self) -> Result<String, JsValue> {

        // added to get better error messages/debugging (not sure if it's really working, though)
        console_error_panic_hook::set_once();

        self.game
            .start_game()
            .map_err(|error| JsValue::from_str(&error.to_string()))?;
        self.take_output()
    }

    /// pass a line of input to the game (callable via javascript)
    pub fn input(&mut self, input: &str) -> Result<String, JsValue> {
        self.game
            .process_input(input)
            .map_err(|error| JsValue::from_str(&error.to_string()))?;
        self.take_output()
    }
}

impl WasmTerminalGame {
    fn take_output(&self) -> Result<String, JsValue> {
        String::from_utf8(std::mem::take(&mut *self.output.borrow_mut()))
            .map_err(|error| JsValue::from_str(&error.to_string()))
    }
}

impl Default for WasmTerminalGame {
    fn default() -> Self {
        Self::new()
    }
}
