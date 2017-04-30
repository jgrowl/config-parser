# config-parser


## How to Develop

```bash
curl https://sh.rustup.rs -sSf | sh # install Rust
cargo test # to run tests
```

## References

- [nom macros](http://rust.unhandledexpression.com/nom/index.html)

## Usage example

```bash
cargo run -- --template slapd.conf test-1=yo test-2=yo2 test3=hihi test_4=blah_-test
```

## Thanks

- [nom](https://github.com/Geal/nom) as the parsing library
