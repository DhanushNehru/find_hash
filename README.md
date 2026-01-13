# find_hash

Find Hash will help to find what type of hash it is. 

**Instant, offline hash type identification.**

`find_hash` is a high-performance command-line utility designed to quickly analyze and identify unknown hash strings. Built in Rust for speed and reliability, it supports over 200 hash algorithms and provides compatible mode codes for popular cracking tools like Hashcat and John the Ripper.

## Why find_hash?
*   **Blazing Fast**: Engineered in Rust with zero runtime overhead. Instant results, every time.
*   **Secure & Offline**: All analysis happens locally on your machine. No keys are sent to the cloud.
*   **Cracker Ready**: Automatically maps identified hashes to their corresponding **Hashcat** modes and **John the Ripper** formats.
*   **Automation Friendly**: Clean, structured JSON output support makes it perfect for pipeline integration.

## Installation
### From Source
Requires [Rust](https://rustup.rs/).
```bash
git clone https://github.com/DhanushNehru/find_hash
cd find_hash
```