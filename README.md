# rock-paper-scissors

A simple rock paper scissors game implementation in Rust.


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


## Modifying

The implementation makes it easy to modify the rules of the game.

For example, to add a `Clip` hand that beats `Paper`,
one needs to modify the code like so:

```
diff --git a/src/hands.rs b/src/hands.rs
     Rock,
     Paper,
     Scissors,
+    Clip,
 }
 
 
@@ -54,7 +55,8 @@ pub fn play_hand(own_hand: Hand, other_hand: Hand) -> HandResult {
     match (own_hand, other_hand) {
         (Rock, Scissors)
         | (Scissors, Paper)
-        | (Paper, Rock)             => Win,
+        | (Paper, Rock)
+        | (Clip, Paper)             => Win,
 
         _ if own_hand == other_hand => Draw,
 
@@ -92,5 +94,12 @@ mod test {
         assert_eq!(play_hand(Scissors, Paper), Win);
         assert_eq!(play_hand(Scissors, Rock), Lose);
         assert_eq!(play_hand(Scissors, Scissors), Draw);
+
+        // tests for non-standard hand, Clip
+        assert_eq!(play_hand(Clip, Paper), Win);
+        assert_eq!(play_hand(Paper, Clip), Lose);
+        assert_eq!(play_hand(Clip, Clip), Draw);
+
+        // ... etc ...
     }
 }

```

Then running the game, `Clip` becomes immediately available to play:

```
=== Rock Paper Scissors Clip Game ===

Your move (Rock / Paper / Scissors / Clip / Quit)? >> c
You play : Clip
CPU plays: Clip

Result: you Draw!
Current score: 0

Your move (Rock / Paper / Scissors / Clip / Quit)? >> c
You play : Clip
CPU plays: Paper

Result: you Win!
Current score: 1

Your move (Rock / Paper / Scissors / Clip / Quit)? >> q
Game over.
```
