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

/// An enumeration could not be encoded.
#[derive(Debug, Eq, PartialEq)]
#[must_use]
pub enum EnumEncodeError<D, F> {
	/// The discriminant could not be encoded.
	BadDiscriminant(D),

	/// A field could not be encoded.
	BadField(F),
}

impl<D, F> Display for EnumEncodeError<D, F>
where
	D: Display,
	F: Display,
{
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match *self {
			Self::BadDiscriminant(ref e)
			=> write!(f, "discriminant could not be encoded: {e}"),

			Self::BadField(ref e)
			=> write!(f, "field could not be encoded: {e}"),
		}
	}
}

impl<D, F> Error for EnumEncodeError<D, F>
where
	D: Error + 'static,
	F: Error + 'static,
{
	#[inline]
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match *self {
			Self::BadDiscriminant(ref e) => Some(e),

			Self::BadField(ref e) => Some(e),
		}
	}
}

impl<D, F> From<Infallible> for EnumEncodeError<D, F> {
	#[inline(always)]
	fn from(_value: Infallible) -> Self {
		// SAFETY: `Infallible` objects can never be con-
		// structed.
		unsafe { unreachable_unchecked() };
	}
}

impl<D, F> From<EnumEncodeError<D, F>> for Infallible
where
	D: Into<Self>,
	F: Into<Self>,
{
	#[inline(always)]
	fn from(_value: EnumEncodeError<D, F>) -> Self {
		unreachable!()
	}
}
