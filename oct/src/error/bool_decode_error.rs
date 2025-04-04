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

/// A boolean could not be decoded.
///
/// Boolean values are boolean.
/// This error type is emitted when <code>&lt;[bool] as [Decode]&gt;::[decode]</code> encounters a non-boolean octet.
///
/// [Decode]: [crate::decode::Decode]
/// [decode]: [crate::decode::Decode::decode]
#[derive(Debug, Eq, PartialEq)]
#[must_use]
pub struct BoolDecodeError {
	/// The non-boolean value.
	pub value: u8,
}

impl Display for BoolDecodeError {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "value {:#02X} is not boolean", self.value)
	}
}

impl Error for BoolDecodeError { }

impl From<Infallible> for BoolDecodeError {
	#[inline(always)]
	fn from(_value: Infallible) -> Self {
		unreachable!()
	}
}
