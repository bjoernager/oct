# Oct

Oct is a Rust crate for cheaply serialising (encoding) and deserialising (decoding) data structures to and from binary streams

What separates this crate from others such as [Postcard](https://crates.io/crates/postcard/) is that this crate is extensively optimised for directly translating into binary encodings (whilst the mentioned crate specifically use Serde as a middle layer).

The original goal of this project was specifically to guarantee size constraints for encodings on a per-type basis at compile-time.
Therefore, this crate may be more suited for networking or other cases where many allocations are unwanted.

Keep in mind that this project is still work-in-progress.

This crate is compatible with `no_std`.

## Performance

As Oct is optimised exclusively for a single, binary format, it *may* outperform other libraries that are more generic in nature.

The `oct-benchmarks` binary compares multiple scenarios using Oct and other, similar crates.
According to my runs, these benchmarks indicate that Oct usually outperforms the other tested crates -- as demonstrated in the following table:

| Benchmark                          | Oct    | [Bincode] | [Borsh] | [Postcard] |
| :--------------------------------- | -----: | --------: | ------: | ---------: |
| `encode_u8`                        | 100.00 |    102.88 |  102.73 |     102.63 |
| `encode_u16`                       | 100.00 |    110.67 |   95.62 |     204.46 |
| `encode_u32`                       | 100.00 |    171.04 |  111.23 |     257.03 |
| `encode_u64`                       | 100.00 |    171.14 |  116.93 |     379.45 |
| `encode_u128`                      | 100.00 |    170.11 |  118.74 |     361.56 |
| `encode_bool`                      | 100.00 |     94.86 |  101.49 |     100.03 |
| `encode_unit_struct`               | 100.00 |     99.87 |   99.85 |      99.96 |
| `encode_newtype`                   | 100.00 |    202.31 |  109.80 |     243.34 |
| `encode_struct`                    | 100.00 |     50.19 |   50.09 |     118.93 |
| `encode_enum`                      | 100.00 |    107.57 |   84.80 |     113.98 |
| `decode_u8`                        | 100.00 |      5.65 |    5.83 |       5.52 |
| `decode_u16`                       | 100.00 |    706.27 |  135.52 |     671.54 |
| `decode_u32`                       | 100.00 |    651.28 |  105.80 |     410.24 |
| `decode_u64`                       | 100.00 |    697.40 |  141.31 |    1549.56 |
| `decode_u128`                      | 100.00 |    529.90 |  117.32 |    1425.68 |
| `decode_bool`                      | 100.00 |     74.59 |   80.24 |      74.55 |
| `decode_unit_struct`               | 100.00 |     76.05 |  115.57 |      65.87 |
| `decode_newtype`                   | 100.00 |    859.92 |  105.83 |     247.19 |
| `decode_struct`                    | 100.00 |     28.59 |   28.60 |      28.35 |

[Bincode]: https://crates.io/crates/bincode/
[Borsh]: https://crates.io/crates/borsh/
[Postcard]: https://crates.io/crates/postcard/

... wherein quantities denote indicies (with `100` being the reference).
Lower is better.

Feedback is greatly appreciated on the mechanics of these benchmarks.
Do also feel free to conduct your own tests of Oct.

## Data model

Primitives encode losslessly by default, although `usize` and `isize` are the exception to this.
Due to their machine-dependent representation, these are truncated to the smallest subset of values guaranteed by Rust, with this equating to a cast to `u16` or `i16`, respectively.

Numerical types in general (including `char`) are encoded as little endian (and **not** ["network order"](https://en.wikipedia.org/wiki/Endianness#Networking) as is the norm in TCP/UDP/IP).
It is recommended for implementors of custom types to adhere to this convention as well.

See specific types' implementations for notes on their data models.

**Note that not all data models may be stabilised at the current moment.**
It may therefore be undesired to store encodings long-term.

## Usage & Examples

This crate revolves around the `Encode` and `Decode` traits, both of which handle conversions to and from byte streams.

These traits are already implemented by Oct for a large set of the standard types, such as `Option` and `Mutex`.
Some [features](#feature-flags) enable an extended set of implementations that are locked behind unstable feature gates or other crates.

The following is an example of a UDP server/client for geographic data:

```rust
use oct::decode::Decode;
use oct::encode::{Encode, SizedEncode};
use oct::slot::Slot;
use std::io;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::thread::spawn;

// City, region, etc.:
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Decode, Encode, Eq, PartialEq, SizedEncode)]
enum Area {
    AlQuds,
    Byzantion,
    Cusco,
    Tenochtitlan,
    // ...
}

// Client-to-server message:
#[non_exhaustive]
#[derive(Debug, Decode, Encode, PartialEq, SizedEncode)]
enum Request {
    AtmosphericHumidity { area: Area },
    AtmosphericPressure { area: Area },
    AtmosphericTemperature { area: Area },
    // ...
}

// Server-to-client message:
#[non_exhaustive]
#[derive(Debug, Decode, Encode, PartialEq, SizedEncode)]
enum Response {
    AtmosphericHumidity(f64),
    AtmosphericPressure(f64), // Pascal
    AtmosphericTemperature(f64), // Kelvin
    // ...
}

struct Party {
    pub socket: UdpSocket,

    pub request_buf:  Slot<Request>,
    pub response_buf: Slot<Response>,
}

impl Party {
    pub fn new<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let socket = UdpSocket::bind(addr)?;

        let this = Self {
            socket,

            request_buf:  Slot::new(),
            response_buf: Slot::new(),
        };

        Ok(this)
    }
}

let mut server = Party::new("127.0.0.1:27015").unwrap();

let mut client = Party::new("0.0.0.0:0").unwrap();

spawn(move || {
    let Party { socket, mut request_buf, mut response_buf } = server;

    // Recieve initial request from client.

    let (len, addr) = socket.recv_from(&mut request_buf).unwrap();
    request_buf.set_len(len);

    let request = request_buf.read().unwrap();
    assert_eq!(request, Request::AtmosphericTemperature { area: Area::AlQuds });

    // Handle request and respond back to client.

    let response = Response::AtmosphericTemperature(44.4); // For demonstration's sake.

    response_buf.write(response).unwrap();
    socket.send_to(&response_buf, addr).unwrap();
});

spawn(move || {
    let Party { socket, mut request_buf, mut response_buf } = client;

    // Send initial request to server.

    socket.connect("127.0.0.1:27015").unwrap();

    let request = Request::AtmosphericTemperature { area: Area::AlQuds };

    request_buf.write(request);
    socket.send(&request_buf).unwrap();

    // Recieve final response from server.

    socket.recv(&mut response_buf).unwrap();

    let response = response_buf.read().unwrap();
    assert_eq!(response, Response::AtmosphericTemperature(44.4));
});
```

## Feature flags

Oct defines the following, default features:

* `alloc`: Enables the `Slot` type and implementations for types in `alloc`, e.g. `Box` and `Arc`
* `proc-macro`: Pulls procedural macros from the [`oct-macros`](https://crates.io/crates/oct-macros/) crate
* `std`: Enables implementations for types `std`, e.g. `Mutex` and `RwLock`

The following features can additionally be enabled for support with nightly-only constructs:

* `f128`: Enable implementations for the `f128` type
* `f16`: Enable implementations for the `f16` type

## Documentation

Oct has its documentation written alongside its source code for use by `rustdoc`.
See [Docs.rs](https://docs.rs/oct/latest/oct/) for an on-line, rendered instance.

Currently, these docs make use of some unstable features for the sake of readability.
The nightly toolchain is therefore always required when rendering them or or running tests herein.

## Contribution

Oct does not accept source code contributions at the moment.
This is a personal choice by the maintainer and may be undone in the future.

Do however feel free to open an issue on [GitLab](https://gitlab.com/bjoernager/oct/issues/), on [GitHub](https://github.com/bjoernager/oct/issues/), or on [`mandelbrot.dk`](https://mandelbrot.dk/bjoernager/oct/issues/) (if a member) if you feel the need to express any concerns over the project.

## Copyright & Licence

Copyright 2024-2025 Gabriel Bjørnager Jensen.

The Source Code Forms of this project are &ndash; where noted as such &ndash; subject to the terms of the Mozilla Public License, v. 2.0.
If a copy of the MPL was not distributed with this project, you can obtain one at <https://mozilla.org/MPL/2.0/>.

<sub>Note that the `oct-benchmarks` executable is differently released under an MIT licence.</sub>
