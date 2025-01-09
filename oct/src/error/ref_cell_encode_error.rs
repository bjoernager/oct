// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use core::cell::BorrowError;
use core::error::Error;
use core::fmt::{self, Display, Formatter};

/// A reference cell could not be encoded.
///
/// The implementation of <code>&lt;[RefCell](core::cell::RefCell)&lt;T&gt; as [Encode](crate::encode::Encode)&gt;::[encode](crate::encode::Encode::encode)</code> will first attempt to call <code>RefCell::[borrow](core::cell::RefCell::borrow)</code>.
/// If this call fails, then the returned error is again returned as a [`BadBorrow`](Self::BadBorrow) instance.
/// If the following call to <code>T::encode</code> fails instead, then the error returned from that call is passed on as a [`BadValue`](Self::BadValue) instance.
#[derive(Debug)]
#[must_use]
pub enum RefCellEncodeError<E> {
	/// The reference cell could not be borrowed.
	BadBorrow(BorrowError),

	/// The contained value could not be encoded.
	BadValue(E),
}

impl<E: Display> Display for RefCellEncodeError<E> {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		let e: &dyn Display = match *self {
			Self::BadBorrow(ref e) => e,

			Self::BadValue(ref e) => e,
		};

		write!(f, "unable to encode reference cell: {e}")
	}
}

impl<E: Error + 'static> Error for RefCellEncodeError<E> {
	#[inline(always)]
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match *self {
			Self::BadBorrow(ref e) => Some(e),

			Self::BadValue(ref e) => Some(e)
		}
	}
}
