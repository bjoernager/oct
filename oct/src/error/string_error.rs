// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::error::{LengthError, Utf8Error};

use core::error::Error;
use core::fmt::{self, Display, Formatter};

/// String error variants.
#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
#[must_use]
pub enum StringError {
	/// An invalid UTF-8 sequence was encountered.
	BadUtf8(Utf8Error),

	/// A fixed-size buffer was too small.
	SmallBuffer(LengthError),
}

impl Display for StringError {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match *self {
			Self::BadUtf8(ref e)
			=> write!(f, "bad utf-8: {e}"),

			Self::SmallBuffer(ref e)
			=> write!(f, "buffer too small: {e}"),
		}
	}
}

impl Error for StringError {
	#[inline]
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match *self {
			Self::BadUtf8(ref e) => Some(e),

			Self::SmallBuffer(ref e) => Some(e),
		}
	}
}
