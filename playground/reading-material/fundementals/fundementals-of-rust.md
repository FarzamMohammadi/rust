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