# Day 2: Rock Paper Scissors

## Challenge

The core of today's challenge was determining the state of Rock, Paper, Scissors games. In part 1, the outcome had to be determined given each player's chosen shapes; in part 2, one player's shape had to be derived from the other player's shape and the desired outcome.

I was unable to find a representation of the game that made comparisons between shapes intuitively, rather than by hard-coding relationships. You could use a 3-element vector and add special case logic for the first and last element (effecively creating a circular array), but the code would still be inelegant.

## Rust

### Switching from `Result` to `Option`, and back again

Given an `Option`, you can transform it into a `Result` using [`ok_or(err)`](https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or). You can do the reverse using [`ok()`](https://doc.rust-lang.org/std/result/enum.Result.html#method.ok).

## Vim

### Problems with Neovim LSP (resolved after restarting)

The following issue plagued me briefly, but was fixed after completely restarting Neovim. In `parse.rs`, I tried to use a `Regex` from the `regex` crate:

```rust
use regex::Regex;

fn test() {
	let re: Regex = Regex::new(r"[A-Z").unwrap();
}
```

While this code works, I can't get any type info for `re` or anything related to its use. Hovering `re`, for instance, shows its type to be "unknown". I have no idea why this is happening.

