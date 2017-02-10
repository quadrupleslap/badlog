# badlog

A garishly colored and extremely simple logger - the best kind.

## Installation

```toml
[dependencies]
badlog = "0.2.3"
log = "0.3"
```

## Documentation

Just read `src/lib.rs`. It's really short, I promise!

## Usage

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
