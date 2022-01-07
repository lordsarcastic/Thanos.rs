# Thanos.rs (in development)
Implementation of Thanos and his stones in Rust

## Disclaimer
I am not responsible for what happens if you use this tool. I will respond to threats with more evil tools.
## Getting started (How to run this project)
Requirements involve the Rust toolchain made up of:
- Rustc
- Cargo
- Rustup

To begin:
- Clone this repository
- `cd` into the repository and run `cargo install` to install dependencies
- Run `cargo run` to run the tool
## Currently available features:
Note: Directory path is relative to the directory you're running the tool from.
- Mind: Run with `cargo run mind <path to a directory>`.


## What is this, really?
Oh well, you know the [infinity stones](https://marvelcinematicuniverse.fandom.com/wiki/Infinity_Stones), don't you?
Thanos.rs is a Rust implementation of the infinity stones and [Thanos' snap](https://marvelcinematicuniverse.fandom.com/wiki/Snap) with relation to files and folders on your beloved PC.

While this is still in implementation, guile and terror looms over he who decides to test it out.

## Features:
- Snap: The ultimate command to delete half the files and folders on the path specified. To be used like so: `thanos snap <dir>`
- Mind: Swaps file and directory names for each other
- Space: Randomly reduces or increases content of files.
- Reality: Moves files and folders to random locations in the filesystem
- Soul: No suitable implementation found yet.
- Power: No suitable implementation found yet.
- Time: No suitable implementation found yet.

## Contributions?
Contribs are very welcome. I don't really have a guideline, but feel free to raise an issue and make a PR if it is justified.

## To do (next few days, as i'm quite occupied)
- Implements tests for `mind`
