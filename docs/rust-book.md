# Rust Book Notes

## 1. Getting Started

### 1.1 Installation

The rust book recommends installing straight from the rust source but I decided to go with rustup as handles rust versioning and tooling chains.

### 1.2 Hello world

- files are denoted with `.rs`
- projects start with a `main.rs` file with a `main()` function
- rust lines end with `;`
- macros are denoted with `!` after the macro name (i.e. `println!`)
  - without `!`, this is just a normal function call
- rust files are first compiled (with `rustc`) then you can run the compiled executable

### 1.3 Hello cargo

Cargo projects are a more structured way to create rust projects. Cargo simplifies builds and library management.

- generate projects with `cargo new [project name]`
  - this will generate the following: `Config.toml`, `src/main.rs` and `.git`
    - `Config.toml`: by default includes [package] for any info about the package and [dependencies] for handling dependencies.
    - `src/main.rs`: the main file for reddit
    - `.git`: instantiates a git repo
- cargo has a few useful commands:
  - `build`: compilies the project into an executable that ends up by default in _/target_
    - `--release`: compiles the project with more optimisations
  - `run`: runs the project rooted at _target/debug/[project name]_
  - `check`: compile project but doesn't produce an executable

## 2. Programming a Guessing Game

This section goes over making a simple guessing game that take user input from console (more info in comments in the file).

- cargo packages can be added with `cargo add [package]`
  - when packages are added, run `cargo build` to get everything up to date
- `Cargo.lock` is a file used to manage dependency versions to ensure that builds are always the same
- cargo can update packages using `cargo update [package]`
  - this command doesn't deal with major versions (i.e. will only go from 0.8.5 -> 0.8.6 instead of 0.9.0)
  - to update major versions, update the version in the `Cargo.toml` file

## 3. Common Programming Concepts

### 3.1 Variables and Mutability

- by convention, variable names should be snake case
- variables are by default immutable in rust (but can be made mutable with the `mut` keyword)
- constants are marked with const and by convention, should use all caps in the name
- shadowing is a concept in rust that allows a variable to be overwritten by another with the same name
  - the let keyword must be used again
  - shadowing is scope specific, so more specific scope shadowing will only overwrite the shadowed value for that scope
