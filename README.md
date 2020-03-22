# F1 API

[![rust](https://github.com/hellobits/f1-api/workflows/rust/badge.svg)](https://github.com/hellobits/f1-api/actions)

_A Rust implementation of the telemetry APIs of modern F1 video games._

This project implements a client library for the telemetry APIs that are
provided by the current generation of [F1 video games][f1] by [Codemasters].
The library is written in Rust, using [tokio] for async networking.

_Built with ❤️ and 🦀 by [Hello Bits](https://github.com/hellobits)._

## Getting Started

Most of the library deals with the low-level details of receiving and decoding
the packets that the F1 games sent. The only piece users need to interact with
is the `F1` struct and its high-level interface.

```rust
use f1_api::F1;
use f1_api::packet::Packet::{
    Event, Lap, Motion, Participants, Session, Setup, Status, Telemetry
};
use std::net::{IpAddr, SocketAddr};
use tokio::stream::StreamExt;

#[tokio::main]
async fn main() {
    let ip_address = IpAddr::from([0, 0, 0, 0]);
    let port = 20777;
    let socket = SocketAddr::new(ip_address, port);

    let mut stream = F1::stream(socket).unwrap();

    while let Some(packet) = stream.next().await {
        match packet {
            Event(_) => println!("Received an Event packet"),
            Lap(_) => println!("Received a Lap packet"),
            Motion(_) => println!("Received a Motion packet"),
            Participants(_) => println!("Received a Participants packet"),
            Session(_) => println!("Received a Session packet"),
            Setup(_) => println!("Received aaSetup packet"),
            Status(_) => println!("Received a Status packet"),
            Telemetry(_) => println!("Received a Telemetry packet"),
        }
    }
}
```

`F1::stream` is an asynchronous function that returns a stream of incoming
packets, and the recommended way to interface with the `f1-api` crate.

## Examples

The `examples` folder contains examples that show how to use this library. For
example, the code in the [Getting Started](#getting-started) section can be run
with the following command:

```shell script
cargo run --example readme
```

A more complex example is the `cli`, which uses the library to analyse incoming
packets and print interesting information about the state of the game to the
terminal. It can be run with:

```shell script
cargo run --example cli
```

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
