# Rust Multithreaded Port Sniffer

A multithreaded port sniffer written in Rust designed to efficiently scan for open ports on a specified IP address using concurrent threads. It parses command-line inputs to accept an IP address and an optional thread count, then reports back any open ports it discovers.

## Allowed Arguments

- **IP address**: The target IP address for the port scan.
- **-j <number>**: Optionally set the number of threads to use for the scan. Default is 50 threads.
- **-h** or **-help**: Display help information and usage instructions.

## Example Usage

- Scan with default settings (50 Threads):
    ```shell
    cargo run -- 192.168.1.1
    ```

- Scan using 1000 threads:
    ```shell
    cargo run -- -j 1000 192.168.1.1
    ```

- Display help information:
    ```shell
    cargo run -- -h
    ```
