# Quick brainfuck interpreter in Rust
[→ See simple implementation][simple]

**Note:** the base implementation is already working.
The optimization procedure is still a work in progress though.

---

A quick [brainfuck][brainfuck] interpreter implemented in the [Rust][rust] language.

This implementation is focussed on being as quick as possible.
This is my approach of building a quick interpreter, I'm sure there are quicker
interpreters out there.

## Usage
For installation, Git and Rust cargo are required.
Install the latest version of Rust with [rustup][rustup].

```bash
# Clone the project
git clone https://github.com/timvisee/brainfuck-rs-quick.git
cd brainfuck-rs-simple

# View help
cargo run --release -- --help

# Run a program
cargo run --release -- programs/hello_world.b

# Test
cargo test
```

## Help
```
brainfuck-rs-quick --help

brainfuck-rs-quick 0.1
Tim Visée <timvisee@gmail.com>
A quick brainfuck interpreter in Rust.

USAGE:
    brainfuck-rs-quick <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <FILE>    Brainfuck file to interpret
```

## License
This project is released under the GNU GPL-3.0 license.
Check out the [LICENSE](LICENSE) file for more information.


[rust]: https://rust-lang.org/
[rustup]: https://rustup.rs/
[brainfuck]: https://en.wikipedia.org/wiki/Brainfuck
[simple]: https://github.com/timvisee/brainfuck-rs-simple
