# Wordler

## Description

This script narrows down the list of valid Wordle words given known data from previous guesses.

## Requirements

Rust 1.58

## Usage:

```shell
cargo r --release -- "<pattern>" <included_chars> <excluded_chars>
```

The program will return a list of all words from the `words_alpha.txt` that:

- are 5 characters long
- contain all characters given in `<included_chars>`
- contain none of the characters given in `<excluded_chars>`
- match the `<pattern>` regex

The regex syntax reference can be found in the docs for the [regex crate](https://crates.io/crates/regex).

## Example

After entering the first guess `boats`, wordle tells us:

```txt
# x: not in final word
# ^: at the right spot
# !: not at the right spot
boats
xx^x!
```

we would run:

```shell
cargo r --release -- "..a.[^s]" s bot
```

which gives us a list of all words that:

- don't contain the characters `b`, `o`, or `t`
- do contain the characters `s`
- has `a` as third character

## Next

There must be a way to avoid using the redundant `<included_chars>` string of included characters, as we already know what characters are part of the final word from the regex pattern.
