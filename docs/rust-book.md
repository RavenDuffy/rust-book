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

### 3.2 Data Types

- rust is statically typed

### Scalar Types

- these are single value types which include: integers, floats, booleans and characters (there are more but these are the relevant ones for now)
- these types are all saved on the stack by default

**Integer Types**

There are a few built in types that can be both signed (i) and unsigned (u).

| length | signed | unsigned |
| ------ | ------ | -------- |
| 8bit   | i8     | u8       |
| 16bit  | i16    | u16      |
| 32bit  | i32    | u32      |
| 64bit  | i64    | u64      |
| 128bit | i128   | u128     |
| arch   | isize  | usize    |

- arch is is set to either 32 or 64 depending on your system (i.e. 32 on 32bit systems)
- integers can be written with the type specified in the number (i.e. 57u8 would be 57 saved as the type u8).
- integers can also have '\_' which serves as a visual (non parsed) separator (i.e. 1_000 is the same as writing 1000)

| Number literals | Example     |
| --------------- | ----------- |
| Decimal         | 98_222      |
| Hex             | 0xff        |
| Octal           | 0o77        |
| Binary          | 0b1111_0000 |
| Byte (u8 only)  | b'A'        |

- if a rust project has been compiled with the `--release` flag, it will not include checks for integer overflows, instead it will wrap the value (i.e. 255 + 1 = 0, 255 + 2 = 1, etc)

**Floating Point Types**

- the default float type is f64

There are a few float types which are unsigned

| length | type |
| ------ | ---- |
| 32bit  | f32  |
| 64bit  | f64  |

**Numeric Operations**

- rust supports the normal numeric operators: `+`, `-`, `/`, `*`, `%`
- rust doesn't allow operations between different types (i.e. int and float)
  - you must convert them first

**Boolean Type**

- values are either true or false
- these values are represented by a single byte

**Character Type**

- specified with single quotes (i.e. `'z'`)
- chars are 4bytes and therefore can represent a lot more than just ascii (including accented letters, chinese characters and even emojis)

### Compound Types

- these are all saved on the stack by default
- these are types that group multiple values into a single type (i.e. an array)
- rust has two of these types: arrays and tuples

**Tuple Type**

- tuples can store a variety of types
- they cannot change size after declaration
- initialise a tuple using brackets `()` and comma separated values
  ```rust
  let tup: (i32, f64, u8) = (500, 6.4, 1);
      OR
  let tup = (500, 6.4, 1);
  ```
- tuples can be destructured using the following syntax:
  ```rust
  let (x, y, z) = tup;
  ```
- tuple values can also be accessed directly using `tuple.index`
  ```rust
  tuple.0;
  ```
- if a tuple is created without any values (i.e. `tuple = ()`) then it has a special name 'unit'
  - a unit's value and type are `()` and this is the default value of any expression that doesn't return any other value

**Array Type**

- array elements must all have the same type
- arrays in rust are also a fixed length
- arrays are initalised using square brackets
  ```rust
  let a = [1, 2, 3];
  ```
- arrays can also be initalised with their type and size explicitly set
  ```rust
  let a: [u32; 5] = [1, 2, 3, 4, 5];
  ```
- using the syntax below, arrays can be initalised with a number of values of a specific value
  ```rust
  let a = [3; 5]; // this evalutes to let a = [5, 5, 5, 5, 5];
  ```
- to access array elements the normal square bracket syntax is used
  ```rust
  a[0];
  ```
- if a value outside of the available indexes is used (i.e. a[10], if a's length is less than 11, rust will panic)

### 3.3 Functions

- rust functions are declared using the `fn` keyword
- a typical function definition will look like the following:
  ```rust
  fn function() {
      // definition here
  }
  ```
  - to add parameters to functions place them in the brackets next to the function name in the definition (separated by comma)
  ```rust
  fn function(x: u32, y: u32) {
      println!("{x}");
  }
  ```

**Statements and Expressions**

- function bodies are made up of a series of statements (optionally ending in an expression)

  - **statements** are instructions that perform some action and do not return a value
  - **expressions** evaluate to a resultant value

    ```rust
    fn function() {
        let y = 6; // this is a statement

        let x = (let y = 6); // will error because setting y is a statement and thus does not return any value

        x + 1 // this is an expression as it will return a new value
    }
    ```

- calling a function is actually an expression due to the following equivalence

```rust
called_function();
    IS THE SAME AS
{
    // function implementation
    let x = 3;
    x + 1 // note, the lack of a ; here (to make sure rust evaluates this as an expression)
}
```

- if an expression has an ending semicolon, then it becomes a statement and therefore, doesn't return anything
- macros are also expressions

**Functions with Return Values**

- to return values in an function a slightly different definition is used
  ```rust
  fn five() -> u32 {
      5
  }
  ```
- the definition above works for a few reasons:
  1. return value functions always return that last line of the function
  2. their type is the one specified after the `->` in the definition
- the `return` keyword can still be used but it isn't necessary

### 3.5 Control Flow

**`if` Expressions**

- very similar form to most other languages' if statements but brackets are not necessary on if statements

```rust
if number != 0 {
    // success code
} else if {
    // else if code
} else {
    // else code
}
```

- rust does not try to evaluate non boolean values in if expressions, it will just throw an error
- because ifs are expressions they can be used anywhere a return value may be used, for example:

```rust
let number = if true { 5 } else { 0 };
```

- if statements (when used to assign values, like the above), must all return the same type or rust will throw an error

**Repetition with Loops**

- rust has 3 types of loops: `loop`, `while` and `for`
- all loops can be ended using the `break` keyword
  - loops can also be early exited using `return` but return will always end the current function, whereas `break` will only end the current loop
- `loop` is a keyword that tells rust to loop forever - or until you explicitly tell it to stop

  ```rust
  fn main() {
      let mut counter = 0;

      let result = loop {
          counter += 1;

          if counter == 10 {
              break counter * 2; // break can return a value to the caller but it does require a semicolon, unlike most expressions
          }
      }
  }
  ```

- `break` and `continue` will always apply to the innermost loop (that they have access too)

  - to apply these keywords to other loops we can use loop labels

  ```rust
  fn main() {
      let mut count = 0;
      'counting_up: loop {
          println!("count = {count}");
          let mut remaining = 10;

          loop {
              println!("remaining = {remaining}");
              if (remaining == 9) {
                  break;
              }
              if (count == 2) {
                  break 'counting_up;
              }
              remaining -= 1;
          }

          count += 1;
      }

      println!("End count = {count}");
  }
  ```

  - a loop label is designated with a single quote followed by a name i.e. `'loop name: loop`
