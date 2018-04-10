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

Any non-move input quits.

Your move ([1] Rock / [2] Paper / [3] Scissors)? >> 2
You play : Paper
CPU plays: Rock

Result: you Win!
Current score: 1

Your move ([1] Rock / [2] Paper / [3] Scissors)? >> 1
You play : Rock
CPU plays: Paper

Result: you Lose!
Current score: 0

Your move ([1] Rock / [2] Paper / [3] Scissors)? >> 3
You play : Scissors
CPU plays: Scissors

Result: you Draw!
Current score: 0

Your move ([1] Rock / [2] Paper / [3] Scissors)? >> q
Game over.
```


## Modifying

The implementation makes it easy to modify the rules of the game.

For example, to add a `Paperclip` hand that beats `Paper`,
but is beaten by `Scissors`, changing the rules of the game like this:

`Rock (beats) Scissors (beats) Paperclip (beats) Paper (beats) Rock`

This modification can be implemented as follows:

```
diff --git a/src/hands.rs b/src/hands.rs
index 998d467..471ab38 100644
--- a/src/hands.rs
+++ b/src/hands.rs
@@ -18,6 +18,7 @@ pub enum HandResult {
 pub enum Hand {
     Rock,
     Paper,
+    Paperclip,
     Scissors,
 }
 
@@ -38,7 +39,8 @@ impl Beats for Hand {
         match *self {
             Rock => Scissors,
             Paper => Rock,
-            Scissors => Paper,
+            Paperclip => Paper,
+            Scissors => Paperclip,
         }
     }
 }
@@ -78,11 +80,19 @@ mod tests {
         assert_eq!(play_hand(Rock, Rock), Draw);
 
         assert_eq!(play_hand(Paper, Rock), Win);
-        assert_eq!(play_hand(Paper, Scissors), Lose);
+        assert_eq!(play_hand(Paper, Scissors), Draw);
         assert_eq!(play_hand(Paper, Paper), Draw);
 
-        assert_eq!(play_hand(Scissors, Paper), Win);
+        assert_eq!(play_hand(Scissors, Paper), Draw);
         assert_eq!(play_hand(Scissors, Rock), Lose);
         assert_eq!(play_hand(Scissors, Scissors), Draw);
+
+        // tests for Paperclip
+        assert_eq!(play_hand(Paperclip, Paper), Win);
+        assert_eq!(play_hand(Paper, Paperclip), Lose);
+        assert_eq!(play_hand(Paperclip, Rock), Draw);
+        assert_eq!(play_hand(Paperclip, Paperclip), Draw);
+
+        // ...
     }
 }
```

Then running the game, `Paperclip` becomes immediately available to play:

```
=== Rock Paper Paperclip Scissors Game ===

Any non-move input quits.

Your move ([1] Rock / [2] Paper / [3] Paperclip / [4] Scissors)? >> 3
You play : Paperclip
CPU plays: Paper

Result: you Win!
Current score: 1

Your move ([1] Rock / [2] Paper / [3] Paperclip / [4] Scissors)? >> 3
You play : Paperclip
CPU plays: Scissors

Result: you Lose!
Current score: 0

Your move ([1] Rock / [2] Paper / [3] Paperclip / [4] Scissors)? >> 3
You play : Paperclip
CPU plays: Scissors

Result: you Lose!
Current score: -1

Your move ([1] Rock / [2] Paper / [3] Paperclip / [4] Scissors)? >> 3
You play : Paperclip
CPU plays: Rock

Result: you Draw!
Current score: -1

Your move ([1] Rock / [2] Paper / [3] Paperclip / [4] Scissors)? >> q
Game over.
```
