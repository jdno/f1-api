# F1 API

_A Rust implementation of the telemetry API provided by modern F1 video games._

This project implements a client library for the telemetry data that is
published by the current generation of [F1 video games][f1] by [Codemasters].
The library is written in Rust, using [tokio] for async networking.

## Examples

The `examples` folder contains examples that show how to use this library. They
can also be helpful for local development, in particular the `cli` example. It
uses the library to listen to telemetry packages from an F1 game, and prints
them on the terminal. More information how to use the CLI can be gathered from
the following command:

    cargo run --example cli -- --help

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[codemasters]: https://www.codemasters.com/
[f1]: https://www.codemasters.com/game/f1-2019/
[tokio]: https://tokio.rs/
