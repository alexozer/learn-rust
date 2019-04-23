# Chapter 1

`rustup update`: Update Rust
`rustup --version`
`rustup doc`: Open offline Rust docs in web browser

"println!()" is a macro

## If "no defualt toolchain configured"

```
rustup install stable
rustup default stable
```

## cargo

cargo new <project>
cargo build -> builds in /target/, creates Cargo.lock
cargo run -> builds and runs main
cargo check: check that it would compile, but don't produce executable (faster)
cargo build --release
cargo doc --open

# Chapter 2

"A::b()" b() is "associated function" of A

I think:
`guess` means "value"
`&guess` means "immutable reference"
`&mut guess` means "mutable reference"
References are immutable by default, even if original var was declared mut

Sometimes idiomatic to break single line with multiple .method() calls into multiple lines

{} is placeholder in println!() string for next argument values

std::cmp::Ordering: another enum (Greater, Less, Equal)

shadowing variables with different types is idiomatic when converting between types in Rust

u32: unsigned 32-bit number, not bad default for small, positive number

Matching Result example:
```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

## Result types

generic Result, io::Result, etc
io::Result::expect() will raise error if result is Err, else returns value of Ok (# bytes read)
So type of expect() is int (or similar)

## Cargo 

Dependency versions:
rand = "0.3.14"
"0.3.14" is short for "^0.3.14" which means "any version with public API compatible with "0.3.14"

Cargo.lock: when it exists for a dependency, it uses that version of a dependency
cargo update: update dependencies with new versions and update Cargo.lock
Uses semantic versioning to decide how to update: "0.3.14" will only update to a higher version of "0.3.*"
To get something like "0.4.*" you need to update Cargo.toml (and then cargo update)

# Chapter 3

## Variables and mutability

Shadowing variable declarations is not completely taboo, even when keeping the type
    can be used to do transformations without needing to make variable mut so it stays not mut afterwards

## Constants

must have type annotation
must be assigned to constant expression
can be declared in global scope too

## Data types

Type inference can happen a lot, sometimes a lot of types are possible and annotation required

Has integer types i8 - i128, u8 - u128, isize and usize

23_523 integer
0xff hex
0o77 octal
0b1100_0110 binary
b'A' byte (for u8)

Debug-compiled Rust panics on overflow!
You shouldn't rely on implicit wrapping, using Wrapping type

f32 and f64: float types
f64 is default

Basic math operations + - * / % work as expected (% works for float too!)
bool stuff: values: true and false, type: bool

char is 16-bit Unicode Scalar Value

Multiple assign doesn't really work except for let with pattern matching

## Tuples

```rust
let tup = ("uh oh", 3, false);
let (x, y, z) = tup;
println!("x: {}, y: {}, z: {}", x, y, z);
```

Can also index tuple elements with tup.0, tup.1, etc

## Arrays

Fixed-length (vectors are not)
normal square bracket notation for literal

types: [<elem type>; <elem count>]
[i32; 5]

[3; 3; 3; 3; 3] is same as [3; 5]
array indexing normal syntax like a[2]

## Functions

snake_case is used instead of camelCase (for variables too)
- function definition order doesn't matter

Must declare types in function signature

Rust has both statements and expressions (normal let is a statement, fn is a statement, etc)
blocks are expressions!
last "statement" without semicolon is expression for whole block
does not having expression at end of block mean block is unit? nope it's "empty tuple"

Most functions return this way (without "return") at end

## Comments

Only has // and "documentation comments"
"more often" you see // above line than next to it

## Conditionals

if has no parens
Must be bool, no truthy stuff
if is an expression!!

```rust
let number = if true {
    2
} else {
    9
};
```

## Loops

Unbounded loop: loop
loops are expressions too!!
break <result expression>
while loops like you'd expect

for <name> in <iter something> { }
Range object used for number ranges with same for structure

.rev() reverses sequences or something?
0..10 sequence / range (endpoints can be arbitrary expressions)

# Chapter 4
