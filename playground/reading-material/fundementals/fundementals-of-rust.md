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

# Constants

## Declaration
To declare a constant, use the `const` keyword followed by an explicit data type annotation. Constants require a data type to be specified, as they do not allow for type inference.

- Syntax: `const VARIABLE_NAME: dataType = value;`

## Naming Convention
Constants are conventionally named using uppercase letters with underscores separating words. This convention helps distinguish constants from variables, making the code more readable.

- Example: `const MAX_POINTS: u32 = 100_000;`

## Constants vs. Variables
- **Declaration**: Constants use `const`, and variables use `let`.
- **Mutability**: Constants are always immutable. While variables are immutable by default, they can be made mutable with `mut`.
- **Type Inference**: Constants require explicit type annotation; variables do not.
- **Scope**: Constants can be declared in any scope, including the global scope, making them useful across different parts of a program.
- **Runtime Evaluation**: Constants can only be set to a constant expression, not the result of a function or any value computed at runtime.

## Shadowing
Rust allows the shadowing of variables but not constants. Shadowing occurs when a new variable is declared with the same name as a previous variable, effectively overriding it within a specific scope.

- Variables can be shadowed, potentially even changing the type with the new declaration.
- Constants cannot be shadowed. Attempting to do so results in a compilation error.

## Examples

### Declaring Constants
```rust
fn main() {
    const USER_LIMIT: i32 = 100; // Integer constant
    const PI: f32 = 3.14;        // Float constant

    println!("User limit is {}", USER_LIMIT);
    println!("Pi value is {}", PI);
}
```

### Attempting to Shadow a Constant
```rust
fn main() {
    const NAME: &str = "Rustacean";
    // Attempting to shadow `NAME` will result in a compile-time error
    // const NAME: usize = NAME.len(); // This will not compile
    println!("The name is {}", NAME);
}
```

Constants play a crucial role in Rust's emphasis on safety and efficiency, providing a reliable mechanism for defining immutable, global values accessible across the entirety of a program.

# Strings

## Types of Strings
- **String Literal (`&str`)**: Known at compile time, immutable, and stored in the program's binary.
- **String Object (`String`)**: Heap-allocated, mutable, growable, and UTF-8 encoded.

## String Literal (`&str`)
- Declared with `let` and the type `&str`.
- Static by default; exists for the entire program's duration.
- Example: `let company:&str = "TutorialsPoint";`

## String Object (`String`)
- Not part of the core language; provided by the Standard Library.
- Can be created empty or from a literal.
- Example: `let mut s = String::from("Hello");`

## Common Methods for `String`
- `new()`: Creates an empty `String`.
- `to_string()`: Converts a literal to a `String`.
- `replace(from, to)`: Replaces occurrences of a pattern.
- `as_str()`: Converts `String` back to string slice `&str`.
- `push(ch)`: Appends a character.
- `push_str(s)`: Appends a string slice.
- `len()`: Returns the length in bytes.
- `trim()`: Removes leading/trailing whitespace.
- `split_whitespace()`: Splits by whitespace into an iterator.
- `split(pattern)`: Splits by a pattern into an iterator.
- `chars()`: Returns an iterator over characters.

## String Concatenation
- Use `+` operator or `format!` macro for concatenation.
- `+` requires the second string to be a slice (`&str`).
- `format!` is more flexible and does not take ownership of the original strings.

## Illustrations
- **Creating and Modifying**: 
    - `let mut s = String::new(); s.push_str("hello");`
- **Replacing Substrings**: 
    - `let replaced = s.replace("hello", "world");`
- **Trimming Spaces**: 
    - `let trimmed = s.trim();`
- **Splitting Strings**: 
    - Use `split_whitespace` or `split(',')` for specific patterns.
- **Accessing Characters**: 
    - Iterate with `for c in s.chars() {}`

## Advanced
- **Type Casting**: Use `to_string()` to convert numbers or other types to `String`.
- **Using `format!` Macro**: Combine strings without taking ownership, e.g., `let full = format!("{} {}", s1, s2);`

# Operators

## Arithmetic Operators
- `+`: Addition (e.g., `a + b` is 15)
- `-`: Subtraction (e.g., `a - b` is 5)
- `*`: Multiplication (e.g., `a * b` is 50)
- `/`: Division (returns quotient, e.g., `a / b` is 2)
- `%`: Modulus (returns remainder, e.g., `a % b` is 0)
- Rust does not support `++` and `--`.

## Relational Operators
- Return Boolean value (true or false).
- `>`: Greater than
- `<`: Lesser than
- `>=`: Greater than or equal to
- `<=`: Lesser than or equal to
- `==`: Equality
- `!=`: Not equal

## Logical Operators
- Combine conditions, return Boolean.
- `&&`: AND (true if all true)
- `||`: OR (true if at least one is true)
- `!`: NOT (inverse of expression's result)

## Bitwise Operators
- Operate on bits of integer arguments.
- `&`: AND (1 if both bits are 1)
- `|`: OR (1 if at least one bit is 1)
- `^`: XOR (1 if bits are different)
- `!`: NOT (inverts bits, e.g., `!3` is -4 because it inverts all bits)
- `<<`: Left Shift (shifts bits left, equivalent to multiplying by 2^n)
- `>>`: Right Shift (shifts bits right, dividing by 2^n)
- `>>>`: Right Shift with Zero (like `>>` but fills with zeros)

### Explaining Bit Operations
- **AND (`&`)**: Each bit of the result is 1 if both corresponding bits are 1. E.g., `2 & 3` (10 & 11) is 2 (10).
- **OR (`|`)**: Each bit is 1 if at least one of the corresponding bits is 1. E.g., `2 | 3` (10 | 11) is 3 (11).
- **XOR (`^`)**: Each bit is 1 if the corresponding bits are different. E.g., `2 ^ 3` (10 ^ 11) is 1 (01).
- **NOT (`!`)**: Inverts the bits of its operand. Note: In Rust, `!` applied to `3` results in `-4`, due to how binary negation works with two's complement representation.
- **Left Shift (`<<`)**: Shifts the bits to the left, multiplying by 2 for each shift. E.g., `2 << 1` shifts bits of 2 (10) left by 1, resulting in 4 (100).
- **Right Shift (`>>`)**: Shifts bits to the right, dividing by 2 for each shift, filling with the sign bit. E.g., `4 >> 1` shifts bits of 4 (100) right by 1, resulting in 2 (10).
- **Right Shift with Zero (`>>>`)**: Similar to `>>` but fills shifted bits with 0. Not directly supported in Rust, but equivalent in functionality to `>>` for unsigned integers.

# Decision Making

## Decision Structures
- `if`, `if...else`, `else...if`, and `match`.

## `if` Statement
- Executes code block if condition is true.

```rust
let num = 5;
if num > 0 {
   println!("number is positive");
}
```

## `if...else` Statement
- Executes one block if condition is true, another if false.

```rust
let num = 12;
if num % 2 == 0 {
   println!("Even");
} else {
   println!("Odd");
}
```

## Nested `if` and `else...if`
- Checks multiple conditions sequentially.

```rust
let num = 2;
if num > 0 {
    println!("{} is positive", num);
} else if num < 0 {
    println!("{} is negative", num);
} else {
    println!("{} is neither positive nor negative", num);
}
```

## `match` Statement
- Similar to switch case in other languages.
- Compares a value against a series of patterns and executes the code of the matching pattern.

```rust
let state_code = "MH";
let state = match state_code {
    "MH" => "Maharashtra",
    "KL" => "Kerala",
    "KA" => "Karnataka",
    "GA" => "Goa",
    _ => "Unknown", // default case
};
println!("State name is {}", state);
```

# Loops

## Overview
- Rust provides `while`, `loop`, and `for` for handling loops.

## For Loop (Definite Loop)
- Executes a block of code a fixed number of times.
- Iterates over a range or collection.

```rust
for x in 1..11 { // 11 is not inclusive
    if x == 5 {
        continue; // Skips the rest of this iteration
    }
    println!("x is {}", x);
}
// Note: `x` is accessible only within the for loop block.
```

## While Loop (Indefinite Loop)
- Executes as long as a condition remains true.

```rust
let mut x = 0;
while x < 10 {
    x += 1;
    println!("inside loop x value is {}", x);
}
println!("outside loop x value is {}", x);
```

## Loop (Indefinite Loop)
- Executes indefinitely until explicitly terminated with `break`.

```rust
let mut x = 0;
loop {
    x += 1;
    println!("x={}", x);

    if x == 15 {
        break; // Exits the loop
    }
}
// `break` exits the loop, `continue` skips to the next iteration.
```

## Continue Statement
- Skips the current loop iteration and proceeds with the next one.

```rust
let mut count = 0;
for num in 0..21 {
    if num % 2 == 0 {
        continue; // Skips even numbers
    }
    count += 1;
}
println!("The count of odd values between 0 and 20 is: {}", count);
// Counts and prints the number of odd values between 0 and 20.
```

## Key Points
- `for` loop is used for definite iterations.
- `while` and `loop` can be used for indefinite iterations, with `loop` running infinitely unless stopped by `break`.
- `continue` skips the rest of the current iteration.
- Loops are a fundamental part of controlling the flow of a Rust program, allowing repetitive tasks to be performed efficiently.

# Functions

## Key Concepts

### 1. Defining a Function
- Use the `fn` keyword.
- Functions can optionally have parameters.
- Syntax: `fn function_name(param1, param2..paramN) { // function body }`
- Example:
  ```rust
  fn fn_hello(){
     println!("hello from function fn_hello ");
  }
  ```

### 2. Invoking a Function
- Functions must be called to execute.
- The function that invokes another function is called the caller function.
  - e.g. `main()` in the example below.
- Syntax: `function_name(val1, val2, valN)`
- Example:
  ```rust
  fn main(){  //Here, main() is the caller function.
    // Calling a function
    fn_hello();
  }

  //Defining a function
  fn fn_hello(){
    println!("hello from function fn_hello ");
  }
  ```

### 3. Returning Functions
- Functions can return values using `return` keyword or by the last expression without a semicolon.
- Syntax1: `fn function_name() -> return_type { return value; }`
- Syntax2: `fn function_name() -> return_type { value } // no semicolon`
- Example:
  ```rust
  fn main(){
     println!("pi value is {}", get_pi());
  }
  fn get_pi() -> f64 {
     22.0/7.0
  }
  ```

### 4. Function with Parameters
- Parameters pass values to functions.
- Passed by value or by reference.
- Pass by value: Copies the actual value, changes inside function don't affect the original variable.
- Pass by reference: Uses `&` to refer to the original memory location, changes inside function affect the original variable.
- Example (Pass by Value):
  ```rust
  fn main(){
     let no: i32 = 5;
     mutate_no_to_zero(no);
     println!("The value of no is:{}", no);
  }
  fn mutate_no_to_zero(mut param_no: i32) {
     param_no = 0;
     println!("param_no value is :{}", param_no);
  }
  ```

- Example (Pass by Reference):
  ```rust
  fn main() {
     let mut no: i32 = 5;
     mutate_no_to_zero(&mut no);
     println!("The value of no is:{}", no);
  }
  fn mutate_no_to_zero(param_no: &mut i32) {
     *param_no = 0; // dereferencing
  }
  ```

### Passing Strings to Functions
## String Passing and Ownership
- When a `String` object is passed to a function, Rust uses move semantics unless explicitly borrowed.
- This means the original variable no longer owns the `String` and cannot be used after being passed unless it's passed by reference.
- Passing by reference (`&String`) or mutable reference (`&mut String`) avoids taking ownership, allowing the original variable to remain usable.
- Example (Ownership is moved):
  ```rust
  fn main(){
     let name = String::from("TutorialsPoint");
     display(name); 
     // Error: `name` value moved and not accessible here
  }
  fn display(param_name: String) {
     println!("Name: {}", param_name);
  }
  ```
- Example (Passing by reference, ownership not moved):
  ```rust
  fn main(){
     let name = String::from("TutorialsPoint");
     display(&name); 
     // `name` is still accessible here
  }
  fn display(param_name: &String) {
     println!("Name: {}", param_name);
  }
  ```

## Tuple

### Overview
- Tuples are compound data types that store more than one value at a time, potentially of different types.
- They have a fixed length and cannot grow or shrink.
- Tuple index starts from 0.

### Syntax
- To declare a tuple: `let tuple_name:(data_type1, data_type2, data_type3) = (value1, value2, value3);`
- Alternatively: `let tuple_name = (value1, value2, value3);`

### Accessing Values
- Use `tuple_name.index` to access individual values.
- Example:
  ```rust
  let tuple:(i32, f64, u8) = (-325, 4.9, 22);
  println!("integer: {:?}", tuple.0); // Accesses the first value
  println!("float: {:?}", tuple.1); // Accesses the second value
  println!("unsigned integer: {:?}", tuple.2); // Accesses the third value

### Passing Tuples to Functions
- Tuples can be passed by value to functions.
- Example:
  ```rust
  fn main(){
     let b:(i32, bool, f64) = (110, true, 10.9);
     print_tuple(b);
  }
  fn print_tuple(x:(i32, bool, f64)){
     println!("Inside print_tuple method");
     println!("{:?}", x);
  }

### Destructuring
- Destructuring allows unpacking of tuple values into separate variables.
- Example:
  ```rust
  fn main(){
     let b:(i32, bool, f64) = (30, true, 7.9);
     print_tuple(b);
  }
  fn print_tuple(x:(i32, bool, f64)){
     let (age, is_male, cgpa) = x; // Destructuring
     println!("Age: {} , isMale? {}, CGPA: {}", age, is_male, cgpa);
  }
