# Rust Book Notes

## 1. Getting Started

### 1.1 Installation

The rust book recommends installing straight from the rust source but I decided to go with rustup as
it handles rust versioning and tooling chains.

### 1.2 Hello world

- files are denoted with `.rs`
- projects start with a `main.rs` file with a `main()` function
- rust lines end with `;`
- macros are denoted with `!` after the macro name (i.e. `println!`)
  - without `!`, this is just a normal function call
- rust files are first compiled (with `rustc`) then you can run the compiled executable

### 1.3 Hello cargo

Cargo projects are a more structured way to create rust projects. Cargo simplifies builds and
library management.

- generate projects with `cargo new [project name]`
  - this will generate the following: `Config.toml`, `src/main.rs` and `.git`
    - `Config.toml`: by default includes `[package]` for any info about the package and
      `[dependencies]` for handling dependencies.
    - `src/main.rs`: the main file for reddit
    - `.git`: instantiates a git repo
- cargo has a few useful commands:
  - `build`: compilies the project into an executable that ends up by default in _/target_
    - `--release`: compiles the project with more optimisations
  - `run`: runs the project rooted at _target/debug/[project name]_
  - `check`: compile project but doesn't produce an executable

## 2. Programming a Guessing Game

This section goes over making a simple guessing game that take user input from console (more info in
comments in the file).

- cargo packages can be added with `cargo add [package]`
  - when packages are added, run `cargo build` to get everything up to date
- `Cargo.lock` is a file used to manage dependency versions to ensure that builds are always the
  same
- cargo can update packages using `cargo update [package]`
  - this command doesn't deal with major versions (i.e. will only go from 0.8.5 -> 0.8.6 instead of
    0.9.0)
  - to update major versions, update the version in the `Cargo.toml` file

## 3. Common Programming Concepts

### 3.1 Variables and Mutability

- by convention, variable names should be snake case
- variables are by default immutable in rust (but can be made mutable with the `mut` keyword)
- constants are marked with const and by convention, should use all caps in the name
- shadowing is a concept in rust that allows a variable to be overwritten by another with the same
  name
- the let keyword must be used again
  - shadowing is scope specific, so more specific scope shadowing will only overwrite the shadowed
    value for that scope

### 3.2 Data Types

- rust is statically typed

<u>**Scalar Types**</u>

- these are single value types which include: integers, floats, booleans and characters (there are
  more but these are the relevant ones for now)
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
- integers can be written with the type specified in the number (i.e. 57u8 would be 57 saved as the
  type u8).
- integers can also have '\_' which serves as a visual (non parsed) separator (i.e. 1_000 is the
  same as writing 1000)

| Number literals | Example     |
| --------------- | ----------- |
| Decimal         | 98_222      |
| Hex             | 0xff        |
| Octal           | 0o77        |
| Binary          | 0b1111_0000 |
| Byte (u8 only)  | b'A'        |

- if a rust project has been compiled with the `--release` flag, it will not include checks for
  integer overflows, instead it will wrap the value (i.e. 255 + 1 = 0, 255 + 2 = 1, etc)

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
- chars are 4bytes and therefore can represent a lot more than just ascii (including accented
  letters, chinese characters and even emojis)

<u>**Compound Types**</u>

- these are all saved on the stack by default
- these are types that group multiple values into a single type (i.e. an array)
- rust has two of these types: arrays and tuples

**Tuple Type**

- tuples can store a variety of types
- they cannot change size after declaration
- initialise a tuple using brackets `()` and comma separated values

  ```rust
  let tup: (i32, f64, u8) = (500, 6.4, 1);

  // OR

  let tup = (500, 6.4, 1);
  ```

- tuples can be destructured using the following syntax:
  ```rust
  let (x, y, z) = tup;
  ```
- tuple values can also be accessed directly using `tuple.index` `rust tuple.0; `
- if a tuple is created without any values (i.e. `tuple = ()`) then it has a special name 'unit'
  - a unit's value and type are `()` and this is the default value of any expression that doesn't
    return any other value

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
  let a = [3; 5]; // this evalutes to let a = [3, 3, 3, 3, 3];
  ```
- to access array elements the normal square bracket syntax is used `rust a[0];`
- if a value outside of the available indexes is used (i.e. a[10], if a's length is less than 11,
  rust will panic)

### 3.3 Functions

- rust functions are declared using the `fn` keyword
- a typical function definition will look like the following:
  ```rust
    fn function() { /** definition here */ }
  ```
- to add parameters to functions place them in the brackets next to the function name in the
  definition (separated by comma)
  ```rust
    fn function(x: u32, y: u32) { println!("{x}"); }
  ```

**Statements and Expressions**

- function bodies are made up of a series of statements (optionally ending in an expression)

  - **statements** are instructions that perform some action and do not return a value
  - **expressions** evaluate to a resultant value

  ```rust
  fn function() {
      let y = 6; // this is a statement

      let x = (let y = 6); // will error as setting y is a statement and thus returns nothing

      x + 1 // this is an expression as it will return a new value
  }
  ```

- calling a function is actually an expression due to the following equivalence

  ```rust
  called_function();

  // IS THE SAME AS

  {
      // function implementation
      let x = 3; x + 1 // note, the lack of a ; here (to make sure this as an expression)
  }
  ```

- if an expression has an ending semicolon, then it becomes a statement and therefore, doesn't
  return anything
- macros are also expressions

**Functions with Return Values**

- to return values in an function a slightly different definition is used
  ```rust
  fn five() -> u32 { 5 }
  ```
- the definition above works for a few reasons:
  1. return value functions always return that last line of the function
  2. their type is the one specified after the `->` in the definition
- the `return` keyword can still be used but it isn't necessary

### 3.5 Control Flow

**`if` Expressions**

- very similar form to most other languages' if statements but brackets are not necessary on if
  statements

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

- if statements (when used to assign values, like the above), must all return the same type or rust
  will throw an error

**Repetition with Loops**

- rust has 3 types of loops: `loop`, `while` and `for`
- all loops can be ended using the `break` keyword
  - loops can also be early exited using `return` but return will always end the current function,
    whereas `break` will only end the current loop
- `loop` is a keyword that tells rust to loop forever - or until you explicitly tell it to stop

  ```rust
  fn main() {
    let mut counter = 0;
      let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // break can return a value to the caller but it does require a
                               // semicolon, unlike most expressions
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

    println!("End count = {count}"); }
  ```

- a loop label is designated with a single quote followed by a name i.e. `'loop name: loop`

- `while` loops work essentially the same way as while loops in most other languages and should be
  use to loop until a condition is met

  ```rust
  fn main() {
    let mut number = 3;

    while number != 0 {
      println!("{number}");

      number -= 1;
    }

    println!("done");
  }
  ```

- `for` loops have a few different approaches:

  ```rust
  let a = [1, 2, 3, 4, 5];

  // this loops through every value in the 'a' array
  for n in a {
    println!("{n}")
  }
  ```

  ```rust
  // this loops through the range (1..4) provided, in reverse (because of the .rev() call)
  // ranges are by default non-inclusive (on the end value) but can be made inclusive with an '='
  // 1..=4 to include 4
  for number in (1..4).rev() {
    println!("{number}")
  }
  ```

## 4. Ownership

Rust has a concept called ownership that allow rust to make memory safe guarentees without the need
for a garbage collector.

### 4.1 What is Ownership?

Ownership is a set of rules rust follows in order to manage memory. Ownership only affects the code
before it runs (i.e. all issues will be caught during compilation).

<u>**The Stack and the Heap**</u>

Both the stack and the heap are available during runtime but they're structured differently. The
stack stores values in the order it receives them and removes the values in the opposite order (i.e.
last in, first out). Data added to the stack is called a _push_; data removed is called a _pop_. All
data stored on the **stack** must have a known, fixed size. Any data with unknown size at compile
time or any data that may change must be stored on the **heap** instead.

When data is stored on the heap a few steps happen: First, a certain amount of space is requested;
then the memory allocator finds an empty spot in the heap (large enough for the data); that spot is
marked as in use and returns a pointer to that address. That process is called _allocating on the
heap_ (often abbreviated as _allocating_). \*Pushing values onto the stack is not considered
allocating. Because the pointer to the heap is known and is a fixed size, it can be stored on the
heap. However, if you want the actual data you have to follow the pointer.

Pushing to the stack is faster than allocating on the heap because the allocator doesn't have to
search for a place to store new data; the location is always the top of the stack. Allocating (on
the heap) requires more work because the allocator must find a space big enough for the new data and
then perform book-keeping to be prepared for the next allocation. Accessing data on the heap is
slower than the accessing it on the stack because you have to follow the pointer to get it.

When rust calls a function, the values passed into that function (which can include pointers to data
in the heap) and that functions local variables get pushed to the stack. When the called function
ends, the values that were pushed to the stack are all popped.

<u>**Ownership Rules**</u>

Rust follows three ownership rules:

1. Each value has an _owner_
2. There can only be one owner at a time
3. When an owner goes out of scope, the value is dropped

**Variable Scope**

- variables are only valid from when they're first initalised, until it goes out of scope
  - scopes are usually defined by curly brackets (anything inside them is within their scope)
  - top level scopes would be defined before anything else and would be available everywhere

**Strings**

In rust there are two main string types: `&str` and `String`. `&str` is a slice (`&[u8]`) which
points to a valid **UTF-8** sequence, these can be used to view `Strings`. The other type is a
`String`, which is stored as a vector of bytes (`Vec<u8>`). `String`s are guaranteed to be a
**UTF-8** sequence, they're heap allocated, growable and not null terminated. There are also string
literals which are defined as `"string"` (which is a `&'static str` type).

`String`s are defined as:

```rust
let s = String::from("hello");

// we can also mutate these strings!
s.push_str(" world");
```

**Memory and Allocation**

In the previous case (string literal), we know the contents at compile time so text is hardcoded
directly into the final executable. This is what lends string literals their speed and efficiency.
However, these properties only come from the string's immutability. When the `String` type is used
instead of a literal, we need to be able to support mutability and growability. In order to do so,
`String` needs to allocate some amount of memory to the heap - an amount that's unknown at compile
time. This means:

- the memory must be requested from the allocator at runtime
- when the `String` is no longer needed, there must be a way to return this memory to the allocator

The first part is done when `String::from` is called; its implementation requests the required
memory. The second part is where rust differs from other programming languages. In rust, memory is
automatically returned as soon as the variable that owns it goes out of scope/is not longer used.
For example:

```rust
{
  let s = String::from("Hello");
  // s is valid until the bottom curly bracket, where it goes out of scope
}
```

When variables go out of scope, rust calls the function `drop` which the author of (in this case)
`String` can implement to handle what happens when an instance goes out of scope. `drop` can be
called manually but it's automatically called when something goes out of scope.

**Variables and Data Interacting with Move**

```rust
let x = 5; let y = x;
```

In the example above, we set x to 5 which is known at compile time and pushed to the stack. We also
set y to equal a new copy of x (which is not a pointer, it is a new copy) which also gets pushed to
the stack for the same reasons as x.

```rust
let s1 = String::from("hello"); let s2 = s1;
```

In this example, when we set s1 to the `String` "hello" we would assume s2 can copy the value of s1
like in the previous integer example. The `String` type does not work the same way as the integer
primative though. To understand whats going on let break down how a `String` works. A `String` is
made up of three parts: A pointer to the memory that holds the `String` contents, the length of the
string and the capacity. The length represents how much memory (in bytes) the string is using, the
capacity is the total memory (also in bytes) the `String` recieved from the allocator. These
components are all stored on the stack while the memory the pointer points to is stored in the heap.

When we to assign `s1` to `s2`, the `String` data is copied, meaning we copy the length, the
capacity and the pointer (all of which are on the stack) - we do not copy the data on the heap that
the pointer refers to. To ensure memory safety, after the line `let s2 = s1;`, Rust no longer
considers `s1` as valid/having a value which means Rust doesn't need to free anything up after `s1`
goes out of scope. Rust will prevent us from trying to do this with the error: `borrow of moved
value: ...`. In a lot of other languages, the concept of copying the values + pointer to memory is
known as a shallow copy but Rust goes a step further and actually does a 'move' instead (known as
such due to it making the variable that was moved from invalid). By default, Rust will never make
deep copies of data (i.e. clones) for performance purposes but, if needed, deep copying can be done.

**Scope and Assignment**

When a new value is assigned to an existing variable, Rust will call `drop` to free the original
value's memory immediately.

```rust
let mut s = String::from("original");
s = String::from("new");

println!("{s}");
```

In the above example, we define a variable (s) and bind a `String` to it with the value "original".
Then we immediately create a new `String` with the value "new" and assign it to s (the same variable
as before). After the second assignment, there is nothing left that refers to the original value in
the heap (i.e. "original"). By consequence, the original string goes out of scope and Rust calls
`drop` to free its memory.

**Variables and Data Interacting with Clone**

If we do want to do a deep copy (i.e. inlcude the heap data), we can use the common method `clone`.
For example:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("{s1} {s2}");
```

This now works as expected and allows us to create a copy of s2 (without any ownership issues).
These strings also have separate pointers in memory so their values remain separate.

**Stack-Only Data: Copy**

To readdress why, in the section that showed the integer copying example, we are able to clone
integers without `.clone()`. The reason we're able to do this is because integers have a known size
at compile time and are therefore stored on a completely different stack (this also applies to other
types who's sizes are known at compile time). The benefit of being stored on this other stack is the
speed that comes with it which allows us to make copies of values very quickly. This boils down to
integers, and types like them, not having any difference between shallow and deep cloning.

Rust has a special annotation called the `Copy` trait which can be placed on types stored on the
stack (i.e. integers). A variable of a type that implements the `Copy` trait don't _move_, instead
they're copiedwhich leads to them still being valid after assignment to another variable.

Any type who (or who's parts) have implemented the `Drop` trait are not allowed to also use the
`Copy` trait. Therefore, if a type needs to do something special when its value goes out of scope,
that type must not have implemented the `Copy` trait because `Drop` will throw a compile time error.

In general, the types we can expect to have implemented the `Copy` trait are types comprised of any
group of simple scalar values. Anything that requires allocation or is some form of resource, will
not be able to implement `Copy`. Some examples of types that implement `Copy` are:

- all integer types (i.e. `u32`)
- the boolean type (`bool`)
- all floating point types (i.e. `f64`)
- the character type (`char`)
- tuples (only if they contain types that also implement `Copy`) i.e. `(i32, i32)`

**Ownership and Functions**

Passing a variable to a function works just the same as if you'd reassigned that value to another
variable. This means that passing a variable performs either a move or a copy depending on what the
original value is.

```rust
fn main() {
  let s = String::from("hello");
  takes_ownership(s); // once s is passed into this function, we can no longer us s in this scope

  let x = 5;
  makes_copy(x); // i32 implements the copy trait so we can continue to use x in after this
}

fn takes_ownership(some_string: String) { // some_string is in scope
  println!("{some_string}");
} // some_string goes out of scope and `drop` is called

fn makes_copy(some_integer: i32) { // some_integer is in scope
  println!("{some_integer}");
} // some_integer goes out of scope, nothing else occurs
```

**Return Values and Scope**

```rust
fn main() {
  let s1 = gives_ownership(); // gives_ownership's return value is moved to this variable

  let s2 = String::from("hello"); // s2 comes into scope

  let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back which moves its
                                     // return value back to s3 (s2 will no longer have a value)
} // s3 goes out of scope and runs `drop`, s2 was moved so nothing happens, s1 goes out of scope
  // and runs `drop`

fn gives_ownership() -> String {
  let some_string = String::from("new"); // some_string comes into scope some_string
                                         // some_string is returned, moving it out of the function
                                         // scope
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope a_string
                                                      // a_string is return, moving it out of the
                                                      // function scope
}
```

Variable ownership follows this same pattern everytime: assigning a value to another variable moves
it; when a variable that includes data goes out of scope, that data is cleaned with `drop` (unless
ownership has been transferred).

- we can return multiple values from a function using a tuple i.e. `(s, length)`

### 4.2 References and Borrowing

The issue with function params as we've been using them so far is that it's can be limiting to
always have return each parameter. To get around this, rust offers an alternative: a _reference_. A
reference is like a pointer in that, it's an address that we can follow to access the underlying
data. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for
that reference's lifetime.

The following function will be presented both without and with using a reference parameter:

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);
}

// this must return the string even though it does not modify it
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
```

```rust
fn main() {
    let s1 = String::from("hello");

    // `&s1` creates a reference to `s1` without owning it
    let len = calculate_length(&s1);
}

// this only needs to return the new value (the length in this case)
fn calculate_length(s: &String) -> usize {
    s.len()
}
```

- we define a reference with the `&` operator and we can dereference them with `*`

By default, references are immutable so if we try to modify something we've borrowed Rust will throw
an error. For example:

```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(" world"); // this will error with: `cannot borrow *some_string* as
                                    // mutable, as it is behind a '&' reference`
}
```

**Mutable References**

We can fix the previous code using a _mutable_ reference.

```rust
fn main() {
    let mut s = String::from("hello"); // make s a mut (mutable)

    change(&mut s); // pass s as a &mut (mutable reference)
}

fn change(some_string: &mut String) { // the type becomes &mut String
    some_string.push_str(" world");
}
```

When a mutable reference is created, there must be **no other references** to that value. For
example:

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;
```

The above code will error with: `cannot borrow 's' as mutable more than once at a time`. This
happens because the second reference (`r2`) is created before the first reference (`r1`) goes out of
scope.

- we cannot create a mutable reference if a non-mutable reference exists at the same time

References' scopes start where they're defined end whenever they either go out of scope or are last
used. For example, the code below compiles due to the last use rule:

```rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;

println!("{r1} {r2}"); // r1 and r2 are not used after this point so we can create new references

let r3 = &mut s; // this is valid because r1 and r2 are not used again
println!("{r3}");
```

**Dangling References**

Rust guarentees that references will never dangle, if you have a reference to some data, the
compiler will ensure that the data will not go out of scope before the refence does.

### 4.3 The Slice Type

_Slices_ are references to a contiguous (neighbouring) sequence of elements in a collection (rather
than a whole collection). A slice is a kind of reference and therefore does not have ownership.

If we wanted to create a function that returns a part of a string, we need to create a function like
the following:

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // converts string to array of bytes

    for (i, &item) in bytes.iter().enumerate() { // create an iterator over the bytes array (index,
                                                 // value ref) (returns as a tuple)
        if item  == b' ' { // comparision to the byte value for a space character
            return i;
        }
    }

    s.len()
}
```

The issue with the above function is that returning `s.len()` has no meaning without being attached
to the `String`. For example, if we call `s.clear()` and wipe the value, the returned `s.len()` will
still equal whatever that string's length was. To solve this problem, Rust has _string slices_.

A string slice is a reference to a part of a `String` which is defined as following:

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

To further breakdown the definition:

1. `&s` to get the reference
2. `[a..b]` to set the range (a inclusive and b non-inclusive, b can be made inclusive by adding
   an = sign) - the range can also be set to include the first index (0) with `[..b]`, the last
   index with `[a..]` or the every index with `[..]`

With this new information, we can rewrite the `first_word` function as following:

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // converts string to array of bytes

    for (i, &item) in bytes.iter().enumerate() { // create an iterator over the bytes array (index,
                                                 // value ref) (returns as a tuple)
        if item  == b' ' { // comparision to the byte value for a space character
            return &s[..i];
        }
    }

    &s[..]
}
```

**String Literals as Slices**

String literals (`let s = "hello world;"`), are slices that point to specific points in binary. This
is why string literals are immutable (because `&str` is an immutable reference).

**String Slices as Parameters**

We can further improve the `first_word` function from before by replacing the type of the parameter
`s` with a string slice (`&str`) instead of an `&String`. The advantage of doing this is that we are
now able to accept both `&str` and `&String` types (by of course converting `&String` beforehand).

**Other Slices**

Aside from string slices, there is a more generic type of slice. For example:

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```

The value of `slice` in this case will be a slice (containing 2, 3, 4) of type `&[i32]`.

## 5. Using Structs to Structure Related Data

A _struct_ or a _structure_ is a custom data type that allows you to group (and label/name) multiple
values to make up a meaningful group.

### 5.1 Defining and Instantiating Structs

A typical struct definition may look something like the below:

```rust
struct User { // `struct` starts a struct definition (this time named User)
    active: bool, // each `struct` field contains a name and a type
    username: String,
    email: String,
    sign_in_count: u64,
}
```

To instantiate the `User` struct we just defined we'd do this:

```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("username"),
        email: String::from("fake@email.not"),
        sign_in_count: 1,
    };
}
```

We can access specified values from structs using dot notation i.e. `user.active`. If the instance
is mutable we can access specified values and edit them i.e. `user.active = false`. However, the
entire struct must be mutable in order for us to do this, Rust does not allow setting only certain
fields as mutable.

**Using the Field Init Shorthand**

In the example below, we're able to skip write both `username` and `email` with just their names.
This is because the parameters have the same name as the field names so we don't need to write them
twice.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // this has the same name as the parameter so we don't need write it twice
        email, // same as above
        sign_in_count: 1
    }
}
```

**Creating Instances from Other Instances with Struct Update Syntax**

If we ever need to create a new struct from an existing one (usually to change a few fields) we can
take advantage of the `..` syntax. For example:

```rust
let user2 = User { // this assignment will move the data - rendering user1 unusable
    email: String::from("new@example.com"),
    ..user1 // this will add all other (non-specified) fields from user1 to user2
};
```

In the example above we cannot use user1 after user2 is created because the data inside user1 is
moved to user2. However, if we create user2 from user1 and give user2 new values for both email and
username we will get two usable instances of User (because active and sign_in_count both implement
the copy trait).

**Using Tuple Structs Without Named Fields**

Rust also includes the _tuple structs_. These are similar to base structs but their fields aren't
named. These are useful to give slightly more complex than base types names so they're more easily
understandable. For example, we could create a colour type using a tuple struct.

```rust
struct Color(u8, u8, u8);

fn main() {
    let black = Color(0, 0, 0);
}
```

Tuple struct types are distinct based on name, meaning that even if the types are exactly the same,
if their names are different, they aren't interchangeable.

```rust
struct Color(u8, u8, u8);

fn main() {
    let black = Color(0, 0, 0);
    let Color(x, y, z) = black. // this destructures the object into named values (x, y and z)
                                // to access a specific value we would use `black.0`, `black.1`, etc
}
```

**Unit-like Structs Without Fields**

We can also define struct that don't have any fields at all. These can be useful when implementing a
trait on some type if we don't have any data we want to store along with it.

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

## 5.2 An Example Program Using Structs

Refer to [structs-example-5_2/src/main.rs](/structs-example-5_2/src/main.rs)

## 5.3 Method Syntax

Methods are similar to functions but they're defined within structs, traits or enums. Methods are
declared with `fn`, they can have parameters, they contain code that runs when the method is
called and their first parameter is always `self`. In order to associate methods with their struct,
trait or enum we have to use the `impl` (implementation) keyword.

The code below defines a Rectangle struct and (via an `impl` also associates an area method that can
be used).

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

**Assocated Functions**

If we wanted to define a constructor for the Rectangle struct (i.e. a square) we can add something
like the following:

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Self is an alias for the type after `impl` so Rectangle could also be written
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```

Compared to the methods we've been defining so far, there are a few things different. This is
because, what we've defined here is called an _associated function_ not a method. Associated
functions are functions defined within `impl` blocks but they don't have `self` as their first
parameter (usually because they don't need an instance of self to do their work).

- _the `new` keyword is often used when calling constructor like functions but `new` actually isn't
  a built in reserved keyword_

To call an associated function, we use `::` (prefixed by the `struct` name and affixed by the
function we're calling). For example, `let square = Rectangle::square(5);` would create a new
Rectangle using the square "constructor".

Each `struct` is allowed to have multiple `impl` blocks which is useful when implementing generic
types and traits.

## 6. Enums and Pattern Matching

### 6.1 Defining an Enum

Enums are used to define sets of possible values. If, for example, we wanted to define an enum for
different ip address types (v4 and v6) we could do the following:

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

Now that we've created this enum we can create instances using the same `::` syntax as when we call
associated functions:

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

We can, of course, set types to expect the `IpAddrKind` enum which can be set to either value. Like
the following:

```rust
fn route(ip_kind: IpAddrKind) {}

// callable with
route(IpAddrKind::V4);
//or
route(IpAddrKind::V6);
```

If we wanted to store data to go along with our `enum` we can take advantage of Rust's ability to
store data inside of enums. For example, if we wanted to create an IpAddr (ip address) enum that
stores both kind and an address we can:

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

We can actually take this even further and define different data definitions per enum type:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

- _`IpAddr` actually has a standard library definition (nearly identical to our definition) but we
  can create and use our own without issue (unless we bring that definition into our scope)_

- Enums can call `impl` to define methods (with the same naming conventions as with `struct`).

Along with associated data, enums can also define named fields (like a struct). The enum in the
example below defines types with a variety of variants:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColour(i32, i32, i32),
}
```

**The `Option` Enum and its Advantages Over Null Values**

Rust does not implement a `Null` type and instead uses the `Option` enum. `Option` is an enum that
accepts any input type and can be used if the return value is either there or not. This enum is
defined as:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

This enum is included in the prelude and therefore doesn't have to be explicitly brought into scope;
its variants (`None` and `Some`) are also included.

- _In rust `T` represents a generic type_

### The `match` Control Flow Construct

`match` allows any value to be compared against a series of patterns then, based on the result (i.e.
which pattern matches), execute code. Patterns (to be compared) can be literals, variable names,
wildcards, etc. `match` works similarly to a series of if else statements i.e. the value is compared
sequentially until it finds something that matches.

For an example using match to find coin values refer to [coin_match_6_2](coin_match_6_2/src/main.rs)

The main difference between `match` and a regular if statement is that `match` doesn't have to
evaluate to a boolean value. As well as that, `match` must be provided with **exactly** one matched
branch (and this must be provable to the compiler). `match` is also an expression (i.e. can be
passed as a value).

Branches of `match` expressions can either be done using shorthand (i.e. without curly braces) or
they can be written long-form. Long and shorthand expressions can be mixed in the same `match`
statement.

```rust
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin:Quarter => 25,
    }
}
```

**Patterns That Bind to Values**

For an example using match with values bound refer to [coin_match_6_2](coin_match_6_2/src/main.rs)

**Matching with `Option<T>`**

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

**Catch-all Patterns and the `_` Placeholder**

In the case where we need to catch any values that either don't match or we don't expect, we can use
a catch all.

```rust
let dice_role = 9;
match dice_role {
    3 => do_something(),
    7 => do_something_else(),
    other => catch_everything(other), // this line will catch all values that aren't 3 and 7

    // we can use _ to catch if we don't need the value caught
    _ => catch_everything(),
}
```

### 6.3 Concise Control Flow with `if let` and `let else`

The `if let` syntax allows `if` and `let` to be combined into one expression that sets a value if
its match condition is met. Fro example, we can convert the following code:

```rust
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The max is now {max}"),
    _ => (),
}

// this can be simplified to

let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The max is now {max}");
}
```

In the simplification above, the `if let` line allows us to do something (in this case, print
something) if config_max's value contains a `Some` of type `u8`. The `if let` syntax is:
`pattern = expression` and means, if the `pattern` is matched evaluate the `expression`.

`if let` can also be combined with with an `else` block:

```rust
// without if let
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {state:?}"),
    _ => count += 1,
}

// with if let
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {state:?}");
} else {
    count += 1;
}
```

The other common scenario is one where we want to do something with a few values and present a
default for all other cases. The `let else` pattern allows us to handle this default case nicely.

```rust
fn describe_state_quarter(coin: Coin) -> Option<String> {
    // this allows us to return early if there is no state (or coin)
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1990) {
        Some(format!("{state:?} is pretty old for america"))
    } else {
        Some(format!("{state:?} is relatively new"))
    }
}
```

## 7. Managing Growing Projects with Packages, Crates and Modules

Rust has a number of ways to manage code organisation:

- **Workspaces**: A way to link packages/crates together (these are covered in section 14)
- **Packages**: A Cargo feature which allows you to build, test and share crates
- **Crates**: A tree of modules that produces a library or executable
- **Modules** and **use**: Way to control organisation, scope and privacy of paths
- **Paths**: Way of naming an item (i.e. a `struct`, `function` or `module`)

### 7.1 Packages and Crates

A _crate_ is the smallest amount of code the compiler considers at a time. There are two types of
crates: binary or library. Binary crates are programs that can be compiled into an executable (i.e.
a cli, server, etc). All binary crates must have a `main` function - which is used to define what
the compiled executable will do when it's run.

Library crates define functionallity aimed to be used by multiple projects. These crates don't have
a `main` function and because of that, they don't produce an executable when compiled.

Crates all have a source file (a.k.a, a _crate root_) where the compiler starts traversal.

A _package_ is a bundle of one or more crates. All packages contain a _Cargo.toml_ file which
describes how to build the bundled crates. Packages can house as many binary crates as necessary but
only one library crate (at the most).

When a new package is created there are two file names that can be used: `src/main.rs` (which is the
crate root of a binary crate) and `src/lib.rs` (which is the crate root of a library crate).
Multiple binary crates can be created by placing files in the `src/bin` directory (where each file
will be a separate crate).

### 7.2 Defining Modules to Control Scope and Privacy

**Modules Cheatsheet**

1. Start from the crate root: when compiling a crate the compiler looks for the crate root first
   (`src/lib.rs` for a library crate or `src/main.rs` for a binary crate).
2. Declaring modules: in the crate root file new modules can be declared using the `mod` keyword
   (i.e. `mod new_module;`) - the compiler will look for module code in the following places:
   - inline, within curly brackets that replace the semicolon (i.e. `mod new_module { /** code here
*/ }`)
   - in the file _src/new_module.rs_
   - in the file _src/new_module/mod.rs_
3. Declaring submodules: in any other file, aside from the crate root, you can declare submodules
   (i.e. `mod newer_module` within the previous `new_module` module). The compiler looks for these
   submodule implementations in the same places as normal modules (just one step down i.e.
   _src/new_module/newer_module.rs_, etc)
4. Paths to code in modules: modules within crates can be accessed from anywhere else within the
   same crate (as long as their public). If, for example, we wanted to access `new_module`, we can
   do `crate::new_module::newer_module`.
5. Private vs public: by default, module code is private from its parent modules. To make a module
   public we do `pub mod` instead of just `mod` when declaring the module. Items within a public
   module are also private by default (by prefixing `pub`).
6. The `use` keyword: `use` allows us to bring other modules into scope (i.e. instead of calling the
   entire `newer_module` path we can just `use crate::new_module::newer_module` and now we can use
   `newer_module` in that scope).

**Grouping Related Code in Modules**

_Modules_ are used to organise code within a crate for readability, reuse and privacy (which is by
default private).

In the [restaurant](restaurant/src/main.rs) directory, we have an example of how modules can help
with organisation.

### 7.3 Paths for Referrring to an Item in the Module Tree

In order to allow Rust to find items in module trees, we use paths which can either be an _absolute
path_ or a _relative path_. An _absolute path_ starts at the crate root - literally `crate` - (or
the crate name if pulling from an external crate). A relative path starts from the current module
and uses `self`, `super` or another identifier in the current module.

Below is an example of calling the `add_to_waitlist` function using both an absolute path and a
relative one.

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();
}
```

In the previous snippet there is actually one issue as it currently stands: if we try to access the
hosting submodule, we'll encounter a privacy error. By default, all children of all other items
(i.e. functions, methods, structs, etc) are private.

To resolve our aforementioned privacy issue we'll simply make both the `hosting` module and the
`add_to_waitlist` function public. We'll do so using the `pub` keyword like the following:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();
}
```

**Starting Relative Paths with `super`**

Relative paths can begin in the parent module using the `super` keyword at the start of the path. In
the example below, we use `super` to get the higher level `deliver_order` function:

```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order () {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

**Making `structs` and `enums` public**

Refer to [modules-7_3/src/main.rs](modules-7_3/src/main.rs)

### 7.4 Bringing Paths into Scope with the `use` Keyword
