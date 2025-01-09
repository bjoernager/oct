// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

#[cfg(test)]
mod tests;

use crate::error::LengthError;

use core::fmt::{self, Debug, Formatter};
use core::hash::{Hash, Hasher};
use core::mem::MaybeUninit;
use core::ops::{Index, IndexMut};
use core::ptr::{copy_nonoverlapping, drop_in_place, null, null_mut};
use core::slice::SliceIndex;

// Encode/decode facilities:
mod code;

// Conversion facilities:
mod conv;

// Comparison facilities:
mod cmp;

// Iterator facilities:
mod iter;

/// Vector container with maximum length.
///
/// This type is intended as a [sized-encodable](crate::encode::SizedEncode) and [decodable](crate::decode::Decode) alternative to slices and the standard library's [`Vec`](alloc::vec::Vec) (in cases where [arrays](array) may not be wanted).
///
/// Note that this type is immutable in the sense that it does **not** define methods like `push` and `pop`, in contrast to the standard library type.
///
/// See [`String`](crate::string::String) for an equivalent alternative to the standard library's [`String`](alloc::string::String).
///
/// # Examples
///
/// All instances of this type with the same `T` and `N` also have the exact same layout:
///
/// ```rust
/// use oct::vec::Vec;
///
/// let vec0 = Vec::<u8, 0x4>::try_from([0x3].as_slice()).unwrap();
/// let vec1 = Vec::<u8, 0x4>::try_from([0x3, 0x2].as_slice()).unwrap();
/// let vec2 = Vec::<u8, 0x4>::try_from([0x3, 0x2, 0x4].as_slice()).unwrap();
/// let vec3 = Vec::<u8, 0x4>::try_from([0x3, 0x2, 0x4, 0x3].as_slice()).unwrap();
///
/// assert_eq!(size_of_val(&vec0), size_of_val(&vec1));
/// assert_eq!(size_of_val(&vec0), size_of_val(&vec2));
/// assert_eq!(size_of_val(&vec0), size_of_val(&vec3));
/// assert_eq!(size_of_val(&vec1), size_of_val(&vec2));
/// assert_eq!(size_of_val(&vec1), size_of_val(&vec3));
/// assert_eq!(size_of_val(&vec2), size_of_val(&vec3));
/// ```
pub struct Vec<T, const N: usize> {
	buf: [MaybeUninit<T>; N],
	len: usize,
}

impl<T, const N: usize> Vec<T, N> {
	/// Constructs a new slice from existing data.
	#[inline(always)]
	pub const fn new(data: &[T]) -> Result<Self, LengthError>
	where
		T: Copy,
	{
		let len = data.len();
		if len > N {
			return Err(LengthError {
				remaining: N,
				count:     len,
			});
		}

		let mut buf = [const { MaybeUninit::<T>::uninit() };N];

		let this = unsafe {
			let src = data.as_ptr();
			let dst = buf.as_mut_ptr().cast();

			copy_nonoverlapping(src, dst, len);

			Self::from_raw_parts(buf, len)
		};

		Ok(this)
	}

	/// Copies elements from a slice.
	///
	/// # Panics
	///
	/// If `self` cannot contain the entirety of `data`, then this method will panic.
	#[inline]
	pub const fn copy_from_slice(&mut self, data: &[T])
	where
		T: Copy,
	{
		assert!(data.len() <= N, "vector cannot contain source slice");

		unsafe {
			let src = data.as_ptr();
			let dst = self.buf.as_mut_ptr().cast();

			// SAFETY: Pointers are exclusive due to reference
			// rules, and `T` implements `Copy`.
			copy_nonoverlapping(src, dst, data.len());

			// SAFETY: The length has
			self.set_len_unchecked(data.len());
		}
	}

	/// Generates a sized slice referencing the elements of `self`.
	#[inline]
	#[must_use]
	pub const fn each_ref(&self) -> Vec<&T, N> {
		let mut buf = [null::<T>(); N];
		let len = self.len;

		let mut i = 0x0;
		while i < len {
			unsafe {
				let item = buf.as_mut_ptr().add(i);

				let value = self.as_ptr().add(i).cast();
				item.write(value);
			}

			i += 0x1;
		}

		// SAFETY: `*const T` has the same layout as
		// `MaybeUninit<&T>`, and every relavent pointer
		// has been initialised as a valid reference.
		let buf = unsafe { (&raw const buf).cast::<[MaybeUninit<&T>; N]>().read() };

		unsafe { Vec::from_raw_parts(buf, len) }
	}

	/// Generates a sized slice mutably referencing the elements of `self`.
	#[inline]
	#[must_use]
	pub const fn each_mut(&mut self) -> Vec<&mut T, N> {
		let mut buf = [null_mut::<T>(); N];
		let len = self.len;

		let mut i = 0x0;
		while i < len {
			unsafe {
				let item = buf.as_mut_ptr().add(i);

				let value = self.as_mut_ptr().add(i).cast();
				item.write(value);
			}

			i += 0x1;
		}

		// SAFETY: `*mut T` has the same layout as
		// `MaybeUninit<&mut T>`, and every relavent point-
		// er has been initialised as a valid reference.
		let buf = unsafe { (&raw const buf).cast::<[MaybeUninit<&mut T>; N]>().read() };

		unsafe { Vec::from_raw_parts(buf, len) }
	}

	/// Sets the length of the vector.
	///
	/// The provided length is tested to be no greater than the current length.
	/// For the same operation *without* these checks, see [`set_len_unchecked`](Self::set_len_unchecked).
	///
	/// # Panics
	///
	/// The new length `len` may not be larger than `N`.
	///
	/// It is only valid to enlarge vectors if `T` supports being in a purely uninitialised state.
	/// Such is permitted with e.g. [`MaybeUninit`].
	#[inline(always)]
	pub const fn set_len(&mut self, len: usize) {
		assert!(len <= self.len(), "cannot extend length of vector");

		// SAFETY: We have asserted that the new length is
		// still within bounds.
		unsafe { self.set_len_unchecked(len) };
	}

	/// Unsafely sets the length of the vector.
	///
	/// The provided length is **not** tested in any way.
	/// For the same operation *with* these checks, see [`set_len`](Self::set_len).
	///
	/// # Safety
	///
	/// The new length `len` may not be larger than `N`.
	///
	/// It is only valid to enlarge vectors if `T` supports being in a purely uninitialised state
	/// Such is permitted by e.g. [`MaybeUninit`].
	#[inline(always)]
	pub const unsafe fn set_len_unchecked(&mut self, len: usize) {
		debug_assert!(len <= N, "cannot set length past bounds");

		self.len = len
	}

	/// Returns the length of the vector.
	///
	/// This value may necessarily be smaller than `N`.
	#[inline(always)]
	#[must_use]
	pub const fn len(&self) -> usize {
		self.len
	}

	/// Checks if the vector is empty, i.e. no elements are recorded.
	///
	/// Note that the internal buffer may still contain objects that have been "shadowed" by setting a smaller length with [`len`](Self::len).
	#[inline(always)]
	#[must_use]
	pub const fn is_empty(&self) -> bool {
		self.len() == 0x0
	}

	/// Checks if the vector is full, i.e. it cannot hold any more elements.
	#[inline(always)]
	#[must_use]
	pub const fn is_full(&self) -> bool {
		self.len() == N
	}
}

impl<T: Clone, const N: usize> Clone for Vec<T, N> {
	#[inline]
	fn clone(&self) -> Self {
		let mut buf = [const { MaybeUninit::uninit() }; N];

		unsafe {
			for i in 0x0..self.len() {
				let value = self.get_unchecked(i).clone();
				buf.get_unchecked_mut(i).write(value);
			}

			Self::from_raw_parts(buf, self.len())
		}
	}
}

impl<T: Debug, const N: usize> Debug for Vec<T, N> {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Debug::fmt(self.as_slice(), f)
	}
}

impl<T, const N: usize> Default for Vec<T, N> {
	#[inline(always)]
	fn default() -> Self {
		unsafe {
			let buf = [const { MaybeUninit::uninit() }; N];

			// SAFETY: The resulting slice is zero lengthed.
			Self::from_raw_parts(buf, 0x0)
		}
	}
}

impl<T, const N: usize> Drop for Vec<T, N> {
	#[inline(always)]
	fn drop(&mut self) {
		// Drop every element that is currently alive.

		let remaining = self.as_mut_slice();
		unsafe { drop_in_place(remaining) };

		// We do not need to ensure that `self` is in a
		// valid state after this call to `drop`.
		// `MaybeUninit` also doesn't run destructors.
	}
}

impl<T: Hash, const N: usize> Hash for Vec<T, N> {
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H) {
		for v in self {
			v.hash(state);
		}
	}
}

impl<T, I: SliceIndex<[T]>, const N: usize> Index<I> for Vec<T, N> {
	type Output = I::Output;

	#[inline(always)]
	fn index(&self, index: I) -> &Self::Output {
		self.get(index).unwrap()
	}
}

impl<T, I: SliceIndex<[T]>, const N: usize> IndexMut<I> for Vec<T, N> {
	#[inline(always)]
	fn index_mut(&mut self, index: I) -> &mut Self::Output {
		self.get_mut(index).unwrap()
	}
}
