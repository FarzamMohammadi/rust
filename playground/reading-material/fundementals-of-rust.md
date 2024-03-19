# Rust

# Intro

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