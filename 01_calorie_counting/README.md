# Day 1: Calorie Counting

## Challenge

Today's challenge boiled down to summing different groups of numbers, then finding the maximum group sum. Part 2 generalized the problem to finding the `n` max groups, where `n = 3`.

## Rust

### `group_by` and `scan`

The input format determined the groups each number belonged to, with a blank line delimiting groups of numbers. I tried to use only iterator functions to achieve this, but found there was no easy way to divide an iterator into arbitrary groups based on a predicate. There is an experimental function called [`group_by`](https://doc.rust-lang.org/std/primitive.slice.html#method.group_by) that aims to add this ability.

Another iterator-only method leverages [`scan`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.scan), but it requires tracking state. At that point, a `for` loop with mutable memo variables was more readable.

### `BinaryHeap` oddities

A [`BinaryHeap`](https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html) makes sense for tracking the maximum of a set of values. Unintuitively, [`iter`](https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html#method.iter) on a `BinaryHeap` returns the elements in arbitrary order; [`into_iter_sorted`](https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html#method.into_iter_sorted) is currently experimental.

### Benchmarking

Using a `BinaryHeap` for part 2 makes sense for large inputs, but for small inputs I'm not sure. It got me thinking about benchmarking, and how you might do that in Rust. [`criterion`](https://docs.rs/criterion/latest/criterion/) appears to be the solution, and the Rust team might adopt it officially in the future.

## Vim

### Neovim LSP code actions

There was a diagnostic I got when I used a function from a module I hadn't `use`d. I figured there'd be a way to automatically add the correct statement, and it turns out that [`vim.lsp.buf.code_action`](https://neovim.io/doc/user/lsp.html#vim.lsp.buf.code_action()) brings up a menu of applicable code actions! I added a mapping for it (`<leader>ca`).

### Easy escape from terminal mode

Escaping from terminal mode in Neovim uses `<C-\><C-n>` by default, which is terrible. I mapped it to `<esc>`:

```lua
vim.api.nvim_set_keymap('t', '<esc>', '<C-\\><C-n>', { noremap = true })
```

### Indenting shortcuts

Some "can't miss" tricks for indentation in Vim:

- Select multiple lines in visual mode, and indent with `>` or `<`. Then, use `.` to repeat as needed!
- Use `=` to perform context-aware auto-indenting. In normal mode it can take a motion (`=i{` indents inside brackets), and in visual mode it operates on a selection.

Here are some interesting keymaps to try, which retain a visual mode selection when indenting:

```vimscript
vnoremap > >gv
vnoremap < <gv
```

