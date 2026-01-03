# Rust 🦀

A comprehensive guide to Rust programming fundamentals.

## Macros vs Functions

Functions use parentheses like `my_function(args)`, while macros use an exclamation mark like `my_macro!(args)`. This syntactic distinction prevents ambiguity and signals compile-time code generation.

## Data Types

### Primitive Types

**Integer**
- **Signed Integers**: `i8`, `i16`, `i32`, `i64` - defines both negative and positive numbers
- **Unsigned Integers**: `u8`, `u16`, `u32`, `u64` - defines only positive numbers

**Float**
- `f32` - offers 7 decimal digit precision
- `f64` - offers 15-17 decimal digit precision

**Char**
- Defines a single character
- Must be in single quotes
- 4 bytes in size

**Bool**
- `true` or `false`
- Uses 1 byte

### Compound Types

**Arrays**
- Store a fixed size of same type elements
- Support indexing and stack allocation for efficiency

**Tuples**
- Hold a fixed number of values of different data types upon creating

**String**
- Mutable and heap-allocated

**&str Slice**
- References existing string without ownership
- Immutable

## Functions

Functions encapsulate reusable code blocks.

- Function requires explicit parameter types and optional return type followed by `→`
- Last expression serves as return type if not explicitly written
- By default, function parameters are value type which means they take ownership, unless explicitly specified as references using `&type`

## Ownership

- Each value in Rust has a variable that's its owner
- There can be only one owner at a time
- When the owner goes out of scope, the value will be dropped

## References & Borrowing

**Structs**: Custom types with named fields, supports initializing the fields

**Impl**: Attaches methods to structs, `self` refers to the struct instance.

- `&self` is a getter/readonly
- `&mut self` is for modifiers
- This borrow check enforces no simultaneous borrows

**Shadowing**: reuse variable names.
- "In Rust, shadowing allows reusing a variable name while creating a new binding, which is especially useful for stepwise transformations without making the variable mut.”
- “Shadowing is scope-based: an inner shadowed variable hides the outer one temporarily; when the inner scope ends, the outer binding becomes visible again, as your _x example demonstrates.”
- “Use mut when you conceptually have one piece of state that changes; use shadowing when you want to refine a value or even change its type while keeping a clean name."

**Commenting** comments for better documentations
single line comment: "//"
Block comment : "/**/"

**Conditions**: If Else, loops
"Rust if eliminates the ternary operator need—direct value assignment with guaranteed type consistency."

"Expression-based if leverages Rust's block expression semantics—last value without semicolon returns."

"No parentheses, always braces: Rust's deliberate syntax prevents C-style dangling-else pitfalls."

Follow-up Expectation
Interviewer: "Rewrite the divisibility check using match."

rust
match (number % 2, number % 3, number % 4) {
    (0, _, _) => println!("divisible by 2"),
    (_, 0, _) => println!("divisible by 3"),
    (_, _, 0) => println!("divisible by 4"),
    _ => println!("none"),
}
Mastery Signal: Mention if let, match superiority for multiple conditions, expression nature enabling functional style.

**Loops**: for, while, and loop
- Rust offers three loop constructs: loop, while, and for. Each serves distinct purposes with unique ownership/borrowing behaviors
loop vs while: 
loop { if !cond { break; } ... }  // Manual exit
while cond { ... }               // Condition-checked entry
loop better for early exits, retry logic. while for traditional condition loops.
--for loops borrow collections zero-cost via iterators. loop returns values. while moves data—avoid for collections.
**Enum**:
Rust enums are algebraic data types that represent "one of several possible values," each called a variant. Unlike C enums (just integers), Rust enums are tagged unions that can hold data.
Basic way:
enum IpAddrKind {
    V4,    // No data
    V6,
}
let kind = IpAddrKind::V4;  // :: accesses variants

Tuple variant:
enum IpAddr {
    V4(u8, u8, u8, u8),     // Tuple style
    V6(String),
}
let home = IpAddr::V4(127, 0, 0, 1);


**Error Handling **
Rust replaces exceptions with explicit error propagation using Result<T, E> and Option<T>. No nulls, no try-catch. Errors are values you handle explicitly.
? operator eliminates 90% of error handling boilerplate while maintaining explicitness."
Result<T,E> treats errors as values, not exceptions—composable and thread-safe."
"No nulls, no exceptions—just Option for absence, Result for
