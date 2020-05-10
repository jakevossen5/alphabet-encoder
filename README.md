# alphabet-encoder

Currently work in progress, this crate will provide an easy way to encode and decode strings and charachters into "alphabet encoding".

Alphabet encoding is a simple encoding scheme created by a professor of mine - [Kieth Hellman](https://www.mcprogramming.com/index.html) and was used during a compilers course.
The description is simple and very easy to understand:

> Characters *NOT* in the sets `0-9`, `A-Z`, `a-w`, or `y-z` (notice the missing `x`) are *always* written in a hexadecimal escape sequence: `xHH` where `H` is an upper or lower case hexadecimal value.
Other characters may be written in escaped form (`xHH`) or as their visual glyph (for instance `A` for ASCII decimal code 65).

This code is currently only tested with ASCII characters, but non-ascii characters should work as well.

## TODOs

- Be able to decode arbitrary strings
- Test with non-ascii charachters
- Create documentations and examples on README/[crates.io](https://crates.io)
