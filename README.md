# fizzbuzz
## A (very) complex fizzbuzz program using rust

this is a fizzbuzz CLI. That is overengineered Including:

- setting starting amount
- setting max amount
- custom definitions
- progress tracking
- is also a library
- makes use of iterators
- hyperspeed fast
- tiny (~5kb)

## programming stuff about this

- written in rust
- crates it uses:
- - indicatif (https://lib.rs/crates/indicatif) (for progress bar)
- - structopt (https://lib.rs/crates/structopt) (for the CLI)
- works via an iterator(Generator)
- - simply checks for matches in `match_against` and if it matches add the value to the output(This allowed for compound things like `FizzBuzz` to work
- - returns `None` if `current` is equal to `max`
- custom matching uses slices
- - if finds where the `=` sign is and then tries to parse the value before and after the `=`

## FAQ

### why?
why not

### why not use xxx
use it if you want (but idk when someone will need a fizzbuzz generator)
