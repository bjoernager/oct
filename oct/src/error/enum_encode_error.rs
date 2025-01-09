// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::encode::Encode;

use core::convert::Infallible;
use core::error::Error;
use core::fmt::{self, Debug, Display, Formatter};

/// An enumeration could not be encoded.
#[derive(Debug)]
#[must_use]
pub enum EnumEncodeError<D: Encode, F> {
	/// The discriminant could not be encoded.
	BadDiscriminant(D::Error),

	/// A field could not be encoded.
	BadField(F),
}

impl<D, F> Display for EnumEncodeError<D, F>
where
	D: Display + Encode<Error: Display>,
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
	D: Debug + Display + Encode<Error: Error + 'static>,
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

impl<D, F> From<EnumEncodeError<D, F>> for Infallible
where
	D: Encode<Error: Into<Self>>,
	F: Into<Self>,
{
	#[inline(always)]
	fn from(_value: EnumEncodeError<D, F>) -> Self {
		unreachable!()
	}
}
