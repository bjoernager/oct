// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use core::convert::Infallible;
use core::error::Error;
use core::fmt::{self, Debug, Display, Formatter};
use core::hint::unreachable_unchecked;

/// A collection's item could not be decoded.
///
/// See also [`CollectionDecodeError`](crate::error::CollectionDecodeError).
#[derive(Debug, Eq, PartialEq)]
#[must_use]
pub struct ItemDecodeError<I, E> {
	/// The index of the invalid item.
	pub index: I,

	/// The decoder's error.
	pub error: E,
}

impl<I, E> Display for ItemDecodeError<I, E>
where
	I: Display,
	E: Display,
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "could not decode item at `{}`: {}", self.index, self.error)
	}
}

impl<I, E> Error for ItemDecodeError<I, E>
where
	Self: Debug + Display,
	E: Error + 'static,
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		Some(&self.error)
	}
}

impl<I, E> From<Infallible> for ItemDecodeError<I, E> {
	#[inline(always)]
	fn from(_value: Infallible) -> Self {
		// SAFETY: `Infallible` objects can never be con-
		// structed.
		unsafe { unreachable_unchecked() };
	}
}

impl<I, E> From<ItemDecodeError<I, E>> for Infallible {
	#[inline(always)]
	fn from(_value: ItemDecodeError<I, E>) -> Self {
		unreachable!()
	}
}
