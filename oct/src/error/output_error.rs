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

#[derive(Debug, Eq, PartialEq)]
#[must_use]
/// An output-related error
///
/// This structure is mainly returned by the [`write`](crate::encode::Output::write) method in [`encode::Output`](crate::encode::Output).
pub struct OutputError {
	/// The total capacity of the output stream.
	pub capacity: usize,

	/// The cursor position of the requested write.
	pub position: usize,

	/// The requested amount of octets.
	pub count: usize,
}

impl Display for OutputError {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(
			f,
			"cannot write ({}) bytes at ({}) to output stream with capacity of ({})",
			self.count,
			self.position,
			self.capacity,
		)
	}
}

impl Error for OutputError { }

impl From<Infallible> for OutputError {
	#[inline(always)]
	fn from(_value: Infallible) -> Self {
		// SAFETY: `Infallible` objects can never be con-
		// structed.
		unsafe { unreachable_unchecked() };
	}
}
