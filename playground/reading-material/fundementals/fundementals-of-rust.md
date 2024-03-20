# Rust

# Introduction

## Application vs. Systems Programming

### Application Programming
- Involves creating software with a high-level, abstract approach.
- Examples include C# and Java, languages designed for building business applications like spreadsheets, word processors, and apps for web or mobile.

### Systems Programming
- Focuses on constructing software and software platforms using a low-level approach, facilitating explicit and sophisticated interaction with hardware.
- Examples are C, C++, and Rust, which are used in the development of operating systems, game engines, and compilers.

## Why Rust?
Rust is versatile, capable of handling both high-level applications and low-level, hardware-specific programming.

It excels in the following aspects:
- Safety: Prevents common bugs and security vulnerabilities at compile-time.
- Speed: Matches the performance of languages like C and C++ without a garbage collector.
- Concurrency: Enables safe concurrent programming, preventing data races.

### Performance
- Rust's lack of a Garbage Collector (GC) boosts performance. It manages memory through ownership and borrowing, eliminating items from memory once they're no longer in scope.

### Memory Safety at Compile Time
- Rust prevents common memory safety issues such as dangling pointers, buffer overruns, and memory leaks through its ownership model.

### Multi-Threading
- The ownership and borrowing concepts in Rust ensure safe concurrency, enabling efficient multi-threading without the risk of data races.

### Support for WebAssembly (WASM)
- Rust's ability to compile into WebAssembly (WASM) makes it an excellent choice for developing high-performance, web-based applications.

### Additional Advantages
- Cargo, Rust's package manager and build system, streamlines dependency management and code compilation.
- Rust offers detailed error messages, aiding in rapid debugging and development.
- Asynchronous programming support in Rust facilitates efficient, non-blocking I/O operations, crucial for scalable web services.
- The Rust ecosystem is rapidly expanding, with a wide array of libraries and tools for web development, embedded systems, and more, reflecting its growing popularity and versatility.

# Environment Setup
- Install Rust: [Rust Installation](https://www.rust-lang.org/tools/install)

# Hello World Example
1. Create a new project:
   - `cargo new hello_world`
2. Navigate to the project directory:
   - `cd hello_world`
3. Build the project:
   - `cargo build`
4. Run the program:
   - In VS Code (requires the rust-analyzer extension):
     - Execute/debug the program using the rust-analyzer extension. 
        ![alt text](<images/run-dubug-in-vs-code.png>)
     - Debugging guide: [VS Code Rust Debugging](https://code.visualstudio.com/docs/languages/rust#_debugging)
   - OR directly via the executable `rustc` creates:
     - Compile the program: `rustc .\src\main.rs`
     - Execute the compiled program: `.\main.exe`

# Data Types

Rust provides rich data types organized into scalar, compound, and other special categories, allowing for precise and safe data handling.

## Scalar Types
Scalar types represent a single value. Rust has four primary scalar types:

### Integer Types
- Signed (negative and positive): `i8`, `i16`, `i32` (default), `i64`, `i128`, `isize` (pointer size).
- Unsigned (positive only): `u8`, `u16`, `u32`, `u64`, `u128`, `usize` (pointer size).
- Use underscores to improve readability: `let x: i32 = 1_000;`
- Overflow behavior: Rust performs two's complement wrapping in debug mode, but it can be avoided with methods like `wrapping_add`, `checked_add`, etc.
- Example:

  ```rust
  let x: i32 = 1_000;
  let y: u8 = 255;
  ```

### Floating-Point Types
- f32: Single precision.
- f64: Double precision (default).
- Suitable for scientific calculations requiring large decimal numbers.
- Example:

  ```rust
  let pi: f64 = 3.141592;
  let e: f32 = 2.7182;
  ```

### Boolean Type
- bool: Can be `true` or `false`.
- Integral in conditionals and loops.
- Example:

  ```rust
  let is_active: bool = true;
  ```

### Character Type
- char: 4 bytes, represents a Unicode Scalar Value.
- Enclosed in single quotes: `let heart_eyed_cat = '😻';`

## Compound Types
Allow multiple values, possibly of different types, to be grouped together.

### Tuple Type
- Fixed-size, immutable once declared.
- Can mix different types: `let tup: (i32, f64, u8) = (500, 6.4, 1);`
- Destructure to use values: `let (x, y, z) = tup;`
- Example:

  ```rust
  let tup: (i32, f64, char) = (500, 6.4, '🚀');
  let (x, y, z) = tup; // Destructuring
  ```

### Array Type
- All elements must be the same type.
- Fixed length: `let a = [1, 2, 3];`
- Access elements by indexing: `let first = a[0];`
- Arrays are stack allocated, making them useful for data that needs to be stored on the stack rather than the heap.
- Example:

  ```rust
  let months: [&str; 12] = [
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December"
  ];
  ```

## Special Types

### The Unit Type
- Represented by `()`, it signifies the absence of a value or that an expression doesn't return anything meaningful.
- Example:

  ```rust
  fn no_return() -> () { println!("This function returns nothing."); }
  ```

### Strings
- String Literals (&str): Immutable, stored in the binary.
- String Object (String): Heap-allocated, mutable, growable.
- Conversion: `let s = "hello".to_string();`
- Example:

  ```rust
  let static_str: &str = "Hello, static world!";
  let dynamic_str: String = String::from("Hello, dynamic world!");
  ```

## Enums
- Define a type by enumerating its possible variants.
- Use with `match` for pattern matching.
- Can hold data: `enum Message { Quit, Move { x: i32, y: i32 }, Write(String), ChangeColor(i32, i32, i32), }`
- Example:

  ```rust
  fn main() {
      enum Direction {
          Up,
          Down,
          Left,
          Right,
      }

      let go = Direction::Up;

      match go {
          Direction::Up => println!("Going up!"),
          _ => println!("Going somewhere else!"),
      }
  }
  ```

## Pattern Matching and `match`
- Powerful control flow that compares a value against a series of patterns and executes the code of the matching pattern.
- Patterns can be literals, variable names, wildcards, and many others.
- Example:

  ```rust
  fn main() {
      let number = 13;

      match number {
          1 => println!("It is one!"),
          2..=12 => println!("It is between two and twelve!"),
          _ => println!("It is something else!"),
      }
  }
  ```

## Memory Safety and Management
- Ownership: Ensures memory safety without a garbage collector.
- Borrowing: References (&) allow you to use values without taking ownership, maintaining safety through the borrow checker.
- Example:

  ```rust
  fn main() {
      let s1 = String::from("hello");

      let len = calculate_length(&s1);

      println!("The length of '{}' is {}.", s1, len);

      fn calculate_length(s: &String) -> usize {
          s.len()
      }
  }
  ```

# Variables

## Declaration
Variables are declared using the `let` keyword. Rust allows for optional type annotations during this process, thanks to its ability to infer data types from the values assigned to variables.

- Without type specification: `let variable_name = value;`
- With type specification: `let variable_name: dataType = value;`

## Naming Conventions
Variable names in Rust can include letters, digits, and underscores, but must start with a letter or an underscore. Rust is case-sensitive, making upper and lower case letters distinct in names.

## Mutability
By default, variables in Rust are immutable (read-only) for safety and concurrency. To declare a variable whose value can be changed, use the `mut` keyword.

- Immutable: `let fees = 25_000;`
- Mutable: `let mut fees = 25_000;`