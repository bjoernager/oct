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

use crate::use_mod;

use_mod!(pub decode);
use_mod!(pub decode_borrowed);
use_mod!(pub input);

/// Implements [`Decode`] for the provided type.
///
/// This macro assumes the same format used by the equivalent [`Encode`](derive@crate::encode::Encode) macro.
#[cfg(feature = "proc-macro")]
#[cfg_attr(doc, doc(cfg(feature = "proc-macro")))]
#[doc(inline)]
pub use oct_macros::Decode;
