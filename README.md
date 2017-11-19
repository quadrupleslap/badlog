# badlog

[![Cargo](https://img.shields.io/crates/v/badlog.svg)](https://crates.io/crates/badlog)
[![Docs.rs](https://docs.rs/badlog/badge.svg)](https://docs.rs/badlog)

A garishly colored and extremely simple logger - the best kind.

![](preview.png)

## Installation

```toml
[dependencies]
badlog = "1.0"
log = "0.3"
```

## Usage

1. Call [one of the initializers](https://docs.rs/badlog).
2. Use [the standard log crate macros](https://doc.rust-lang.org/log/log/index.html).
3. That's it!

## Example

```rust
#[macro_use] extern crate log;
extern crate badlog;

fn main() {
    badlog::init_from_env("LOG_LEVEL");

    trace!("Unimportant details.");
    debug!("Debugging information.");
    info!("Hello, World!");
    warn!("Uh, captain, I think we're going down...");
    error!("IT'S THE END.");
}
```

## License

MIT.
