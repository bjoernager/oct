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

/// An input-related error.
///
/// This structure is mainly returned by the [`read`](crate::decode::Input::read) and [`read_into`](crate::decode::Input::read_into) methods in [`decode::Input`](crate::decode::Input).
#[derive(Debug, Eq, PartialEq)]
#[must_use]
pub struct InputError {
	/// The total capacity of the output stream.
	pub capacity: usize,

	/// The cursor position of the requested read.
	pub position: usize,

	/// The requested amount of octets.
	pub count: usize,
}

impl Display for InputError {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(
			f,
			"cannot read ({}) bytes at ({}) from input stream with capacity of ({})",
			self.count,
			self.position,
			self.capacity,
		)
	}
}

impl Error for InputError { }

impl From<Infallible> for InputError {
	#[inline(always)]
	fn from(_value: Infallible) -> Self {
		unreachable!()
	}
}
