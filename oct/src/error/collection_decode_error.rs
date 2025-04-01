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

/// A collection could not be decoded.
///
/// This type is intended as a partially-generic decode error for collections.
/// It supports denoting an error for when the collection's length is invalid -- see the [`BadLength`](Self::BadLength) variant -- and when an element is invalid -- see the [`Item`](Self::BadItem)) variant.
#[derive(Debug, Eq, PartialEq)]
#[must_use]
pub enum CollectionDecodeError<L, I> {
	/// The collection length could not be decoded or was invalid.
	///
	/// For most dynamically-sized collections, the suitable type here is [`Infallible`] due to there basically being no restriction on the collection's size (depending on the data type used for denoting lengths).
	///
	/// Sometimes the length isn't even encoded in the stream (instead lying in the type signature), and in these cases the appropriate type would also be `Infallible`.
	BadLength(L),

	/// A collection item could not be decoded.
	///
	/// Sometimes the index of the item may be desired.
	/// In these cases the [`ItemDecodeError`](crate::error::ItemDecodeError) could be used here.
	BadItem(I),
}

impl<L, I> Display for CollectionDecodeError<L, I>
where
	L: Display,
	I: Display,
{
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match *self {
			Self::BadLength(ref e)
			=> write!(f, "unable to decode collection length: {e}"),

			Self::BadItem(ref e)
			=> write!(f, "unable to decode collection item: {e}"),
		}
	}
}

impl<L, I> Error for CollectionDecodeError<L, I>
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

impl<L, I> From<Infallible> for CollectionDecodeError<L, I> {
	#[inline(always)]
	fn from(_value: Infallible) -> Self {
		unreachable!()
	}
}

impl<L, I> From<CollectionDecodeError<L, I>> for Infallible
where
	L: Into<Self>,
	I: Into<Self>,
{
	#[inline(always)]
	fn from(_value: CollectionDecodeError<L, I>) -> Self {
		unreachable!()
	}
}
