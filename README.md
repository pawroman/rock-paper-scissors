# rock-paper-scissors

A simple rock paper scissors game implementation in Rust.

The implementation makes it easy to modify the rules of the game.


## Running

To run:

```
$ cargo run
```

Example game:

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/rock-paper-scissors`
=== Rock Paper Scissors Game ===

Your move (Rock / Paper / Scissors / Quit)? >> p
You play : Paper
CPU plays: Paper

Result: you Draw!
Current score: 0

Your move (Rock / Paper / Scissors / Quit)? >> s
You play : Scissors
CPU plays: Paper

Result: you Win!
Current score: 1

Your move (Rock / Paper / Scissors / Quit)? >> r
You play : Rock
CPU plays: Paper

Result: you Lose!
Current score: 0

Your move (Rock / Paper / Scissors / Quit)? >> q
Game over.

```
