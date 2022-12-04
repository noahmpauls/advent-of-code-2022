# Day 3: Rucksack Reorganization

## Challenge

The core of today's challenge was finding shared letters between sets of strings. Part 1 required a solution for two strings, while part 2 generalized for `n` strings, where `n = 3`.

## Rust

I'm probably dusting off Rust cobwebs, but today was quite challenging for some reason.

### Returning `Iterator`s

`Iterator` is a trait, not an actual type. This makes sense: anything should be able to implement the `Iterator` "interface". But I get confused about that all the time, and as a result my brain gets twisted in knots when trying to return iterators without using the heap (with a `Box`).

Here's what I've reasoned: an iterator, being just a trait, represents a borrow of what's being iterated over. We can thus deduce that for a function like the one below:

```rust
// note: not necessarily a valid signature
pub fn rucksack_repeats(rucksack: &str) -> impl Iterator<Item = char> { ... }
```

Since the returned iterator must be a borrow of something, and we can't return a borrow of anything created within the function body to avoid using the heap, the returned iterator must borrow the input (`rucksack`), and take on that input's lifetime.

I stupidly avoided using `Box` values for my solution, so I ended up converting everything to owned values. I need to stop being shy about using the heap.

### Iterating in chunks

If you want to iterate over every set of `n` elements in an iterator, the current easiest way is to use the [`chunks`](https://doc.rust-lang.org/std/primitive.slice.html#method.chunks) method on a slice. Iterator chunks are [experimental](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.array_chunks).

## Vim

I didn't learn anything particularly new; just one or two annoyances:

- I'm still very slow at navigating. I get all the keys mixed up, especially the basic `hjkl` and `web`.
- My terminal mode `esc` remap broke the `fzf` plugin's floating windows. I can no longer `esc` out of them.
- If I want to remove a buffer from the list of open buffers, I use `:bd`. But this closes the split I'm working in!

