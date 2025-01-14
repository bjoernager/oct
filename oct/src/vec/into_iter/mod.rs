// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

#[cfg(test)]
mod test;

use core::iter::{DoubleEndedIterator, ExactSizeIterator, FusedIterator};
use core::mem::MaybeUninit;
use core::ptr::drop_in_place;
use core::slice;

/// Owning iterator to a vector.
///
/// This type is exclusively used by the deconstruction of the [`Vec`](crate::vec::Vec) type.
/// When just borrowing such vectors, the standard library's <code>core::slice::{[Iter](core::slice::Iter), [IterMut](core::slice::IterMut)}</code> types are used instead.
#[must_use]
pub struct IntoIter<T, const N: usize> {
	len: usize,
	pos: usize,

	buf: [MaybeUninit<T>; N],
}

impl<T, const N: usize> IntoIter<T, N> {
	/// Constructs a new, size-constrained iterator.
	#[inline(always)]
	pub(crate) const unsafe fn new(buf: [MaybeUninit<T>; N], len: usize) -> Self {
		debug_assert!(len <= N, "cannot construct iterator longer than its capacity");

		Self { len, pos: 0x0, buf }
	}

	/// Gets a slice of the remaining elements.
	#[inline(always)]
	pub const fn as_slice(&self) -> &[T] {
		unsafe {
			let ptr = self.buf
				.as_ptr()
				.add(self.pos)
				.cast();

			slice::from_raw_parts(ptr, self.len)
		}
	}

	/// Gets a mutable slice of the remaining elements.
	#[inline(always)]
	pub const fn as_mut_slice(&mut self) -> &mut [T] {
		unsafe {
			let ptr = self.buf
				.as_mut_ptr()
				.add(self.pos)
				.cast();

			slice::from_raw_parts_mut(ptr, self.len)
		}
	}
}

impl<T, const N: usize> AsMut<[T]> for IntoIter<T, N> {
	#[inline(always)]
	fn as_mut(&mut self) -> &mut [T] {
		self.as_mut_slice()
	}
}

impl<T, const N: usize> AsRef<[T]> for IntoIter<T, N> {
	#[inline(always)]
	fn as_ref(&self) -> &[T] {
		self.as_slice()
	}
}

impl<T: Clone, const N: usize> Clone for IntoIter<T, N> {
	#[inline]
	fn clone(&self) -> Self {
		let mut buf = [const { MaybeUninit::<T>::uninit() }; N];
		let Self { pos, len, .. } = *self;

		let start = pos;
		let stop  = start + len;

		for i in start..stop {
			unsafe {
				let item = (&raw const *self.buf.get_unchecked(i)).cast();

				let value = Clone::clone(&*item);

				buf.get_unchecked_mut(i).write(value);
			}
		}

		Self { len, pos, buf }
	}
}

impl<T, const N: usize> DoubleEndedIterator for IntoIter<T, N> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		if self.len == 0x0 { return None };

		let index = self.pos + self.len - 0x1;

		let item = unsafe { self.buf.get_unchecked(index).assume_init_read() };

		self.len -= 0x1;

		Some(item)
	}
}

impl<T, const N: usize> Drop for IntoIter<T, N> {
	#[inline(always)]
	fn drop(&mut self) {
		// Drop every element that hasn't been consumed.

		let remaining = self.as_mut_slice();
		unsafe { drop_in_place(remaining) };

		// We do not need to ensure that `self` is in a
		// valid state after this call to `drop`.
		// `MaybeUninit` also doesn't run destructors.
	}
}

impl<T, const N: usize> ExactSizeIterator for IntoIter<T, N> { }

impl<T, const N: usize> FusedIterator for IntoIter<T, N> { }

impl<T, const N: usize> Iterator for IntoIter<T, N> {
	type Item = T;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		if self.len == 0x0 { return None };

		let index = self.pos;

		let item = unsafe { self.buf.get_unchecked(index).assume_init_read() };

		self.len -= 0x1;
		self.pos += 0x1;

		Some(item)
	}

	#[inline]
	fn nth(&mut self, index: usize) -> Option<Self::Item> {
		if index > self.len { return None };

		let skipped = {
			let start = self.pos;
			let stop  = start + index - 0x1;

			unsafe { self.buf.get_unchecked_mut(start..stop) }
		};

		// Drop each skipped element.

		unsafe { drop_in_place(skipped) };

		// Read the final element.

		// SAFETY: `index` has been tested to be within
		// bounds, and the element at that position is also
		// guaranteed to still be alive.
		let item = unsafe { self.buf.get_unchecked(index).assume_init_read() };

		self.len -= index;
		self.pos += index;

		Some(item)
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let rem = unsafe { self.len.unchecked_sub(self.pos) };

		(rem, Some(rem))
	}
}
