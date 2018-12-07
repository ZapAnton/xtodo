## Description

A simple command-line utility, that prints any missing or outdated exercises on a given [Exercism](https://exercism.io) track.

Currently two commands are available:
- `missing` - prints the all exercises, which are missing on the current track, but are present on the [Exercism problem-specifications repository](https://github.com/exercism/problem-specifications/tree/master/exercises).
- `outdated` - prints all the exercises, which have the version that differs from the corresponding
canonical-data version.

Since not every Exercism track supports exercise versioning, the `outdated` command is not usable on every track.

Tracks that support the `outdated` command:
- [Rust](https://github.com/exercism/rust)
- [Haskell](https://github.com/exercism/haskell)

## Usage

The utility works with the track repositories located on your local machine.

Use the `-d`/`--track-dir` flag to show the path to the corresponding track.
By default the flag is set to the current directory.

For example, to get the outdated exercises on the Rust track, the following steps could be used:

```shell
git clone https://github.com/exercism/rust.git /local/path/to/rust/track # If you do not have the Rust track repository on your machine

xtodo -d /local/path/to/rust/track outdated
```

or

```shell
git clone https://github.com/exercism/rust.git /local/path/to/rust/track # If you do not have the Rust track repository on your machine

cd /local/path/to/rust/track

xtodo outdated
```

## Installation

Install the latest stable version of the [Rust programming language](https://www.rust-lang.org/) (for instance via [rustup](https://rustup.rs/)).

After that:

```shell
cargo install --git https://github.com/ZapAnton/xtodo.git
```
