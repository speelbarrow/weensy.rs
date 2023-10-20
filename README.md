# weensy.rs

A collection of useful bits and bobs for programming the 
[Teensy 4.1 microcontroller](https://www.pjrc.com/store/teensy41.html) using [Rust](https://rust-lang.org).

**WARNING**:
This crate is mostly for exploration/fun, and probably shouldn't be used in a production environment. It is **NOT**
tested, and won't be published to `crates.io` or semantically versioned for that reason. Instead, please use the 
following format in your crate's `Cargo.toml` manifest to add this package to your crate:
```toml
[dependencies]
# ...
weensy = { git = "https://github.com/speelbarrow/weensy.rs" }
# ...
```

## USAGE
This package should be used by an executable crate targetting the Teensy 4.1. The following configuration of `cargo` 
is required to build your executable properly:
```toml
[build]
target = "thumbv7em-none-eabihf"

[target.thumbv7em-none-eabihf]
rustflags = ["-C", "link-arg=-Tt4link.x"]
```
This code should be placed in `<CRATE_ROOT>/.cargo/config.toml`. It can also be passed directly to the `cargo` command
using CLI flags, but this is not recommended because it's clunky.

See [`cargo`'s documentation](https://doc.rust-lang.org/cargo/reference/config.html) for more details. The `t4link.x`
file will be generated by [`teensy4-bsp`](https://docs.rs/teensy4-bsp/latest/teensy4_bsp/)'s
[`build.rs`](https://github.com/mciantyre/teensy4-rs/blob/master/build.rs) file. `teensy4-bsp` is included by this
package, and thus the build script will run when building any crate importing this package. All feature flags for this package's dependencies are re-exported.

Once this is done, `cargo install` [`cargo-binutils`](https://github.com/mciantyre/teensy4-rs/blob/master/build.rs) and
then run the following in the root directory of your executable crate:
```sh
cargo objcopy --release -- -o ihex out.hex
```

You can then flash `out.hex` to your Teensy board using the
[Teensy's provided tools](https://www.pjrc.com/teensy/loader.html).
