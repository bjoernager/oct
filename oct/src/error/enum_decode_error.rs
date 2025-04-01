// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::decode::Decode;

use core::convert::Infallible;
use core::error::Error;
use core::fmt::{self, Debug, Display, Formatter};

/// An enumeration could not be decoded.
#[derive(Debug, Eq, PartialEq)]
#[must_use]
pub enum EnumDecodeError<T, D, F> {
	/// The discriminant could not be decoded.
	InvalidDiscriminant(D),

	/// An otherwise valid discriminant has not been assigned.
	///
	/// Remember that this error does **not** indicate that the discriminant couldn't be decoded, merely that it does not match with that of any variant.
	/// See also [`InvalidDiscriminant`](Self::InvalidDiscriminant).
	UnassignedDiscriminant(T),

	/// A field could not be encoded.
	BadField(F),
}

impl<T, D, F> Display for EnumDecodeError<T, D, F>
where
	T: Display,
	D: Display,
	F: Display,
{
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match *self {
			Self::InvalidDiscriminant(ref e)
			=> write!(f, "discriminant could not be decoded: {e}"),

			Self::UnassignedDiscriminant(ref value)
			=> write!(f, "`{value}` is not an assigned discriminant for the given enumeration"),

			Self::BadField(ref e)
			=> write!(f, "variant could not be decoded: {e}"),
		}
	}
}

impl<T, D, F> Error for EnumDecodeError<T, D, F>
where
	T: Debug + Display,
	D: Error + 'static,
	F: Error + 'static,
{
	#[inline]
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match *self {
			Self::InvalidDiscriminant(ref e) => Some(e),

			Self::BadField(ref e) => Some(e),

			_ => None,
		}
	}
}

impl<T, D, F> From<Infallible> for EnumDecodeError<T, D, F> {
	#[inline(always)]
	fn from(_value: Infallible) -> Self {
		unreachable!()
	}
}

impl<T, D, F> From<EnumDecodeError<T, D, F>> for Infallible
where
	T: Decode<Error: Into<Self>>,
	F: Into<Self>,
{
	#[inline(always)]
	fn from(_value: EnumDecodeError<T, D, F>) -> Self {
		unreachable!()
	}
}
