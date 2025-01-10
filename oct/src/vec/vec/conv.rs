// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::vec::Vec;
use crate::error::LengthError;

use core::borrow::{Borrow, BorrowMut};
use core::mem::{ManuallyDrop, MaybeUninit};
use core::ops::{Deref, DerefMut};
use core::ptr::copy_nonoverlapping;
use core::slice;

#[cfg(feature = "alloc")]
use {
	alloc::alloc::alloc,
	alloc::boxed::Box,
	core::alloc::Layout,
};

impl<T, const N: usize> Vec<T, N> {
	/// Constructs a fixed-size vector from raw parts.
	///
	/// The provided parts are not tested in any way.
	///
	/// # Safety
	///
	/// The value of `len` may not exceed that of `N`.
	/// Additionally, all elements of `buf` in the range specified by `len` must be initialised.
	///
	/// If any of these requirements are violated, behaviour is undefined.
	#[inline(always)]
	#[must_use]
	pub const unsafe fn from_raw_parts(buf: [MaybeUninit<T>; N], len: usize) -> Self {
		debug_assert!(len <= N, "cannot construct vector longer than its capacity");

		Self { buf, len }
	}

	/// Gets a pointer to the first element.
	///
	/// The pointed-to element may not necessarily be initialised.
	/// See [`len`](Self::len) for more information.
	#[inline(always)]
	#[must_use]
	pub const fn as_ptr(&self) -> *const T {
		self.buf.as_ptr().cast()
	}

	/// Gets a mutable pointer to the first element.
	///
	/// The pointed-to element may not necessarily be initialised.
	/// See [`len`](Self::len) for more information.
	#[inline(always)]
	#[must_use]
	pub const fn as_mut_ptr(&mut self) -> *mut T {
		self.buf.as_mut_ptr().cast()
	}

	/// Borrows the vector as a slice.
	///
	/// The range of the returned slice only includes the elements specified by [`len`](Self::len).
	#[inline(always)]
	#[must_use]
	pub const fn as_slice(&self) -> &[T] {
		let ptr = self.as_ptr();
		let len = self.len();

		unsafe { slice::from_raw_parts(ptr, len) }
	}

	/// Borrows the vector as a mutable slice.
	///
	/// The range of the returned slice only includes the elements specified by [`len`](Self::len).
	#[inline(always)]
	#[must_use]
	pub const fn as_mut_slice(&mut self) -> &mut [T] {
		let ptr = self.as_mut_ptr();
		let len = self.len();

		unsafe { slice::from_raw_parts_mut(ptr, len) }
	}

	/// Destructs the vector into its raw parts.
	///
	/// The returned parts are valid to pass back to [`from_raw_parts`](Self::from_raw_parts).
	#[inline(always)]
	#[must_use]
	pub const fn into_raw_parts(self) -> ([MaybeUninit<T>; N], usize) {
		let this = ManuallyDrop::new(self);

		unsafe {
			// SAFETY: `ManuallyDrop<T>` is transparent to `T`.
			// We also aren't dropping `this`, so we can safely
			// move out of it.
			let this = &*(&raw const this).cast::<Self>();

			let buf = (&raw const this.buf).read();
			let len = this.len;

			(buf, len)
		}
	}

	/// Converts the vector into a boxed slice.
	///
	/// The vector is reallocated using the global allocator.
	#[cfg(feature = "alloc")]
	#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
	#[must_use]
	pub fn into_boxed_slice(self) -> Box<[T]> {
		let (buf, len) = self.into_raw_parts();

		unsafe {
			let layout = Layout::array::<T>(len).unwrap();
			let ptr = alloc(layout).cast::<T>();

			assert!(!ptr.is_null(), "allocation failed");

			copy_nonoverlapping(buf.as_ptr().cast(), ptr, len);

			let slice = core::ptr::slice_from_raw_parts_mut(ptr, len);
			Box::from_raw(slice)

			// `self.buf` is dropped without destructors being
			// run.
		}
	}

	/// Converts the vector into a dynamically-allocated vector.
	///
	/// The vector is reallocated using the global allocator.
	#[cfg(feature = "alloc")]
	#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
	#[inline(always)]
	#[must_use]
	pub fn into_alloc_vec(self) -> alloc::vec::Vec<T> {
		self.into_boxed_slice().into_vec()
	}
}

impl<T, const N: usize> AsMut<[T]> for Vec<T, N> {
	#[inline(always)]
	fn as_mut(&mut self) -> &mut [T] {
		self.as_mut_slice()
	}
}

impl<T, const N: usize> AsRef<[T]> for Vec<T, N> {
	#[inline(always)]
	fn as_ref(&self) -> &[T] {
		self.as_slice()
	}
}

impl<T, const N: usize> Borrow<[T]> for Vec<T, N> {
	#[inline(always)]
	fn borrow(&self) -> &[T] {
		self.as_slice()
	}
}

impl<T, const N: usize> BorrowMut<[T]> for Vec<T, N> {
	#[inline(always)]
	fn borrow_mut(&mut self) -> &mut [T] {
		self.as_mut_slice()
	}
}

impl<T, const N: usize> Deref for Vec<T, N> {
	type Target = [T];

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		self.as_slice()
	}
}

impl<T, const N: usize> DerefMut for Vec<T, N> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.as_mut_slice()
	}
}

impl<T, const N: usize> From<[T; N]> for Vec<T, N> {
	#[inline(always)]
	fn from(value: [T; N]) -> Self {
		unsafe {
			let buf = value.as_ptr().cast::<[MaybeUninit<T>; N]>().read();

			Self { buf, len: N }
		}
	}
}

impl<T: Copy, const N: usize> TryFrom<&[T]> for Vec<T, N> {
	type Error = LengthError;

	#[inline(always)]
	fn try_from(value: &[T]) -> Result<Self, Self::Error> {
		Self::new(value)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T, const N: usize> From<Vec<T, N>> for Box<[T]> {
	#[inline(always)]
	fn from(value: Vec<T, N>) -> Self {
		value.into_boxed_slice()
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T, const N: usize> From<Vec<T, N>> for alloc::vec::Vec<T> {
	#[inline(always)]
	fn from(value: Vec<T, N>) -> Self {
		value.into_alloc_vec()
	}
}
