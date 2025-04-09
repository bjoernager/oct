// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

mod test;

use core::borrow::Borrow;
use core::fmt::{self, Debug, Formatter};
use core::ptr::copy_nonoverlapping;
use core::slice;

/// Byte stream suitable for reading.
pub struct Input<'a> {
	buf: &'a [u8],
	pos: usize,
}

// FIXME: Needed due to borrow rules; can't simply
// use the method.
macro_rules! peek {
	($self:expr, $count:expr$(,)?) => {{
		let this: &Input = $self;

		let count: usize = $count;

		let remaining = this.buf.len() - this.pos;

		if remaining < count {
			panic!(
				"cannot peek or read `{}` bytes at `{}` from input stream of capacity `{}`",
				count,
				this.position(),
				this.capacity(),
			)
		}

		unsafe {
			let ptr = this.buf.as_ptr().add(this.pos);

			slice::from_raw_parts(ptr, count)
		}
	}};
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
	/// # Panics
	///
	/// If the requested amount of bytes could not be exactly read, then this method will panic.
	#[inline]
	pub fn read(&mut self, count: usize) -> &[u8] {
		let data = peek!(self, count);

		self.pos += count;

		data
	}

	/// Reads bytes from the stream into a predefined buffer.
	///
	/// This method may be preferred over [`read`](Self::read) if the read data are *directly* needed, e.g. if all required transformations can be done in-place.
	///
	/// # Panic
	///
	/// If the provided buffer could not be completely filled, then this method will panic.
	#[inline]
	pub fn read_into(&mut self, buf: &mut [u8]) {
		self.peek_into(buf);

		self.pos += buf.len();
	}

	/// Reads bytes from the stream without moving the cursor.
	///
	/// This method may be preferred over [`read`](Self::read) if the same bytes may be needed more than once.
	/// It may additionally be preferred over [`peek_into`](Self::peek_into) if the read data aren't directly needed, such as if an iterator is applied anyway to map the data.
	///
	/// # Panics
	///
	/// If the requested amount of bytes could not be exactly read, then this method will panic.
	#[inline]
	pub fn peek(&self, count: usize) -> &[u8] {
		peek!(self, count)
	}

	/// Reads bytes from the stream into a predefined buffer without moving the cursor.
	///
	/// This method may be preferred over [`read_into`](Self::read_into) if the same bytes may be needed more than once.
	/// It may additionally be preferred over [`read`](Self::read) if the read data are *directly* needed, e.g. if all required transformations can be done in-place.
	///
	/// # Panics
	///
	/// If the provided buffer could not be completely filled, then this method will panic.
	#[inline]
	pub fn peek_into(&self, buf: &mut [u8]) {
		let count     = buf.len();
		let remaining = self.remaining();

		assert!(
			remaining >= count,
			"cannot peek or read `{}` bytes at `{}` from input stream of capacity `{}`",
			count,
			self.position(),
			self.capacity(),
		);

		unsafe {
			let src = self.buf.as_ptr().add(self.pos);
			let dst = buf.as_mut_ptr();

			copy_nonoverlapping(src, dst, count);
		}
	}

	/// Retrieves the maximum capacity of the input stream.
	#[inline(always)]
	#[must_use]
	pub fn capacity(&self) -> usize {
		self.buf.len()
	}

	/// Retrieves the remaining, free capacity of the input stream.
	#[inline(always)]
	#[must_use]
	pub fn remaining(&self) -> usize {
		// SAFETY: The cursor position can never exceed the
		// stream's capacity.
		unsafe { self.capacity().unchecked_sub(self.position()) }
	}

	/// Retrieves the current cursor position of the input stream.
	#[inline(always)]
	#[must_use]
	pub fn position(&self) -> usize {
		self.pos
	}

	/// Gets a pointer to the next byte of the input stream.
	#[inline(always)]
	#[must_use]
	pub fn as_ptr(&self) -> *const u8 {
		unsafe { self.buf.as_ptr().add(self.position()) }
	}

	/// Gets a slice of the remaining bytes.
	#[inline(always)]
	#[must_use]
	pub fn as_slice(&self) -> &[u8] {
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
	#[inline]
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
