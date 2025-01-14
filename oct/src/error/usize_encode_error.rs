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

/// A [`usize`] value could not be decoded.
///
/// Any `usize` object that can fit in an [`u16`] can be encoded successfully.
#[derive(Debug, Eq, PartialEq)]
#[must_use]
pub struct UsizeEncodeError(
	/// The unencodable value.
	pub usize,
);

impl Display for UsizeEncodeError {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(
			f,
			"unsigned size value ({}) cannot be serialised: must be at most ({})",
			self.0,
			u16::MAX,
		)
	}
}

impl Error for UsizeEncodeError { }

impl From<Infallible> for UsizeEncodeError {
	#[inline(always)]
	fn from(_value: Infallible) -> Self {
		// SAFETY: `Infallible` objects can never be con-
		// structed.
		unsafe { unreachable_unchecked() };
	}
}
