# Learn-Rust

A snapshot of my notes / learning experiments with Rust.

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
(0..10).rev() reverses the sequence (not sure about exact mechanics of what is going on here yet though)

# Chapter 4

## References and Borrowing

String literals are different than String
let s2 = s // invalidates s ("move")
s.clone() will do the deep copy and not invalidate s
- clone() is explicit call which is good since it's generally more expensive and complex

None of the above matters for simple stack-only types which are all always copied directly anyway
- Copy trait applied to these types
- If Drop trait is applied to type (something special needs to happen when goes out of scope), it's a compile error to apply Copy
- Tuples are not Copy if they contains a value that is not Copy, otherwise they are

Passing non-Copy to function takes ownership
- Doesn't seem to occur with println!() macro, maybe that has something to do with why it's a macro? Not sure

Function return values transfer ownership as well

print_len(&s) // pass reference to String s
fn print_len(string: &String) // takes reference to String (does not own, so does not free when goes out of scope)
This is called "borrowing"
All references are immutable by default

More than one mutable reference to the same thing can't be used at once!
- Seems like you can create more than one mutable reference, but using either one when both exist does NOT work

```rust
let r1 = &mut s1;
let r2 = &mut s1;
println!("{}", r1);
```

- Also can't have a mutable and an immutable
- Can use curly braces to separate scope of references

Rust won't let you create dangling references like returning reference to something destroyed at end of function for example

## Slice

enumerate() returns tuple of index and reference to original item

s.as_bytes(): convert string to byte array for searching
String slices: &s[5..10]
Can drop first or second slice value to mean start or end

String slice type is &str (immutable reference)

can't mutate s because then a mutable and immutable reference would be in use at the same time!

Type of string literal is &str too!

Often better to have function take &str instead of String so literals can easily be used
then pass String's as &s[..]
Also, you can slice &str as much as you want to get &str

Also works for arrays:
let a = [4; 10];
let b = &a[3..5];

# Chapter 5, Structs

Example

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

When var and struct field have same name, can just use "username," to init struct field in struct init

struct update syntax (like ocaml "with")
Type {assignments      ..orig_struct}

Unit-like structs are structs with no fields
Often used when you want to define trait on type but don't want to store any data in type itself

Tuple-like structs
don't have field names, just ordered types
Useful when you want to give tuple type a name and make different than other tuples with same types of elements, but don't want to name elements

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

In order to store references in structs, you need to learn about lifetimes (like &str)

Printing: "{:?}" means "debug formatting"

to auto-implement Debug trait:
"#[derive(Debug)]" above struct definition

## Methods

first param is some variant of "self" like "&self" (don't need explicit type annotation, but must be called self)
method can also take ownership of self and borrow mutably

"main benefits of methods with auto self parameter is organization in 'impl' is organization"

Rust doesn't have -> for references for example; it automatically takes references as it needs with "."
the alternative is something like (&mut a).blah(&b) (ew)

Can have function in imp block without self argument, it's an "associated function" and can be accessed with <type>::func()

Can also use multiple impl blocks per type

# Chapter 6: enums and pattern matching

Rust basically has variants like OCaml ("enums with data attached")

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

Move has an anonymous struct in it
For each of the enum entries, you could separate it out and place "struct" in front of it and it would work as a struct (for Write and ChangeColor they'd be tuple structs, Quit would be unit struct)

You can also define methods on enum types inside impl

Rust has option like ocaml: Option<T> (with Some(T))

eventually read more about Option: https://doc.rust-lang.org/std/option/enum.Option.html

## Match

Use curly braces for more than one line in arm, otherwise omit them
Destructuring with match (at least for tuple struct-like enums) seems to work the same as OCaml
In Option matches in Rust book, None arm seems to be placed above Some arm (convention)
match is exhaustive just like OCaml
_ is also catchall placeholder like OCaml

## If-let

Syntactic sugar for single-case match

```rust
if let Some(3) = some_u8_value {
    println!("three");
}
```

Else works with it too

# Module System

## Packages and Crates

Crate is library or binary
Package contains 1+ crates and has Cargo.toml
At most 1 crate in package is library
So `cargo new my-project` creates a package named "my-project"

If path `src/main.rs` exists in package, it means a binary crate with same name as package exists in package with crate root at `src/main.rs`
If path `src/lib.rs` exists, then package also has a library crate with same name as package and crate root as that
Can have both of those files and package will have both a binary and a library crate
Can create multiple binary crates by placing files in `src/bin`; one binary crate for each file there

## Modules

Crate root creates module with name "crate" which serves as root for modules within
Anything in current or parent module relative to you is accessible, everything in a child module (relative to current) is private by default
use "pub" prefix to make thing publically accessible
Use "super::" to access relative to parent module
    Useful because even if code (and referenced code with super::) are moved inside a new module, it'll still work
Structs can be made public, the fields must be designated public individually though
Enum variants are all public if enum is public

Example

```rust
mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

fn main() {
    let mut v = plant::Vegetable::new("squash");

    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name);

    // The next line won't compile if we uncomment it:
    // println!("The ID is {}", v.id);
}
```

### Use

Use `use` to shorten module paths:

```rust
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // Function body code goes here
        }
    }
}

use crate::sound::instrument;

fn main() {
    instrument::clarinet();
    instrument::clarinet();
    instrument::clarinet();
}
```

Relative paths with `use` currently require using `self`:

```rust
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // Function body code goes here
        }
    }
}

use self::sound::instrument;

fn main() {
    instrument::clarinet();
    instrument::clarinet();
    instrument::clarinet();
}
```

Using absolute path with `use` makes it easier to refactor when moving code to
another module. Rust book authors generally use "crate::" absolute paths with
`use` more often as code using `use` more likely to be moved around module tree
independently from code being called

Idiom: `use` module path for functions (makes it clearer that function isn't locally defined)
But, `use` full path of thing for structs, enums, and other items

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

Not a strong reason for the second one, it's just what ppl have gotten used to writing

The exception is if `use` would import two things with same name; then just import module

```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
}

fn function2() -> io::Result<()> {
}
```

Can also use `as` like in Python
It's idiomatic to use `as` to prevent name conflict

Use `pub use` to re-export the new name that `use` brought into scope (it's private to the outside by default, even if the thing given to `use` is public)

Can factor out paths from same parent path:

```rust
use std::{cmp::Ordering, io};
```

To bring both path and direct subpath into scope, use `self`:

```rust
use std::io::{self, Write};
```

Can also use '*', bad to use in general for the usual reasons
    Often used in testing though to bring stuff that's tested into scope in `tests` module
    Also used for preludes, which are sets of commonly-used things in crate which are picked by estimated frequency of use
        All Rust modules contains `use std::prelude::v1::*`
