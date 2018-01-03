# Quick brainfuck interpreter in Rust
[→ See simple implementation][simple]

**Note:** this is currently a work in progress

---

A quick [brainfuck][brainfuck] interpreter implemented in the [Rust][rust] language.

This implementation is focussed on being as quick as possible.
This is my approach of building a quick interpreter, I'm sure there are quicker
interpreters out there.

## Usage
For installation, Git and Rust cargo are required.
Install the latest version of Rust with [rustup][rustup].

Create a `program.bf` file, which this application reads the program from.

```bash
# Clone the project
git clone https://github.com/timvisee/brainfuck-rs-quick.git
cd brainfuck-rs-simple

# Run
cargo run --release

# Test
cargo test
```

## License
This project is released under the GNU GPL-3.0 license.
Check out the [LICENSE](LICENSE) file for more information.


[rust]: https://rust-lang.org/
[rustup]: https://rustup.rs/
[brainfuck]: https://en.wikipedia.org/wiki/Brainfuck
[simple]: https://github.com/timvisee/brainfuck-rs-simple
