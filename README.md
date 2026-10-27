# rust-checkout
Project for learning and exploring Rust.

## Details
Implements a simplistic command-line version of the game, Balatro (https://www.playbalatro.com), written in Rust.  The game can be run in a normal terminal, or via a JavaScript xterm.js in a browser.  Will be hosting the javascript terminal version on my github pages (mfacelle.github.io).

Created some basic modules for data management (decks, cards, etc), and I/O (prompts and input handling).

Currently working on preparing for xterm.js and wasm support, which requires updating how input handling is done to make it asynchronous and not blocking calls.

## Running
Run this on a command line with:

```sh
cargo run
```

The game itself should provide enough information to make it clear how to play it.