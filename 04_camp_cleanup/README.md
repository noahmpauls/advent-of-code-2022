# Day 4: Camp Cleanup

## Challenge

The core of today's challenge was to find overlaps between pairs of ranges. Part 1 specified finding complete overlap (one range completely contains another), while part 2 required finding any overlap. This was a rare occurrence of part 2 being simpler than part 1.

## Rust

Parsing remains the most difficult part of the challenges so far. I used a regex to extract numbers from the string, but I learned of a smarter solution afterwards that just splits on key characters to get the numbers. With both methods, I have trouble with all the unwrapping and error handling, and my parse functions do not do a good job in the general case.

## Vim

I narrowed down the possible causes of the LSP failure I ran into on a previous day. I again created a `parse` module and added `lazy_static` and `regex`. All LSP typings failed in `parse` after that, and I needed to restart Neovim to get things working. Something in that process must be causing an issue, probably related to the added packages.

