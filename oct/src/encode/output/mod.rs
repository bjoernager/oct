// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

mod test;

use crate::error::OutputError;

use core::borrow::Borrow;
use core::fmt::{self, Debug, Formatter};
use core::ptr::copy_nonoverlapping;
use core::slice;

/// Byte stream suitable for writing.
pub struct Output<'a> {
	buf: &'a mut [u8],
	pos: usize,
}

impl<'a> Output<'a> {
	/// Constructs a new output stream.
	#[inline(always)]
	#[must_use]
	pub const fn new(buf: &'a mut [u8]) -> Self {
		Self { buf, pos: 0x0 }
	}

	/// Writes bytes to the stream.
	///
	/// # Errors
	///
	/// If the requested amount of bytes could not be exactly written, then this method will return an error.
	#[inline]
	pub const fn write(&mut self, data: &[u8]) -> Result<(), OutputError> {
		let remaining = self.buf.len() - self.pos;
		let count     = data.len();

		if remaining < count {
			return Err(OutputError {
				capacity: self.buf.len(),
				position: self.pos,
				count,
			});
		}

		unsafe {
			let src = data.as_ptr();
			let dst = self.buf.as_mut_ptr().add(self.pos);

			copy_nonoverlapping(src, dst, count);
		}

		self.pos += count;

		Ok(())
	}

	/// Retrieves the maximum capacity of the output stream.
	#[inline(always)]
	#[must_use]
	pub const fn capacity(&self) -> usize {
		self.buf.len()
	}

	/// Retrieves the remaining, free capacity of the output stream.
	#[inline(always)]
	#[must_use]
	pub const fn remaining(&self) -> usize {
		// SAFETY: The cursor position can never exceed the
		// stream's capacity.
		unsafe { self.capacity().unchecked_sub(self.position()) }
	}

	/// Retrieves the current cursor position of the output stream.
	#[inline(always)]
	#[must_use]
	pub const fn position(&self) -> usize {
		self.pos
	}

	/// Gets a pointer to the first byte of the output stream.
	#[inline(always)]
	#[must_use]
	pub const fn as_ptr(&self) -> *const u8 {
		self.buf.as_ptr()
	}

	/// Gets a slice of the written bytes.
	#[inline(always)]
	#[must_use]
	pub const fn as_slice(&self) -> &[u8] {
		unsafe {
			let ptr = self.as_ptr();
			let len = self.position();

			slice::from_raw_parts(ptr, len)
		}
	}
}

impl AsRef<[u8]> for Output<'_> {
	#[inline(always)]
	fn as_ref(&self) -> &[u8] {
		self.as_slice()
	}
}

impl Borrow<[u8]> for Output<'_> {
	#[inline(always)]
	fn borrow(&self) -> &[u8] {
		self.as_slice()
	}
}

impl Debug for Output<'_> {
	#[inline]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		Debug::fmt(self.as_slice(), f)
	}
}

impl Eq for Output<'_> { }

impl PartialEq for Output<'_> {
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool {
		*self.as_slice() == *other.as_slice()
	}

	#[expect(clippy::partialeq_ne_impl)]
	#[inline(always)]
	fn ne(&self, other: &Self) -> bool {
		*self.as_slice() != *other.as_slice()
	}
}

impl PartialEq<[u8]> for Output<'_> {
	#[inline(always)]
	fn eq(&self, other: &[u8]) -> bool {
		*self.as_slice() == *other
	}

	#[expect(clippy::partialeq_ne_impl)]
	#[inline(always)]
	fn ne(&self, other: &[u8]) -> bool {
		*self.as_slice() != *other
	}
}

impl PartialEq<&[u8]> for Output<'_> {
	#[inline(always)]
	fn eq(&self, other: &&[u8]) -> bool {
		*self.as_slice() == **other
	}

	#[expect(clippy::partialeq_ne_impl)]
	#[inline(always)]
	fn ne(&self, other: &&[u8]) -> bool {
		*self.as_slice() != **other
	}
}
