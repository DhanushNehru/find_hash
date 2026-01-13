# find_hash

High-performance, offline hash type identification tool written in Rust.

## Usage

### Run with Cargo (Recommended for Dev)

You can run the tool directly without installing it:

```bash
cargo run -- <HASH>
```

**Example:**

```bash
cargo run -- e3b98a4da31a127d4bde6e43033f66ba274cab0eb7eb1c70ec41402bf6273cf8
```

**Output:**

```text
Analyzing hash: e3b98a4da31a127d4bde6e43033f66ba274cab0eb7eb1c70ec41402bf6273cf8
Most Likely:
  1. SHA-256
     Hashcat Mode: 1400
     John Format:  raw-sha256
  2. Haval-256
     John Format:  haval-256-4
```

### JSON Output

Get machine-readable output for scripts:

```bash
cargo run -- e3b98a4da31a127d4bde6e43033f66ba274cab0eb7eb1c70ec41402bf6273cf8 --json
```

## Installation (Optional)

If you want to use the `find_hash` command globally (without `cargo run`), install it:

```bash
cargo install --path .
```

(or)


Cargo installs automatically with the toolchain, but for explicit setup:

```bash
cargo install find_hash
```

Then you can use:

```bash
find_hash e3b98a4da31a127d4bde6e43033f66ba274cab0eb7eb1c70ec41402bf6273cf8
```

Contributions are welcome — feel free to open an issue, submit a pull request or suggest improvements to help this repo grow.

If you find this project useful, please give it a ⭐ to show your support!

