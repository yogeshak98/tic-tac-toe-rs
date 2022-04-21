# TicTacToe

An AI enabled Tic Tac Toe Game.
The game uses random, poss-win and minimax algorithms to simulate gameplay by the computer.

## Modes
### Easy
The game generates a random valid move.
### Moderate
The game blocks any player from completing the triad if possible, but plays a random move if no triad is being completed.
### Impossible
The game uses minimax method to find the best possible move to win. It is impossible for the player to win.

## Setup
### Dependencies
* Install [rustup](https://doc.rust-lang.org/book/ch01-01-installation.html)
* Install "wasm-pack" -> cargo install wasm-pack
* Install npm dependencies -> npm install (must have [Node JS](https://nodejs.org/))

### Compilation
* Compile the rust code every time the changes are made in `src` directory
* Compile rust code into web-assembly `wasm-pack build --target web --release`
* To start the server run `npm start`

 Open the browser on localhost:3000
