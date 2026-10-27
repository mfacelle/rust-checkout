use std::collections::VecDeque;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Default)]
pub struct XtermIo {
    incoming: VecDeque<String>,
    outgoing: String,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl XtermIo {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new() -> Self {
        Self::default()
    }

    /// Queue a string received from xterm.js.
    pub fn receive_from_terminal(&mut self, input: &str) {
        self.incoming.push_back(input.to_owned());
    }

    /// Retrieve the next string received from xterm.js, if any.
    pub fn read_input(&mut self) -> Option<String> {
        self.incoming.pop_front()
    }

    /// Queue a string to be returned to xterm.js.
    pub fn write_to_terminal(&mut self, output: &str) {
        self.outgoing.push_str(output);
    }

    /// Return all pending output for xterm.js and clear the output buffer.
    pub fn take_output(&mut self) -> String {
        std::mem::take(&mut self.outgoing)
    }
}

#[cfg(test)]
mod tests {
    use super::XtermIo;

    #[test]
    fn transports_input_and_output_strings() {
        let mut io = XtermIo::new();

        assert_eq!(io.read_input(), None);
        io.receive_from_terminal("play 1 3");
        io.receive_from_terminal("quit");
        assert_eq!(io.read_input().as_deref(), Some("play 1 3"));
        assert_eq!(io.read_input().as_deref(), Some("quit"));
        assert_eq!(io.read_input(), None);

        io.write_to_terminal("Game output\n");
        io.write_to_terminal("Next prompt");
        assert_eq!(io.take_output(), "Game output\nNext prompt");
        assert_eq!(io.take_output(), "");
    }
}
