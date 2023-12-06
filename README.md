## Advent of Code 2023: Day 5

Puzzle URL: https://adventofcode.com/2023/day/5

### Building

1. `mkdir -p src/input`
2. Save your puzzle input as `src/input/5.txt`
3. build the project

I have the solutions for each part of the puzzle as separate commits.

### Design

Bear in mind that, while I have experience with a number of different languages,
I just started learning Rust via trying to solve these `Advent of Code` puzzles.

#### Part 1
In this puzzle, we figure out how Rust structs work because we finally need something more complex than basic
pattern matching.

First challenge: parsing the input. This became a lot easier when I realized that I could easily split the input into
sections by splitting on `\n\n`. I defined a `Mapper` struct to parse the lines of each section into.

Oh, neat. Rust structs are really similar to Python classes in how you write struct methods.

Second challenge: resolving the giant chain of mappers.

You read "chain" and you might be tempted to think ".map().map()...etc" but that ends up getting really messy.

What I landed on was creating a struct `SeedLocation` and using a builder pattern to pass in the list of mappers for
each stage.

#### Part 2

Well, fixing up how the seed numbers were parsed wasn't too bad ... but the
puzzle inputs result in like ~1.9 billion keys generated, and testing them all
takes ... a very long time. So if you were to let this run for ~2-3 weeks you'd
probably eventually get an answer.

I'm sure there's a faster way, but I don't know how to find it.
