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
   ```

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
   ```

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
  ```

# Arrays
- Arrays hold multiple values, unlike scalar variables which hold one value.
- Arrays allocate memory sequentially, unlike variables which allocate randomly.
- An array is a collection of similar values.
- Arrays are fixed in size once initialized.
- Each array element occupies a memory block.
- Array elements are accessed by a unique index.
- Array values can be updated but not deleted.

## Examples of Array Declaration

```rust
// Syntax1 - Array type inferred from first element's data type
let variable_name = [value1, value2, value3];

// Syntax2 - Explicit type and size
let variable_name: [dataType; size] = [value1, value2, value3];

// Syntax3 - Initialize all elements to a default value
let variable_name: [dataType; size] = [default_value_for_elements; size];
```

## Array Declaration With Default Values

```rust
fn main() {
   let arr: [i32; 4] = [-1; 4];
   println!("array is {:?}", arr);
   println!("array size is :{}", arr.len());
}
```

Outputs:

```plaintext
array is [-1, -1, -1, -1]
array size is :4
```

## Iterating Arrays

### For Loop

```rust
fn main() {
   let arr: [i32; 4] = [10, 20, 30, 40];
   println!("array is {:?}", arr);
   println!("array size is :{}", arr.len());

   for index in 0..4 {
      println!("index is: {} & value is : {}", index, arr[index]);
   }
}
```

Outputs:

```plaintext
array is [10, 20, 30, 40]
array size is :4
index is: 0 & value is : 10
index is: 1 & value is : 20
index is: 2 & value is : 30
index is: 3 & value is : 40
```

### `iter()` Function

```rust
fn main() {
let arr: [i32; 4] = [10, 20, 30, 40];
   println!("array is {:?}", arr);
   println!("array size is :{}", arr.len());

   for val in arr.iter() {
      println!("value is :{}", val);
   }
}
```

Outputs:
```plain text
array is [10, 20, 30, 40]
array size is :4
value is :10
value is :20
value is :30
value is :40
```

## Array Mutation
Similar to C#:

```rust
fn main() {
   let mut arr: [i32; 4] = [10, 20, 30, 40];
   arr[1] = 0;
   println!("{:?}", arr);
}
```

Outputs:
```plaintext
[10, 0, 30, 40]
```

## Passing Arrays as Parameters to Functions
Arrays can be passed by value or reference.

### Passing By Value

```rust
fn main() {
   let arr = [10, 20, 30];
   update(arr);

   print!("Inside main {:?}", arr);
}

fn update(mut arr: [i32; 3]) {
   for i in 0..3 {
      arr[i] = 0;
   }
   println!("Inside update {:?}", arr);
}
```

Outputs:

```plaintext
Inside update [0, 0, 0]
Inside main [10, 20, 30]
```

### Passing By Reference

```rust
fn main() {
   let mut arr = [10, 20, 30];
   update(&mut arr);
   print!("Inside main {:?}", arr);
}

fn update(arr: &mut [i32; 3]) {
   for i in 0..3 {
      arr[i] = 0;
   }
   println!("Inside update {:?}", arr);
}
```

Outputs:

```plaintext
Inside update [0, 0, 0]
Inside main [0, 0, 0]
```

## Array Declaration and Constants

```rust
fn main() {
   let N: usize = 20;
   let arr = [0; N]; //Error: non-constant used with constant
   print!("{}", arr[10])
}
```

The compiler throws an exception because the array's length cannot be determined at compile time. `N` variable value will be determined at runtime, not compile time.

To fix we set `N` as a constant. This way, its value can be determined at compile time and therefore cannot be changed at runtime:

```rust
fn main() {
   const N: usize = 20; 
   let arr = [0; N];

   print!("{}", arr[10])
}
```

# Memory Allocation
The memory of a program is allocated in
1. Stack
2. Heap

## Stack
- Operates on a last-in, first-out basis.
- Ideal for storing fixed-size values known at compile time, such as variables of type `i32`.
- All scalar values (e.g., integers, booleans) are stored here due to their predetermined size.

## Heap
- Accommodates values with sizes unknown at compile time or those that can change size, like strings.
- Serves for dynamic data storage, accommodating growth or shrinkage over the program's lifecycle.
- The Heap is less structured compared to the Stack, allowing for the flexibility of dynamic memory allocation.

# Memory Management
## Ownership
- Every value in Rust has a unique 'owner'.
  - For example, in `let age = 30`, `age` owns the data value `30`.
- A piece of data can only have one owner at a time.
  - Data refers to the location in memory, not the value itself.
- Two variables cannot directly reference the same memory location.
- Primitive types are copied when assigned or passed to functions, not following the strict ownership rules.
  - Primitives are simpler and take up less memory, making copying efficient.
- Strict ownership rules apply to non-primitive types.
  - Copying complex data structures, like arrays, is resource-intensive.

## Transferring Ownership
Ownership can be transferred by:
1. Assigning the value of one variable to another
2. Passing the value to a function
3. Returning the value from a function

### Assigning the value of one variable to another
In the example:

```rust
fn main(){
   let v = vec![1,2,3]; 
   // vector v owns the object in heap

   // only a single variable owns the heap memory at any given time
   let v2 = v; 
   // here two variables owns heap value,
   // two pointers to the same content is not allowed in rust

   // Rust is very smart in terms of memory access ,so it detects a race condition
   // as two variables point to same heap

   println!("{:?}",v); // Throws exception due to race condition
}
```

The variable `v` loses its ownership once it's assigned to `v2`, making `v` unusable thereafter.

### Passing value to a function
In the example:

```rust
fn main(){
   let v = vec![1,2,3];     // vector v owns the object in heap
   let v2 = v;              // moves ownership to v2
   display(v2);             // v2 is moved to display and v2 is invalidated
   println!("In main {:?}",v2);    //v2 is No longer usable here
}

fn display(v:Vec<i32>){
   println!("inside display {:?}",v);
}
```

The `println!` call will result in a compile-time error because `v2` is no longer valid after being moved to the function.

### Returning value from a function
In this snippet:

```rust
fn main(){
   let v = vec![1,2,3];       // vector v owns the object in heap
   let v2 = v;                // moves ownership to v2
   let v2_return = display(v2);    
   println!("In main {:?}",v2_return);
}

fn display(v:Vec<i32>)->Vec<i32> { 
   println!("inside display {:?}",v);
}
```

Both `v` and `v2` are invalidated during their transfer. `v` is invalidated when assigned to `v2`, and `v2` is invalidated when passed to the function. We regain access to the same value and its memory location by returning it from the function.

# Borrowing
Borrowing in Rust allows for the temporary use of a value without transferring ownership. This is particularly useful when you want to access data while still keeping it under the original owner's control.

- Borrowing is achieved by passing a reference (`&var_name`) instead of the actual value. This method temporarily lends the value to a function or another variable without giving up ownership.
- A reference allows a function to access the value without owning it, ensuring that the original owner retains its ownership rights after the function completes.

   ```rust
   fn main(){
      let v = vec![10,20,30];
      print_vector(&v); // passing reference
      println!("Printing the value from main() v[0]={}",v[0]);
   }

   fn print_vector(x:&Vec<i32>){
      println!("Inside print_vector function {:?}",x);
   }
   ```
## Mutable References
By default, references in Rust are immutable, meaning the borrowed data cannot be altered. If you need to modify the borrowed data, Rust requires explicit permissions through mutable references.

- To create a mutable reference, both the original data and the reference must be declared as mutable (`mut`). This explicit handling helps prevent data races by ensuring that only one mutable reference to a piece of data exists at a time in a given scope.
- Mutable references allow temporary modification rights to the borrowed data, under strict rules to ensure safe concurrency.

   ```rust
   fn main() {
      let mut i = 3;
      add_one(&mut i);
      println!("{}", i);
   }

   fn add_one(e: &mut i32) {
      *e+= 1;
   }
   ```

# Slices
Slices provide a way to access contiguous segments of data in structures like arrays, vectors, and strings without copying the original data.

- Essentially, slices are references to a portion of data, allowing efficient access without ownership transfer.
- Important: In Rust, slice ranges are defined with a start index (inclusive) and an end index (exclusive). Thus, a slice from [start..end] includes the element at the start index up to, but not including, the element at the end index.

Example 1:

```rust
fn main() {
   let n1 = "Tutorials".to_string();
   println!("length of string is {}",n1.len());
   let c1 = &n1[4..9]; 
   
   // fetches characters at 4,5,6,7, and 8 indexes
   println!("{}",c1);
}
```

Output:

```plaintext
length of string is 9
rials
```

In this example, a slice of a String object is created, capturing a portion of the string based on specified indices.

Example 2:

```rust
fn main(){
   let data = [10,20,30,40,50];
   use_slice(&data[1..4]);
   //this is effectively borrowing elements for a while
}

fn use_slice(slice:&[i32]) { 
   // is taking a slice or borrowing a part of an array of i32s
   println!("length of slice is {:?}",slice.len());
   println!("{:?}",slice);
}
```

Output:

```plaintext
length of slice is 3
[20, 30, 40]
```

This example demonstrates slicing an array to access a subset of its elements, effectively borrowing part of the array for temporary use.

## Mutable Slices
Just as variables can be mutable, slices can also be mutable when declared with the `mut` keyword.

Example:

```rust
fn main(){
   let mut data = [10,20,30,40,50];
   use_slice(&mut data[1..4]);
   // passes references of 
   20, 30 and 40
   println!("{:?}",data);
}

fn use_slice(slice:&mut [i32]) {
   println!("length of slice is {:?}",slice.len());
   println!("{:?}",slice);
   slice[0] = 1010; // replaces 20 with 1010
}
```

Output:

```plaintext
length of slice is 3
[20, 30, 40]
[10, 1010, 30, 40, 50]
```

Here, a mutable slice of an array is created, allowing modification of the elements within the specified range. Note how the first element of the slice is modified, demonstrating the mutable nature of slices.

# Structure
Structures, or structs, allow us to package together data items of different types, including other structures. They define data as key-value pairs.

## Struct Declaration & Initialization
- Use the `struct` keyword to define a structure.
- Structs are statically typed and must be associated with a specific data type.
- Struct naming follows the same rules as variables.
- When initializing a struct, a struct block must end with a semicolon.

```rust
struct Employee {
   name:String,
   company:String,
   age:u32
}

fn main() {
   let employee_one = Employee {
      company:String::from("Dudes In the Hood"),
      name:String::from("Farzam"),
      age:69
   };
   println!("Name is :{} company is {} age is {}", employee_one.name, employee_one.company, employee_one.age);
}
```

## Struct Mutation
To mutate a struct, the struct itself must be declared as mutable using the `mut` keyword. Once declared, its fields can be altered.

```rust
let mut employee_one = Employee {
   company:String::from("Dudes In the Hood"),
   name:String::from("Farzam"),
   age:69
};

employee_one.age = 27;

println!("Name is :{} company is {} age is {}", employee_one.name, employee_one.company, employee_one.age);
```

## Struct Function Handling
Structs can be passed to and returned from functions, demonstrating how functions can operate on and return structs.

```rust
fn main() {
   let employee_one = Employee {
      company:String::from("Dudes In the Hood"),
      name:String::from("Farzam"),
      age:69
   };

   let employee_one = Employee {
      company:String::from("Dudes In the Hood"),
      name:String::from("Billy"),
      age:96
   };

   let elder = who_is_elder(emp1,emp2);

   println!("elder is:");

   display(elder);
   // Output:
   // elder is:
   // Name is :Billy company is Dudes In the Hood age is 96
}

fn who_is_elder (employee_one:Employee, employee_two:Employee)->Employee {
   if employee_one.age > employee_two.age {
      return employee_one;
   } else {
      return employee_two;
   }
}

fn display(employee:Employee) {
   println!("Name is :{} company is {} age is {}", employee.name, employee.company, employee.age);
}

struct Employee {
   name:String,
   company:String,
   age:u32
}
```

## Structure Instance Methods
Instance methods for structs are defined within an `impl` (implementation) block. These methods can access or modify the state of the struct instance.

- The first parameter of an instance method is always `self`, which represents the instance on which the method is being called.
- Instance methods are similar to class methods in object-oriented programming.

```rust
struct Rectangle {
   width:u32, 
   height:u32
}

impl Rectangle {
   fn area(&self) -> u32 {
      self.width * self.height
   }
}

fn main() {
   let small = Rectangle {
      width:10,
      height:20
   };

   println!("width is {} height is {} area of Rectangle is {}", small.width, small.height, small.area());
}
```

## Struct Static Methods
Static methods are associated with the struct itself, not any particular instance. They do not take `self` as a parameter.

- Static methods are called using the syntax `StructureName::methodName(arguments)`.

```rust
struct Point {
   x: i32,
   y: i32,
}

impl Point {
   fn getInstance(x: i32, y: i32) -> Point {
      Point { x: x, y: y }
   }

   fn display(&self){
      println!("x ={} y={}",self.x,self.y );
   }
}

fn main(){
   let p1 = Point::getInstance(10,20);

   p1.display();
}
```

# Enums

## Basic Enum
Enums allow you to define a type by enumerating its possible variants. Here's a simple example of an enum representing gender categories:

```rust
// The `derive` attribute automatically provides the implementation required to make this `enum` printable with `fmt::Debug`.
#[derive(Debug)]
enum GenderCategory {
   Male,
   Female
}

fn main() {
   let male = GenderCategory::Male;
   let female = GenderCategory::Female;

   println!("{:?}", male); // Male
   println!("{:?}", female); // Female
}
```

## Struct With Enum
Structs can include enums as fields. This allows for a composition of complex types, adding flexibility in data structuring. Here's how an enum can be used inside a struct:

```rust
#[derive(Debug)]
struct Person {
   name:String,
   gender:GenderCategory
}

fn main() {
   let p1 = Person {
      name:String::from("Farzam"),
      gender:GenderCategory::Male
   };

   let p2 = Person {
      name:String::from("Amy"),
      gender:GenderCategory::Female
   };

   println!("{:?}",p1); Person { name: "Farzam", gender: Male }

   println!("{:?}",p2); Person { name: "Amy", gender: Female }
}
```

## Option Enum
The `Option` enum is predefined in Rust's standard library and is used to encapsulate the possible absence of a value. It's a safer alternative to nulls found in other languages.

- `Some(T)`: Contains a value of type `T`.
- `None`: Represents the absence of a value.

```rust
enum Option<T> {
   Some(T),      // used to return a value
   None          // used to return null, as Rust doesn't support the null keyword
}
```

```rust
fn main() {
   let result = is_even(3);

   println!("{:?}", result); // None
   println!("{:?}", is_even(30)); // Some(true)
}

fn is_even(no:i32)->Option<bool> {
   if no%2 == 0 {
      Some(true)
   } else {
      None
   }
}
```

## Match Statement & Enum
The `match` statement in Rust is a control flow construct that allows handling of different cases based on the enum variants. It's similar to switch statements in other languages but is more powerful.

```rust
enum CarType {
   Hatch,
   Sedan,
   Suv
}

fn print_size(car:CarType) {
   match car {
      CarType::Hatch => {
         println!("Small sized car");
      },
      CarType::Sedan => {
         println!("medium sized car");
      },
      CarType::Suv =>{
         println!("Large sized Sports Utility car");
      }
   }
}

fn main(){
   print_size(CarType::Suv); // Large sized Sports Utility car
   print_size(CarType::Hatch); // Small sized car
   print_size(CarType::Sedan); // Medium sized car
}
```

## Match Statement With Option
The `match` statement can be effectively used with the `Option` enum to handle cases where a value might or might not be present.

```rust
fn main() {
   match is_even(5) {
      Some(data) => {
         if data==true {
            println!("Even no");
         }
      },
      None => {
         println!("Not even");
      }
   }
}

fn is_even(no:i32)->Option<bool> {
   if no%2 == 0 {
      Some(true)
   } else {
      None
   }
}

// Output: Not even
```

## Enum with Associated Data
Enums can also hold different types of data in each of their variants. This allows for versatile data structures that are neatly encapsulated within an enum.

```rust
#[derive(Debug)]
enum GenderCategory {
   Name(String),Usr_ID(i32)
}

fn main() {
   let p1 = GenderCategory::Name(String::from("Farzam"));
   let p2 = GenderCategory::Usr_ID(100);

   println!("{:?}", p1);
   println!("{:?}", p2);

   match p1 {
      GenderCategory::Name(val)=> {
         println!("{}", val);
      }
      GenderCategory::Usr_ID(val)=> {
         println!("{}", val);
      }
   }
}

// Output:
// Name("Farzam")
// Usr_ID(100)
// Farzam
```