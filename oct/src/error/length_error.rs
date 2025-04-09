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

/// A collection buffer was too small to contain all of its elements.
///
/// Some data types use a statically-sized buffer whilst still allowing for partial usage of this buffer.
/// These types should return this error in cases where their size limit has exceeded.
#[derive(Debug, Eq, PartialEq)]
#[must_use]
pub struct LengthError {
	/// The remaining capacity of the buffer.
	pub remaining: usize,

	/// The required amount of elements.
	pub count: usize,
}

impl Display for LengthError {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "collection with `{}` remaining size cannot hold `{}` more elements", self.remaining, self.count)
	}
}

impl Error for LengthError { }

impl From<Infallible> for LengthError {
	#[inline(always)]
	fn from(_value: Infallible) -> Self {
		unreachable!()
	}
}
