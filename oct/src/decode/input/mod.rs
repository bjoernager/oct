// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

#[cfg(test)]
mod test;

use crate::error::InputError;

use core::borrow::Borrow;
use core::fmt::{self, Debug, Formatter};
use core::ptr::copy_nonoverlapping;
use core::slice;

/// Byte stream suitable for reading.
pub struct Input<'a> {
	buf: &'a [u8],
	pos: usize,
}

impl<'a> Input<'a> {
	/// Constructs a new input stream.
	#[inline(always)]
	#[must_use]
	pub fn new(buf: &'a [u8]) -> Self {
		Self { buf, pos: 0x0 }
	}

	/// Reads bytes from the stream.
	///
	/// This method may be preferred over [`read_into`](Self::read_into) if the read data aren't directly needed, such as if an iterator is applied anyway to map the data.
	///
	/// # Errors
	///
	/// If the requested amount of bytes could not be exactly read, then this method will return an error.
	#[inline]
	pub const fn read(&mut self, count: usize) -> Result<&'a [u8], InputError> {
		let data = match self.peek(count) {
			Ok(data) => data,

			Err(e) => return Err(e),
		};

		self.pos += count;

		Ok(data)
	}

	/// Reads bytes from the stream into a predefined buffer.
	///
	/// This method may be preferred over [`read`](Self::read) if the read data are *directly* needed, e.g. if all required transformations can be done in-place.
	///
	/// # Errors
	///
	/// If the provided buffer could not be completely filled, then this method will return an error.
	#[inline]
	pub const fn read_into(&mut self, buf: &mut [u8]) -> Result<(), InputError> {
		if let Err(e) = self.peek_into(buf) {
			return Err(e);
		}

		self.pos += buf.len();

		Ok(())
	}

	/// Reads bytes from the stream without moving the cursor.
	///
	/// This method may be preferred over [`read`](Self::read) if the same bytes may be needed more than once.
	/// It may additionally be preferred over [`peek_into`](Self::peek_into) if the read data aren't directly needed, such as if an iterator is applied anyway to map the data.
	///
	/// # Errors
	///
	/// If the requested amount of bytes could not be exactly read, then this method will return an error.
	#[inline]
	pub const fn peek(&mut self, count: usize) -> Result<&'a [u8], InputError> {
		let remaining = self.buf.len() - self.pos;

		if remaining < count {
			return Err(InputError {
				capacity: self.buf.len(),
				position: self.pos,
				count,
			});
		}

		let data = unsafe {
			let ptr = self.buf.as_ptr().add(self.pos);

			slice::from_raw_parts(ptr, count)
		};

		Ok(data)
	}

	/// Reads bytes from the stream into a predefined buffer without moving the cursor.
	///
	/// This method may be preferred over [`read_into`](Self::read_into) if the same bytes may be needed more than once.
	/// It may additionally be preferred over [`read`](Self::read) if the read data are *directly* needed, e.g. if all required transformations can be done in-place.
	///
	/// # Errors
	///
	/// If the provided buffer could not be completely filled, then this method will return an error.
	#[inline]
	pub const fn peek_into(&mut self, buf: &mut [u8]) -> Result<(), InputError> {
		let count     = buf.len();
		let remaining = self.remaining();

		if remaining < count {
			return Err(InputError {
				capacity: self.buf.len(),
				position: self.pos,
				count,
			});
		}

		unsafe {
			let src = self.buf.as_ptr().add(self.pos);
			let dst = buf.as_mut_ptr();

			copy_nonoverlapping(src, dst, count);
		}

		Ok(())
	}

	/// Retrieves the maximum capacity of the input stream.
	#[inline(always)]
	#[must_use]
	pub const fn capacity(&self) -> usize {
		self.buf.len()
	}

	/// Retrieves the remaining, free capacity of the input stream.
	#[inline(always)]
	#[must_use]
	pub const fn remaining(&self) -> usize {
		// SAFETY: The cursor position can never exceed the
		// stream's capacity.
		unsafe { self.capacity().unchecked_sub(self.position()) }
	}

	/// Retrieves the current cursor position of the input stream.
	#[inline(always)]
	#[must_use]
	pub const fn position(&self) -> usize {
		self.pos
	}

	/// Gets a pointer to the next byte of the input stream.
	#[inline(always)]
	#[must_use]
	pub const fn as_ptr(&self) -> *const u8 {
		unsafe { self.buf.as_ptr().add(self.position()) }
	}

	/// Gets a slice of the remaining bytes.
	#[inline(always)]
	#[must_use]
	pub const fn as_slice(&self) -> &[u8] {
		unsafe {
			let ptr = self.as_ptr();
			let len = self.remaining();

			slice::from_raw_parts(ptr, len)
		}
	}
}

impl AsRef<[u8]> for Input<'_> {
	#[inline(always)]
	fn as_ref(&self) -> &[u8] {
		self.as_slice()
	}
}

impl Borrow<[u8]> for Input<'_> {
	#[inline(always)]
	fn borrow(&self) -> &[u8] {
		self.as_slice()
	}
}

impl Debug for Input<'_> {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		Debug::fmt(self.as_slice(), f)
	}
}

impl Eq for Input<'_> { }

impl PartialEq for Input<'_> {
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

impl PartialEq<[u8]> for Input<'_> {
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

impl PartialEq<&[u8]> for Input<'_> {
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
