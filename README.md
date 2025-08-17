# azathoth_logger

A Logging crate for `no-std` environments used by the [`AzathothC2` framework](https://github.com/AzathothC2)

> [!WARNING]
> This crate is not the same as [`az_logger`](https://crates.io/crates/az_logger), although they do export the same macros!

## Usage
Using the logger requires initializing the static logger:

```rust
fn main() {
    if unsafe { !azathoth_logger::LOG.init() } {
        return; // Failed
    }
    azathoth_logger::success!("Hello world!");
}
```

## Installation
Add the crate via Cargo:
```cargo add azathoth_libload```

Or manually in `Cargo.toml`: ```azathoth_libload = "0.1.0";```

## License
MIT