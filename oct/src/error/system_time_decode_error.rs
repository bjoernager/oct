// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use core::error::Error;
use core::fmt::{self, Display, Formatter};

/// The [`SystemTime`](std::time::SystemTime) type could not represent a UNIX timestamp.
///
/// Note that a UNIX timestamp is here defined as a signed, 64-bit integer denoting a difference of time to 1 january 1970, as measured in Greenwich using seconds.
/// This error should therefore not occur on systems that use the same or a more precise counter.
#[cfg_attr(doc, doc(cfg(feature = "std")))]
#[derive(Debug, Eq, PartialEq)]
#[must_use]
pub struct SystemTimeDecodeError {
	/// The unrepresentable timestamp.
	pub timestamp: i64,
}

#[cfg_attr(doc, doc(cfg(feature = "std")))]
impl Display for SystemTimeDecodeError {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "could not represent `{}` as a system timestamp", self.timestamp)
	}
}

#[cfg_attr(doc, doc(cfg(feature = "std")))]
impl Error for SystemTimeDecodeError { }
