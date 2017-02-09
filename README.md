# badlog

A garish coloured and extremely simple logger. The best kind.

## Installation

```toml
[dependencies]
badlog = "0.1"
log = "0.3"
```

## Usage

```rust
#[macro_use] extern crate log;
extern crate badlog;

fn main() {
    badlog::init("LOG_LEVEL");

    trace!("Unimportant details.");
    debug!("Debugging information.");
    info!("Hello, World!");
    warn!("Uh, captain, I think we're going down...");
    error!("IT'S THE END.");
}
```

## License

MIT.
