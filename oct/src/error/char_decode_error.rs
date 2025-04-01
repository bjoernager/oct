// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use core::convert::Infallible;
use core::error::Error;
use core::fmt::{self, Display, Formatter};
use core::hint::unreachable_unchecked;

/// A character could not be decoded.
///
/// Unicode defines only the code points inclusively between `U+0000` and `U+D7FFF` as well between `U+E0000` and `U+10FFFF` as being valid.
/// UTF-32 (the format used by the [`char`] data type) additionally specifies that these code points are padded to 32 bits.
///
/// The encoding scheme used by `char` yields an untransformed representation (disregarding endian corrections), but this regrettably also leads to many bit patterns being undefined with respect to UTF-32.
/// If any of these values is read by <code>&lt;char as [Decode](crate::decode::Decode)&gt;::[decode](crate::decode::Decode::decode)</code>, then an instance of this error type is returned.
#[derive(Debug, Eq, PartialEq)]
#[must_use]
pub struct CharDecodeError {
	/// The undefined code point.
	pub code_point: u32,
}

impl Display for CharDecodeError {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "code point U+{:04X} is not defined", self.code_point)
	}
}

impl Error for CharDecodeError { }

impl From<Infallible> for CharDecodeError {
	#[inline(always)]
	fn from(_value: Infallible) -> Self {
		// SAFETY: `Infallible` objects can never be con-
		// structed.
		unsafe { unreachable_unchecked() };
	}
}
