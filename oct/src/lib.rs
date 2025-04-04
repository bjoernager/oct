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
//! | `encode_u8`                        |     1.004 |   0.880 |  0.812 |      0.860 |
//! | `encode_u32`                       |     1.312 |   0.973 |  0.818 |      2.706 |
//! | `encode_u128`                      |     2.631 |   2.174 |  1.561 |      6.159 |
//! | `encode_char`                      |     1.598 |   1.256 |  0.830 |      2.393 |
//! | `encode_struct_unit`               |     0.000 |   0.000 |  0.000 |      0.000 |
//! | `encode_struct_unnamed`            |     1.452 |   1.238 |  0.815 |      2.296 |
//! | `encode_struct_named`              |     1.464 |   1.471 |  1.105 |      3.026 |
//! | `encode_enum_unit`                 |     0.290 |   0.290 |  0.000 |      0.288 |
//! | `decode_u8`                        |     0.975 |   1.018 |  0.987 |      0.974 |
//! | `decode_non_zero_u8`               |     1.192 |   1.292 |  1.268 |      1.286 |
//! | `decode_bool`                      |     1.037 |   1.099 |  1.041 |      1.101 |
//! | **Total time** &#8594;             |    12.957 |  11.690 |  9.238 |     21.091 |
//! | **Total deviation (p.c.)** &#8594; |       +40 |     +27 |     ±0 |       +128 |
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
//! Primitives encode losslessly by default, although [`usize`] and [`isize`] are the exception to this.
//! Due to their machine-dependent representation, these are truncated to the smallest subset of values guaranteed by Rust, with this equating to a cast to [`u16`] or [`i16`], respectively.
//!
//! Numerical types in general (including `char`) are encoded as little endian (and **not** ["network order"](https://en.wikipedia.org/wiki/Endianness#Networking) as is the norm in TCP/UDP/IP).
//! It is recommended for implementors of custom types to adhere to this convention as well.
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
//! These traits are already implemented by Oct for a large set of the standard types, such as [`Option`] and [`Mutex`](std::sync::Mutex).
//! Some [features](#feature-flags) enable an extended set of implementations that are locked behind unstable feature gates or other crates.
//!
//! The following is an example of a UDP server/client for geographic data:
//!
//! ```rust
//! # #[cfg(not(miri))]
//! # {
//! use oct::decode::Decode;
//! use oct::encode::{Encode, SizedEncode};
//! use oct::slot::Slot;
//! use std::io;
//! use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
//! use std::thread::spawn;
//!
//! // City, region, etc.:
//! #[non_exhaustive]
//! #[derive(Clone, Copy, Debug, Decode, Encode, Eq, PartialEq, SizedEncode)]
//! enum Area {
//!     AlQuds,
//!     Byzantion,
//!     Cusco,
//!     Tenochtitlan,
//!     // ...
//! }
//!
//! // Client-to-server message:
//! #[non_exhaustive]
//! #[derive(Debug, Decode, Encode, PartialEq, SizedEncode)]
//! enum Request {
//!     AtmosphericHumidity { area: Area },
//!     AtmosphericPressure { area: Area },
//!     AtmosphericTemperature { area: Area },
//!     // ...
//! }
//!
//! // Server-to-client message:
//! #[non_exhaustive]
//! #[derive(Debug, Decode, Encode, PartialEq, SizedEncode)]
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
//!     pub request_buf:  Slot<Request>,
//!     pub response_buf: Slot<Response>,
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
//!     response_buf.write(&response).unwrap();
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
//!     request_buf.write(&request);
//!     socket.send(&request_buf).unwrap();
//!
//!     // Recieve final response from server.
//!
//!     socket.recv(&mut response_buf).unwrap();
//!
//!     let response = response_buf.read().unwrap();
//!     assert_eq!(response, Response::AtmosphericTemperature(44.4));
//! });
//!
//! # }
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
//! The following features can additionally be enabled for support with nightly-only constructs:
//!
//! * `f128`: Enable implementations for the [`f128`] type
//! * `f16`: Enable implementations for the [`f16`] type
//!
//! # Documentation
//!
//! Oct has its documentation written alongside its source code for use by `rustdoc`.
//! See [Docs.rs](https://docs.rs/oct/latest/oct/) for an on-line, rendered instance.
//!
//! Currently, these docs make use of some unstable features for the sake of readability.
//! The nightly toolchain is therefore always required when rendering them or or running tests herein.
//!
//! # Contribution
//!
//! Oct does not accept source code contributions at the moment.
//! This is a personal choice by the maintainer and may be undone in the future.
//!
//! Do however feel free to open an issue on [GitLab](https://gitlab.com/bjoernager/oct/issues/), on [GitHub](https://github.com/bjoernager/oct/issues/), or on [`mandelbrot.dk`](https://mandelbrot.dk/bjoernager/oct/issues/) (if a member) if you feel the need to express any concerns over the project.
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

#![cfg_attr(feature = "f128", feature(f128))]
#![cfg_attr(feature = "f16",  feature(f16))]

#![cfg_attr(doc, feature(doc_cfg, rustdoc_internals))]

#![warn(missing_docs)]
#![cfg_attr(doc, allow(internal_features))]

// For use in macros:
extern crate self as oct;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod prim_discriminant;
mod prim_repr;

pub mod decode;
pub mod encode;
pub mod error;

#[cfg(feature = "alloc")]
pub mod slot;

pub use prim_discriminant::PrimDiscriminant;
pub use prim_repr::PrimRepr;

pub(crate) use prim_repr::SealedPrimRepr;

#[doc(hidden)]
#[macro_export]
macro_rules! enum_encoded_size {
	($(($($tys:ty),+$(,)?)),*$(,)?) => {
		const {
			#[allow(unused)]
			use ::oct::encode::SizedEncode;

			let mut total_size = 0x0usize;

			$({
				let current_size = 0x0usize $(+ <$tys as SizedEncode>::MAX_ENCODED_SIZE)*;

				if current_size > total_size {
					total_size = current_size;
				}
			})*

			total_size
		}
	};
}

// NOTE: Stable equivalent of `core::hint::
// cold_path`. Should not be used directly, but is
// used in derive macros.
#[doc(hidden)]
#[cold]
pub const fn __cold_path() { }
