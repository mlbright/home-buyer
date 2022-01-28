# home-buyer

> Proof of concept calculator for buying a home in Toronto.

**Do not use this calculator to make any purchase/sale decisions: I take no responsibility for your own financial loss.**

This program is not comprehensive and almost certainly makes significant accounting mistakes or omissions based on incomplete undertanding of real estate financial transactions.

It would be foolish to use a program some random dude wrote in his spare time, when instead one could use any of the widely available online calculators from reputable financial institutions.

This was merely an exercise in [Rust][rust] programming.

## Build

```bash
cargo build --release
cp target/release/home-buyer ~/.bin # or somewhere in your PATH
```

## Run

```bash
home-buyer -a 25 -i 2.50 -p 1200000 --purchase-lawyer-fee 1500 --sale-lawyer-fee 1200 --title-insurance 1000 --sale 1000000 --outstanding-mortgage 255000.00
```

## Help

```bash
home-buyer --help
```

[rust]: https://www.rust-lang.org/
