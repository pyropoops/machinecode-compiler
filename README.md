# ByteCode Compiler

## Developer

Saif: `saif@hot.fish`

## How to Compile

- Install Rust
- `cd` into this directory
- Run the command `cargo build`

## Usage

- `compiler.exe <input file> <output file>`
- The compiler ignores any character in the input file that is not a `0` or `1`. This means you can format your binary between bytes, such as `0011 0110, 0101`
