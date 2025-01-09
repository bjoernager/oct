// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use core::error::Error;
use core::fmt::{self, Display, Formatter};

/// A non-zero integer could not be decoded.
///
/// The implementations of [`Decode`](crate::decode::Decode) for <code>[NonZero](core::num::NonZero)&lt;T&gt;</code> yield this error type if decoding `T` yields zero.
#[derive(Debug, Eq, PartialEq)]
pub struct NonZeroDecodeError;

impl Display for NonZeroDecodeError {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "expected non-zero integer but found `0`")
	}
}

impl Error for NonZeroDecodeError { }
