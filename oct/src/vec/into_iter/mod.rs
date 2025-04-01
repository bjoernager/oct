// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

mod test;

use crate::vec::{clone_to_uninit_in_range, Vec};

use core::iter::{DoubleEndedIterator, ExactSizeIterator, FusedIterator};
use core::mem::MaybeUninit;
use core::ptr::drop_in_place;
use core::slice;

/// Owning iterator to a vector.
///
/// This type is exclusively used by the deconstruction of the [`Vec`] type, as per <code>[IntoIterator]::[into_iter](IntoIterator::into_iter)</code>.
#[must_use]
pub struct IntoIter<T, const N: usize> {
	/// The cursor position in the buffer.
	///
	/// # Safety
	///
	/// This field may not be greater than [`isize::MAX`].
	pos: usize,

	/// The length; the count remaining of alive elements remaining in the buffer.
	///
	/// # Safety
	///
	/// This field may not be greater than [`isize::MAX`].
	len: usize,

	/// The internal buffer.
	///
	/// # Safety
	///
	/// We must **always** guarantee that all objects in the range `pos..pos + len` are initialised and alive.
	/// One may therefore assume that interpreting these objects as such is valid.
	buf: [MaybeUninit<T>; N],
}

impl<T, const N: usize> IntoIter<T, N> {
	/// Constructs a new, owning iterator to a vector.
	#[inline(always)]
	#[track_caller]
	pub(super) fn new(v: Vec<T, N>) -> Self {
		let (buf, len) = v.into_raw_parts();

		let pos = Default::default();

		Self { pos, len, buf }
	}

	/// Incremenets the cursor position by a specified amount.
	///
	/// The caller is responsible for dropping the skipped elements.
	///
	/// # Safety
	///
	/// The iterator `self` may not contain less than `count` elements.
	#[inline(always)]
	#[track_caller]
	unsafe fn advance_by_unchecked(&mut self, count: usize) {
		debug_assert!(count <= self.len);

		// SAFETY: The caller guarantees that at least
		// `count` element are remaining.
		self.len = unsafe { self.len.unchecked_sub(count) };

		// SAFETY: It is not invalid for us to go one-past-
		// the-end or exceed `isize::MAX`; `len` guarantees
		// that this counter will not be used as a pointer
		// offset in such cases.
		self.pos = unsafe { self.pos.unchecked_add(count) };
	}

	/// Decrements the length counter by a specified amount.
	///
	/// The caller is responsible for dropping the skipped elements.
	///
	/// # Safety
	///
	/// The iterator `self` may not contain less than `count` elements.
	#[inline(always)]
	#[track_caller]
	unsafe fn advance_back_by_unchecked(&mut self, count: usize) {
		debug_assert!(count <= self.len);

		// SAFETY: The caller guarantees that at least
		// `count` element are remaining.
		self.len = unsafe { self.len.unchecked_sub(count) };
	}

	/// Gets a pointer to the current element.
	///
	/// If the iterator `self` is currently empty, then the returned pointer will instead be dangling.
	#[inline(always)]
	fn as_ptr(&self) -> *const T {
		let pos = self.pos;

		// SAFETY: `MaybeUninit<T>` is transparent to `T`.
		let base = self.buf.as_ptr() as *const T;

		unsafe { base.add(pos) }
	}

	/// Gets a mutable pointer to the current element.
	///
	/// If the iterator `self` is currently empty, then the returned pointer will instead be dangling.
	#[inline(always)]
	fn as_mut_ptr(&mut self) -> *mut T {
		let pos = self.pos;

		// SAFETY: `MaybeUninit<T>` is transparent to `T`.
		let base = self.buf.as_mut_ptr() as *mut T;

		unsafe { base.add(pos) }
	}

	/// Gets a slice of the remaining elements.
	#[inline(always)]
	pub fn as_slice(&self) -> &[T] {
		let len = self.len;
		let ptr = self.as_ptr();

		unsafe { slice::from_raw_parts(ptr, len) }
	}

	/// Gets a mutable slice of the remaining elements.
	#[inline(always)]
	pub fn as_mut_slice(&mut self) -> &mut [T] {
		let len = self.len;
		let ptr = self.as_mut_ptr();

		unsafe { slice::from_raw_parts_mut(ptr, len) }
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
		let     pos = self.pos;
		let     len = self.len;
		let mut buf = [const { MaybeUninit::uninit() }; N];

		// SAFETY: `MaybeUninit<T>` is transparent to `T`.
		let src = self.buf.as_ptr() as *const T;

		let dst = buf.as_mut_ptr() as *mut T;

		let start = pos;
		let end   = pos + len;

		// SAFETY: The range
		//
		// pos..pos + len
		//
		// defines in and of itself the bounds of valid
		// elements.
		unsafe { clone_to_uninit_in_range(src, dst, start..end) };

		// SAFETY: The buffer has been initialised in the
		// provided range - which does not extend beyond
		// bounds.
		Self { pos, len, buf }
	}
}

impl<T, const N: usize> Default for IntoIter<T, N> {
	#[inline(always)]
	fn default() -> Self {
		Vec::default().into_iter()
	}
}

impl<T, const N: usize> DoubleEndedIterator for IntoIter<T, N> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		// Test whether the iterator is empty.

		if self.len == 0x0 {
			return None;
		}

		// Take the next value.

		// Get a pointer to the next item.

		// SAFETY: `self.pos` is guaranteed to always be
		// within bounds. `self.pos + self.len` is guaran-
		// teed one-past-the-end index.
		let index = self.pos + self.len - 0x1;

		// SAFETY: `MaybeUninit<T>` is transparent to `T`.
		let base = self.buf.as_ptr() as *const T;

		let item = unsafe { base.add(index) };

		// Read the item value.

		let value = unsafe { item.read() };

		// Update counters, **not** including the position.

		// SAFETY: We have tested that at least one element
		// remains.
		unsafe { self.advance_back_by_unchecked(0x1) };

		Some(value)
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
	}
}

impl<T, const N: usize> ExactSizeIterator for IntoIter<T, N> {
	#[inline(always)]
	fn len(&self) -> usize {
		self.len
	}
}

impl<T, const N: usize> FusedIterator for IntoIter<T, N> { }

impl<T, const N: usize> Iterator for IntoIter<T, N> {
	type Item = T;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		// Test whether the iterator is empty.

		if self.len == 0x0 {
			return None;
		}

		// Take the next value.

		// Get a pointer to the next item.

		// SAFETY: `self.pos` is guaranteed to always be
		// within bounds.
		let index = self.pos;

		// SAFETY: `MaybeUninit<T>` is transparent to `T`.
		let base = self.buf.as_ptr() as *const T;

		let item = unsafe { base.add(index) };

		// Read the item value.

		// SAFETY: We guarantee that all items in the range
		//
		// self.pos..self.pos + self.len
		//
		// are alive (and initialised).
		let value = unsafe { item.read() };

		// Update counters.

		// SAFETY: We have tested that at least one element
		// remains.
		unsafe { self.advance_by_unchecked(0x1) };

		// Return the item value.

		Some(value)
	}

	#[inline]
	fn nth(&mut self, n: usize) -> Option<Self::Item> {
		// Test whether the iterator is empty.

		if n >= self.len {
			return None;
		}

		// Get the indices of the involved items.

		let drop_start = self.pos;
		let drop_end   = drop_start + n;

		let index = drop_end;

		// SAFETY: `MaybeUninit<T>` is transparent to `T`.
		let base = self.buf.as_mut_ptr() as *mut T;

		// Drop each skipped element.

		for i in drop_start..drop_end {
			let item = unsafe { base.add(i) };

			// SAFETY: We guarantee that all items in the range
			//
			// self.pos..self.pos + self.len
			//
			// are alive (and initialised).
			unsafe { drop_in_place(item) };
		}

		// Read the final value.

		let item  = unsafe { base.add(index) };
		let value = unsafe { item.read() };

		// Update counters.

		// SAFETY: This cannot overflow as `n` has been
		// tested to be less than `self.len`, which itself
		// cannot be greater than `isize::MAX`.
		let count = unsafe { n.unchecked_add(0x1) };

		// SAFETY: We have tested that there are at least
		// `count` elements left.
		unsafe { self.advance_by_unchecked(count) };

		// Return the value.

		Some(value)
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len;
		(len, Some(len))
	}

	#[inline(always)]
	fn count(self) -> usize {
		// NOTE: Elements are dropped automatically.
		self.len
	}

	#[inline(always)]
	fn is_sorted(self) -> bool
	where
		T: PartialOrd,
	{
		self.as_slice().is_sorted()
	}

	#[inline(always)]
	fn is_sorted_by<F: FnMut(&Self::Item, &Self::Item) -> bool>(self, compare: F) -> bool {
		self.as_slice().is_sorted_by(compare)
	}
}
