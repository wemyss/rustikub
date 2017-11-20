# Rustikub - A rummikub solver in Rust

[![Build Status](https://travis-ci.org/wemyss/rustikub.svg?branch=master)](https://travis-ci.org/wemyss/rustikub)

> A basic rummikub solver that uses linear programming to find the best move (and ideally place all tiles in our hand)

---

## Formatting and input

### Colors
```
b = black
l = blue
r = red
y = yellow
j = joker
```

### Data file format (exclude comments)
```
blyr1 r9-11 y4-12 yrj13     # tiles on board
b1 r2 y2 y3 l9 l9 j         # tiles in our hand
```

### Tile input formats
```
j           # joker
y1          # yellow one
bry2        # black two, red two, yellow two
ryj6        # red 6, yellow 6, joker
r1-5        # red 1, 2, 3, 4, 5
ry1-3       # red 1, 2, 3 and yellow 1, 2, 3
```

---

## Goals

- Learn Rust
- Compute optimal solution in < 1s
- Parallelise what I can
- Easy to use
