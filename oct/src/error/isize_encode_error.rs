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

/// An [`isize`] value could not be decoded.
///
/// Any `isize` object that can fit in an [`i16`] can be encoded successfully.
#[derive(Debug, Eq, PartialEq)]
#[must_use]
pub struct IsizeEncodeError(
	/// The unencodable value.
	pub isize,
);

impl Display for IsizeEncodeError {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(
			f,
			"signed size value ({}) cannot be serialised: must be in the range ({}) to ({})",
			self.0,
			i16::MIN,
			i16::MAX,
		)
	}
}

impl Error for IsizeEncodeError { }

impl From<Infallible> for IsizeEncodeError {
	#[inline(always)]
	fn from(_value: Infallible) -> Self {
		// SAFETY: `Infallible` objects can never be con-
		// structed.
		unsafe { unreachable_unchecked() };
	}
}
