// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

#![doc(html_logo_url = "https://gitlab.com/bjoernager/oct/-/raw/master/doc-icon.svg")]

//! Oct is a Rust crate for cheaply serialising (encoding) and deserialising (decoding) data structures to and from binary streams
//!
//! What separates this crate from others such as [Bincode](https://crates.io/crates/bincode/) or [Postcard](https://crates.io/crates/postcard/) is that this crate is extensively optimised for directly translating into binary encodings (whilst the mentioned crates specifically use Serde as a middle layer).
//!
//! The original goal of this project was specifically to guarantee size constraints for encodings on a per-type basis at compile-time.
//! Therefore, this crate may be more suited for networking or other cases where many allocations are unwanted.
//!
//! Keep in mind that this project is still work-in-progress.
//!
//! This crate is compatible with `no_std`.
//!
//! # Performance
//!
//! As Oct is optimised exclusively for a single, binary format, it *may* outperform other libraries that are more generic in nature.
//!
//! The `oct-benchmarks` binary compares multiple scenarios using Oct and other, similar crates.
//! According to my runs on an AMD Ryzen 7 3700X with default settings, these benchmarks indicate that Oct usually outperforms the other tested crates -- as demonstrated in the following table:
//!
//! | Benchmark                          | [Bincode] | [Borsh] | Oct    | [Postcard] |
//! | :--------------------------------- | --------: | ------: | -----: | ---------: |
//! | `encode_u8`                        |    0.977s |  0.871s | 0.754s |     0.916s |
//! | `encode_u32`                       |    0.967s |  0.983s | 0.730s |     2.727s |
//! | `encode_u128`                      |    2.178s |  2.175s | 1.481s |     6.002s |
//! | `encode_struct_unit`               |    0.000s |  0.000s | 0.000s |     0.000s |
//! | `encode_struct_unnamed`            |    1.206s |  1.168s | 0.805s |     2.356s |
//! | `encode_struct_named`              |    3.021s |  1.532s | 0.952s |     3.013s |
//! | `encode_enum_unit`                 |    0.245s |  0.294s | 0.000s |     0.294s |
//! | `decode_u8`                        |    0.952s |  0.895s | 0.885s |     0.894s |
//! | `decode_non_zero_u8`               |    1.215s |  1.250s | 1.229s |     1.232s |
//! | `decode_bool`                      |    1.204s |  1.224s | 1.126s |     1.176s |
//! | **Total time** &#8594;             |   11.964s | 10.392s | 7.963s |    18.609s |
//! | **Total deviation (p.c.)** &#8594; |       +50 |     +31 |     ±0 |       +134 |
//!
//! [Bincode]: https://crates.io/crates/bincode/
//! [Borsh]: https://crates.io/crates/borsh/
//! [Postcard]: https://crates.io/crates/postcard/
//!
//! All quantities are measured in seconds unless otherwise noted.
//!
//! Currently, Oct's weakest point seems to be decoding.
//! Please note that I myself find large (relatively speaking) inconsistencies between runs in these last three benchmarks.
//! Do feel free to conduct your own tests of Oct.
//!
//! # Data model
//!
//! Most primitives encode losslessly, with the main exceptions being [`usize`] and [`isize`].
//! These are instead first cast as [`u16`] and [`i16`], respectively, due to portability concerns (with respect to embedded systems).
//!
//! Numerical primitives in general encode as little endian (and **not** ["network order"](https://en.wikipedia.org/wiki/Endianness#Networking)).
//! It is recommended for implementors to follow this convention as well.
//!
//! See specific types' implementations for notes on their data models.
//!
//! **Note that the data model is currently not stabilised,** and may not necessarily be in the near future (at least before [specialisation](https://github.com/rust-lang/rust/issues/31844/)).
//! It may therefore be undesired to store encodings long-term.
//!
//! # Usage & Examples
//!
//! This crate revolves around the [`Encode`](encode::Encode) and [`Decode`](decode::Decode) traits, both of which handle conversions to and from byte streams.
//!
//! Many standard types come implemented with Oct, including most primitives as well as some standard library types such as [`Option`] and [`Result`].
//! Some [features](#feature-flags) enable an extended set of implementations.
//!
//! It is recommended in most cases to simply derive these two traits for user-defined types, although this is only supported for enumerations and structures -- not untagged unions.
//! When deriving, each field is *chained* according to declaration order:
//!
//! ```rust
//! use oct::decode::Decode;
//! use oct::encode::Encode;
//! use oct::slot::Slot;
//!
//! #[derive(Debug, Decode, Encode, PartialEq)]
//! struct Ints {
//!     value0: u8,
//!     value1: u16,
//!     value2: u32,
//!     value3: u64,
//!     value4: u128,
//! }
//!
//! const VALUE: Ints = Ints {
//!     value0: 0x00,
//!     value1: 0x02_01,
//!     value2: 0x06_05_04_03,
//!     value3: 0x0E_0D_0C_0B_0A_09_08_07,
//!     value4: 0x1E_1D_1C_1B_1A_19_18_17_16_15_14_13_12_11_10_0F,
//! };
//!
//! let mut buf = Slot::with_capacity(0x100);
//!
//! buf.write(VALUE).unwrap();
//!
//! assert_eq!(buf.len(), 0x1F);
//!
//! assert_eq!(
//!     buf,
//!     [
//!         0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
//!         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
//!         0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
//!         0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E
//!     ].as_slice(),
//! );
//!
//! assert_eq!(buf.read().unwrap(), VALUE);
//! ```
//!
//! The following is a more complete example of a UDP server/client for geographic data:
//!
//! ```rust
//! use oct::decode::Decode;
//! use oct::encode::{Encode, SizedEncode};
//! use oct::slot::Slot;
//! use std::io;
//! use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
//! use std::thread::spawn;
//!
//! // City, region, etc.:
//! #[derive(Clone, Copy, Debug, Decode, Encode, Eq, PartialEq, SizedEncode)]
//! #[non_exhaustive]
//! enum Area {
//!     AlQuds,
//!     Byzantion,
//!     Cusco,
//!     Tenochtitlan,
//!     // ...
//! }
//!
//! // Client-to-server message:
//! #[derive(Debug, Decode, Encode, PartialEq, SizedEncode)]
//! #[non_exhaustive]
//! enum Request {
//!     AtmosphericHumidity { area: Area },
//!     AtmosphericPressure { area: Area },
//!     AtmosphericTemperature { area: Area },
//!     // ...
//! }
//!
//! // Server-to-client message:
//! #[derive(Debug, Decode, Encode, PartialEq, SizedEncode)]
//! #[non_exhaustive]
//! enum Response {
//!     AtmosphericHumidity(f64),
//!     AtmosphericPressure(f64), // Pascal
//!     AtmosphericTemperature(f64), // Kelvin
//!     // ...
//! }
//!
//! struct Party {
//!     pub socket: UdpSocket,
//!
//!     pub request_buf:  Slot::<Request>,
//!     pub response_buf: Slot::<Response>,
//! }
//!
//! impl Party {
//!     pub fn new<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
//!         let socket = UdpSocket::bind(addr)?;
//!
//!         let this = Self {
//!             socket,
//!
//!             request_buf:  Slot::new(),
//!             response_buf: Slot::new(),
//!         };
//!
//!         Ok(this)
//!     }
//! }
//!
//! let mut server = Party::new("127.0.0.1:27015").unwrap();
//!
//! let mut client = Party::new("0.0.0.0:0").unwrap();
//!
//! spawn(move || {
//!     let Party { socket, mut request_buf, mut response_buf } = server;
//!
//!     // Recieve initial request from client.
//!
//!     let (len, addr) = socket.recv_from(&mut request_buf).unwrap();
//!     request_buf.set_len(len);
//!
//!     let request = request_buf.read().unwrap();
//!     assert_eq!(request, Request::AtmosphericTemperature { area: Area::AlQuds });
//!
//!     // Handle request and respond back to client.
//!
//!     let response = Response::AtmosphericTemperature(44.4); // For demonstration's sake.
//!
//!     response_buf.write(response).unwrap();
//!     socket.send_to(&response_buf, addr).unwrap();
//! });
//!
//! spawn(move || {
//!     let Party { socket, mut request_buf, mut response_buf } = client;
//!
//!     // Send initial request to server.
//!
//!     socket.connect("127.0.0.1:27015").unwrap();
//!
//!     let request = Request::AtmosphericTemperature { area: Area::AlQuds };
//!
//!     request_buf.write(request);
//!     socket.send(&request_buf).unwrap();
//!
//!     // Recieve final response from server.
//!
//!     socket.recv(&mut response_buf).unwrap();
//!
//!     let response = response_buf.read().unwrap();
//!     assert_eq!(response, Response::AtmosphericTemperature(44.4));
//! });
//! ```
//!
//! # Feature flags
//!
//! Oct defines the following, default features:
//!
//! * `alloc`: Enables the [`Slot`](slot::Slot) type and implementations for types in [`alloc`], e.g. [`Box`](alloc::boxed::Box) and [`Arc`](alloc::sync::Arc)
//! * `proc-macro`: Pulls procedural macros from the [`oct-macros`](https://crates.io/crates/oct-macros/) crate
//! * `std`: Enables implementations for types [`std`], e.g. [`Mutex`](std::sync::Mutex) and [`RwLock`](std::sync::RwLock)
//!
//! # Documentation
//!
//! Oct has its documentation written alongside its source code for use by `rustdoc`.
//! See [Docs.rs](https://docs.rs/oct/latest/oct/) for an on-line, rendered instance.
//!
//! Currently, these docs make use of some unstable features for the sake of readability.
//! The nightly toolchain is therefore required when rendering them.
//!
//! # Contribution
//!
//! Oct does not accept source code contributions at the moment.
//! This is a personal choice by the maintainer and may be undone in the future.
//!
//! Do however feel free to open an issue on [GitLab](https://gitlab.com/bjoernager/oct/issues/) or [GitHub](https://github.com/bjoernager/oct/issues/) if you feel the need to express any concerns over the project.
//!
//! # Copyright & Licence
//!
//! Copyright 2024-2025 Gabriel Bjørnager Jensen.
//!
//! The Source Code Forms of this project are -- where noted as such -- subject to the terms of the Mozilla Public License, v. 2.0.
//! If a copy of the MPL was not distributed with this project, you can obtain one at <https://mozilla.org/MPL/2.0/>.
//!
//! <sub>Note that the `oct-benchmarks` executable is differently released under an MIT licence.</sub>

#![no_std]

#![cfg_attr(feature = "f16",        feature(f16))]
#![cfg_attr(feature = "f128",       feature(f128))]
#![cfg_attr(feature = "never-type", feature(never_type))]

#![cfg_attr(doc, feature(doc_cfg, rustdoc_internals))]

#![warn(missing_docs)]
#![cfg_attr(doc, allow(internal_features))]

// For use in macros:
extern crate self as oct;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

macro_rules! use_mod {
	($vis:vis $name:ident$(,)?) => {
		mod $name;
		$vis use $name::*;
	};
}
pub(crate) use use_mod;

use_mod!(pub primitive_discriminant);

pub mod decode;
pub mod encode;
pub mod error;
pub mod string;
pub mod vec;

#[cfg(feature = "alloc")]
pub mod slot;
