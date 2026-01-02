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
