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
don't have field names, just ordered types
can't use one in place of other just because they have same types of elements

Unit-like structs are structs with no fields

In order to store references in structs, you need to learn about lifetimes (like &str)

Printing: "{:?}" means "debug formatting"

to auto-implement Debug trait:
"#[derive(Debug)]" above struct definition

## Methods

first param is some variant of "self" like "&self" (don't need explicit type annotation, but muast be called self)
method can also take ownership of self and borrow mutably

"main benefits of methods with auto self parameter is organization in 'impl' is organization"

Rust doesn't have -> for references for example; it automatically takes references as it needs with "."
the alternative is something like (&mut a).blah(&b) (ew)

Can have function in imp block without self argument, it's an "associated function" and can be accessed with <type>::func()

Can also use multiple impl blocks per type

# Chapter 6: enums and pattern matching
