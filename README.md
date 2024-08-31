# A minimalistic Parser Combinator Library

A _very_ minimal parser combinator library.

# Example/s

- A [json_parser](https://github.com/angelcerveraroldan/json_parser) made with this library. This took about 130 lines (excluding testing)

# Features 

Supports parsing of UTF-8 characters, not just ASCII.

Right now you can create (let a, b be types):
- Parser a
- Parser (a and b)
- Parser (a or  b)
- Parser (f a)

# Todo's

- [ ] Better Error handing
- [ ] Parser bytes instead of characters

# Notes

## Error Handing

At some point it should use this_error, and miette.
