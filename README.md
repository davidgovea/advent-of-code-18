# Advent of Code (very incomplete, of course!)

Working through the AoC 2018 challenges in Rust. The contest is already over, which means there are a lot of existing solutions to refer to.

The project structure is heavily inspired by [BurntSushi's solutions](https://github.com/BurntSushi/advent-of-code) -- I like the separate-crate-per-challenge structure, and the method of feeding the input text via `stdin`.

The structure of the `main()` functions was taken from [this reddit tip](https://www.reddit.com/r/rust/comments/8ilg97/small_tip_on_new_main_result_behavior/) -- I'm liking `-> Result` recently -- the `function_returning_result()?` question-mark operator is great.

#### Running
To run a solution, cd into its directory and invoke the program with Cargo:

```
$ cd aoc01
$ cargo run --release < input.txt
```

## 2018 progress

<!--- advent_readme_stars_2018 table --->

## 2019 progress

<!--- advent_readme_stars_2019 table --->

## 2020 progress

<!--- advent_readme_stars_2020 table --->

## 2022 - new approach!
Went with a template this year - the crate-per-challenge approach is nice but kinda boilerplatey. Wanted to jump into latest Rust version, so I am using: https://github.com/fspoettel/advent-of-code-rust

<!--- advent_readme_stars_2022 table --->
