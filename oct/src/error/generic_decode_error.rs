// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::{PrimDiscriminant, PrimRepr};
use crate::error::{
	BoolDecodeError,
	CharDecodeError,
	CollectionDecodeError,
	EnumDecodeError,
	ItemDecodeError,
	NonZeroDecodeError,
	LengthError,
	Utf8Error,
};

use core::convert::Infallible;
use core::error::Error;
use core::fmt::{self, Display, Formatter};

#[cfg(feature = "std")]
use crate::error::SystemTimeDecodeError;

/// A generic decoding error type.
///
/// The intended use of this type is by [derived](derive@crate::decode::Decode) implementations of [`crate::decode::Decode`].
/// Manual implementors are recommended to use a custom or less generic type for the sake of efficiency.
#[must_use]
#[non_exhaustive]
#[derive(Debug, Eq, PartialEq)]
pub enum GenericDecodeError {
	/// A character was not a valid UTF-32 unit.
	BadChar(CharDecodeError),

	/// A string contained a non-UTF-8 sequence.
	BadString(Utf8Error),

	#[cfg(feature = "std")]
	#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "std")))]
	/// The [`SystemTime`](std::time::SystemTime) type was too narrow.
	NarrowSystemTime(SystemTimeDecodeError),

	/// A boolean was not boolean.
	NonBool(BoolDecodeError),

	/// A non-zero integer was null.
	NullInteger(NonZeroDecodeError),

	/// A size-constrained buffer was too small.
	SmallBuffer(LengthError),

	/// An unassigned discriminant value was encountered.
	///
	/// The contained value denotes the raw, numerical value of the discriminant.
	UnassignedDiscriminant(PrimDiscriminant),
}

impl Display for GenericDecodeError {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match *self {
			Self::BadChar(ref e)
			=> write!(f, "{e}"),

			Self::BadString(ref e)
			=> write!(f, "{e}"),

			#[cfg(feature = "std")]
			Self::NarrowSystemTime(ref e)
			=> write!(f, "{e}"),

			Self::NonBool(ref e)
			=> write!(f, "{e}"),

			Self::NullInteger(ref e)
			=> write!(f, "{e}"),

			Self::SmallBuffer(ref e)
			=> write!(f, "{e}"),

			Self::UnassignedDiscriminant(value)
			=> write!(f, "discriminant value `{value:#X} has not been assigned"),
		}
	}
}

impl Error for GenericDecodeError {
	#[inline]
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match *self {
			Self::BadChar(ref e) => Some(e),

			Self::BadString(ref e) => Some(e),

			#[cfg(feature = "std")]
			Self::NarrowSystemTime(ref e) => Some(e),

			Self::NonBool(ref e) => Some(e),

			Self::NullInteger(ref e) => Some(e),

			Self::SmallBuffer(ref e) => Some(e),

			_ => None,
		}
	}
}

impl From<BoolDecodeError> for GenericDecodeError {
	#[inline(always)]
	fn from(value: BoolDecodeError) -> Self {
		Self::NonBool(value)
	}
}

impl From<CharDecodeError> for GenericDecodeError {
	#[inline(always)]
	fn from(value: CharDecodeError) -> Self {
		Self::BadChar(value)
	}
}

impl<L, I> From<CollectionDecodeError<L, I>> for GenericDecodeError
where
	L: Into<Self>,
	I: Into<Self>,
{
	#[inline]
	fn from(value: CollectionDecodeError<L, I>) -> Self {
		use CollectionDecodeError as Error;

		match value {
			Error::BadLength(e) => e.into(),

			Error::BadItem(e) => e.into(),
		}
	}
}

impl<T, D, F> From<EnumDecodeError<T, D, F>> for GenericDecodeError
where
	T: PrimRepr,
	D: Into<Self>,
	F: Into<Self>,
{
	#[inline]
	fn from(value: EnumDecodeError<T, D, F>) -> Self {
		use EnumDecodeError as Error;

		match value {
			Error::InvalidDiscriminant(e) => e.into(),

			Error::UnassignedDiscriminant(value) => Self::UnassignedDiscriminant(value.into_prim_discriminant()),

			Error::BadField(e) => e.into(),
		}
	}
}

impl From<Infallible> for GenericDecodeError {
	#[inline(always)]
	fn from(_value: Infallible) -> Self {
		unreachable!()
	}
}

impl<I, E: Into<Self>> From<ItemDecodeError<I, E>> for GenericDecodeError {
	#[inline(always)]
	fn from(value: ItemDecodeError<I, E>) -> Self {
		value.error.into()
	}
}

impl From<NonZeroDecodeError> for GenericDecodeError {
	#[inline(always)]
	fn from(value: NonZeroDecodeError) -> Self {
		Self::NullInteger(value)
	}
}

impl From<LengthError> for GenericDecodeError {
	#[inline(always)]
	fn from(value: LengthError) -> Self {
		Self::SmallBuffer(value)
	}
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "std")))]
impl From<SystemTimeDecodeError> for GenericDecodeError {
	#[inline(always)]
	fn from(value: SystemTimeDecodeError) -> Self {
		Self::NarrowSystemTime(value)
	}
}

impl From<Utf8Error> for GenericDecodeError {
	#[inline(always)]
	fn from(value: Utf8Error) -> Self {
		Self::BadString(value)
	}
}
