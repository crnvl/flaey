# flaey
 Iterates over all available IPv4 addresses as fast as possible, starting at 1.1.1.1.
 This may take up all of your CPU and memory, so be careful with this.
 Also, there's literally no reason why you'd want to run this.

# Build it yourself
Make sure you have [Rust installed](https://www.rust-lang.org/tools/install)!
When that's done, you can just clone the repository and run
```bash
cargo run
```
in your command prompt.

Or alternatively, you can build your own release version using
```
cargo build --release
```
Output file is in `./target/release/`