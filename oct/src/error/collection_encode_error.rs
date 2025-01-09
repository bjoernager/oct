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

/// A collection could not be encoded.
///
/// This type is intended as a partially-generic encode error for collections.
/// It supports denoting an error for when the collection's length is invalid -- see the [`BadLength`](Self::BadLength) variant -- and when an element is invalid -- see the [`Item`](Self::BadItem)) variant.
#[derive(Debug)]
#[must_use]
pub enum CollectionEncodeError<L, I> {
	/// The collection length could not be encoded.
	BadLength(L),

	/// A collection item could not be encoded.
	BadItem(I),
}

impl<L, I> Display for CollectionEncodeError<L, I>
where
	L: Display,
	I: Display,
{
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match *self {
			Self::BadLength(ref e)
			=> write!(f, "unable to encode collection length: {e}"),

			Self::BadItem(ref e)
			=> write!(f, "unable to encode collection item: {e}"),
		}
	}
}

impl<L, I> Error for CollectionEncodeError<L, I>
where
	L: Error + 'static,
	I: Error + 'static,
{
	#[inline]
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match *self {
			Self::BadLength(ref e) => Some(e),

			Self::BadItem(ref e) => Some(e),
		}
	}
}

impl<L, I> From<CollectionEncodeError<L, I>> for Infallible
where
	L: Into<Self>,
	I: Into<Self>,
{
	#[inline(always)]
	fn from(_value: CollectionEncodeError<L, I>) -> Self {
		unreachable!()
	}
}
