// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

//! Decoding-related facilities.
//!
//! To decode a byte array using the [`Decode`], simply call the [`decode`](decode::Decode::decode) method with an [`Input`] object:
//!
//! ```rust
//! use oct::decode::{Decode, Input};
//!
//! let data = [0x54, 0x45];
//! let mut stream = Input::new(&data);
//!
//! assert_eq!(u16::decode(&mut stream).unwrap(), 0x4554);
//!
//! // Data can theoretically be reinterpretred:
//!
//! stream = Input::new(&data);
//!
//! assert_eq!(u8::decode(&mut stream).unwrap(), 0x54);
//! assert_eq!(u8::decode(&mut stream).unwrap(), 0x45);
//!
//! // Including as tuples:
//!
//! stream = Input::new(&data);
//!
//! assert_eq!(<(u8, u8)>::decode(&mut stream).unwrap(), (0x54, 0x45));
//! ```

mod decode;
mod decode_borrowed;
mod input;

pub use decode::Decode;
pub use decode_borrowed::DecodeBorrowed;
pub use input::Input;

/// Implements [`Decode`] for the provided type.
///
/// This derive macro assumes the same format used by the equivalent [`Encode`](derive@crate::encode::Encode) derive macro.
///
/// By default, this macro assumes that all fields implement <code>Decode&lt;[Error]: [Into]&lt;[GenericDecodeError]&gt;&gt;</code>.
/// If this is **not** the case, then the `#[oct(decode_error = T)` attribute should be used for the desired error `T` on the deriving type.
///
/// [Error]: Encode::Error
/// [GenericDecodeError]: crate::error::GenericDecodeError
#[cfg(feature = "proc-macro")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "proc-macro")))]
#[doc(inline)]
pub use oct_macros::Decode;
