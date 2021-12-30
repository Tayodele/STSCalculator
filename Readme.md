# Slay the Spire Odds Calculator

## Overview

Calculates the odds of drawing 1 card per turn for an entire run of the deck and for a single turn. Also thre in a armor-damage hea;th loss calculator.

## Requirements

* Cargo

## How to build

This is a pet project and will probably not see production light of day. But if you would like to try it out, run 

```cargo build --release``` 

and run the executable in the `target/release` folder

## Theory

P(draw at least one card) = P(al1) =

![ali](eqs/al1.png)

P(draw at least one card for every draw until the end of the deck) = P(??) = 

![ali](eqs/al1deck.png)

## Contributing

1. My math is most certainly wrong. If you want to contribute please leav a comment in this repo issue with fixed math or a PR if you know rust (or want to update the readme).