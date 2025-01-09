// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use core::error::Error;
use core::fmt::{self, Display, Formatter};

/// An invalid UTF-8 sequence was encountered.
#[derive(Debug, Eq, PartialEq)]
#[must_use]
pub struct Utf8Error {
	/// The invalid UTF-8 octet.
	pub value: u8,

	/// The index of the invalid octet.
	pub index: usize,
}

impl Display for Utf8Error {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "found invalid utf-8 octet {:#02X} at offset ({})", self.value, self.index)
	}
}

impl Error for Utf8Error { }
