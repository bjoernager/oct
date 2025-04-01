// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::error::{
	CollectionEncodeError,
	EnumEncodeError,
	IsizeEncodeError,
	ItemEncodeError,
	UsizeEncodeError,
};

use core::cell::BorrowError;
use core::convert::Infallible;
use core::error::Error;
use core::fmt::{self, Display, Formatter};

/// A generic encoding error type.
///
/// The intended use of this type is by [derived](derive@crate::encode::Encode) implementations of [`crate::encode::Encode`].
/// Manual implementors are recommended to use a custom or less generic type for the sake of efficiency.
#[must_use]
#[non_exhaustive]
#[derive(Debug)]
pub enum GenericEncodeError {
	/// A [`RefCell`](core::cell::RefCell) object could not be borrowed.
	BadBorrow(BorrowError),

	/// An `isize` object was outside the allowed domain.
	LargeIsize(IsizeEncodeError),

	/// A `usize` object was outside the allowed domain.
	LargeUsize(UsizeEncodeError),
}

impl Display for GenericEncodeError {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match *self {
			Self::BadBorrow(ref e)
			=> write!(f, "{e}"),

			Self::LargeIsize(ref e)
			=> write!(f, "{e}"),

			Self::LargeUsize(ref e)
			=> write!(f, "{e}"),
		}
	}
}

impl Error for GenericEncodeError {
	#[inline]
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match *self {
			Self::BadBorrow(ref e) => Some(e),

			Self::LargeIsize(ref e) => Some(e),

			Self::LargeUsize(ref e) => Some(e),
		}
	}
}

impl Eq for GenericEncodeError { }

impl From<BorrowError> for GenericEncodeError {
	#[inline(always)]
	fn from(value: BorrowError) -> Self {
		Self::BadBorrow(value)
	}
}

impl<L, I> From<CollectionEncodeError<L, I>> for GenericEncodeError
where
	L: Into<Self>,
	I: Into<Self>,
{
	#[inline(always)]
	fn from(value: CollectionEncodeError<L, I>) -> Self {
		use CollectionEncodeError as Error;

		match value {
			Error::BadLength(e) => e.into(),

			Error::BadItem(e) => e.into(),
		}
	}
}

impl<D, F> From<EnumEncodeError<D, F>> for GenericEncodeError
where
	D: Into<Self>,
	F: Into<Self>,
{
	#[inline(always)]
	fn from(value: EnumEncodeError<D, F>) -> Self {
		use EnumEncodeError as Error;

		match value {
			Error::BadDiscriminant(e) => e.into(),

			Error::BadField(e) => e.into(),
		}
	}
}

impl From<Infallible> for GenericEncodeError {
	#[inline(always)]
	fn from(_value: Infallible) -> Self {
		unreachable!()
	}
}

impl From<IsizeEncodeError> for GenericEncodeError {
	#[inline(always)]
	fn from(value: IsizeEncodeError) -> Self {
		Self::LargeIsize(value)
	}
}

impl<I, E: Into<Self>> From<ItemEncodeError<I, E>> for GenericEncodeError {
	#[inline(always)]
	fn from(value: ItemEncodeError<I, E>) -> Self {
		value.error.into()
	}
}

impl From<UsizeEncodeError> for GenericEncodeError {
	#[inline(always)]
	fn from(value: UsizeEncodeError) -> Self {
		Self::LargeUsize(value)
	}
}

impl PartialEq for GenericEncodeError {
	#[inline]
	fn eq(&self, other: &Self) -> bool {
		match *self {
			Self::BadBorrow(..) => {
				matches!(*other, Self::BadBorrow(..))
			}

			Self::LargeIsize(ref lvalue) => {
				matches!(*other, Self::LargeIsize(ref rvalue) if *rvalue == *lvalue)
			}

			Self::LargeUsize(ref lvalue) => {
				matches!(*other, Self::LargeUsize(ref rvalue) if *rvalue == *lvalue)
			}
		}
	}
}
