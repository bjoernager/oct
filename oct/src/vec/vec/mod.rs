// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

mod test;

use crate::decode::{self, Decode, DecodeBorrowed};
use crate::encode::{self, Encode, SizedEncode};
use crate::error::{CollectionDecodeError, ItemDecodeError, LengthError};
use crate::vec::{clone_to_uninit_in_range, IntoIter};

use core::borrow::{Borrow, BorrowMut};
use core::cmp::Ordering;
use core::fmt::{self, Debug, Formatter};
use core::hash::{Hash, Hasher};
use core::mem::{offset_of, ManuallyDrop, MaybeUninit};
use core::ops::{Deref, DerefMut, Index, IndexMut};
use core::ptr::{copy_nonoverlapping, drop_in_place};
use core::slice::{self, SliceIndex};

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

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
/// let v0 = Vec::<u8, 0x4>::try_from([0x3].as_slice()).unwrap();
/// let v1 = Vec::<u8, 0x4>::try_from([0x3, 0x2].as_slice()).unwrap();
/// let v2 = Vec::<u8, 0x4>::try_from([0x3, 0x2, 0x4].as_slice()).unwrap();
/// let v3 = Vec::<u8, 0x4>::try_from([0x3, 0x2, 0x4, 0x3].as_slice()).unwrap();
///
/// assert_eq!(size_of_val(&v0), size_of_val(&v1));
/// assert_eq!(size_of_val(&v0), size_of_val(&v2));
/// assert_eq!(size_of_val(&v0), size_of_val(&v3));
/// assert_eq!(size_of_val(&v1), size_of_val(&v2));
/// assert_eq!(size_of_val(&v1), size_of_val(&v3));
/// assert_eq!(size_of_val(&v2), size_of_val(&v3));
/// ```
pub struct Vec<T, const N: usize> {
	len: usize,
	buf: [MaybeUninit<T>; N],
}

impl<T, const N: usize> Vec<T, N> {
	/// Constructs a new slice from existing data.
	///
	/// This constructor takes inherits an existing array and converts it to a vector.
	/// If the provided array's length `M` is greater than `N`, then this function will panic at compile-time.
	#[inline(always)]
	#[must_use]
	#[track_caller]
	pub const fn new<const M: usize>(data: [T; M]) -> Self {
		const { assert!(M <= N, "cannot construct vector from array that is longer") };

		let data = ManuallyDrop::new(data);

		let len = M;

		let buf = if N == M {
			// Reuse the existing buffer.

			// SAFETY: `ManuallyDrop<[T; N]>` and
			// `[MaybeUninit<T>; N]` are both transparent to
			// `[T; N]`. `data` can also be forgotten as its
			// constructor is supressed. Also remember that
			// `N` and `M` have been tested to be equal.
			let ptr = &raw const data as *const [MaybeUninit<T>; N];

			unsafe { ptr.read() }
		} else {
			// Reallocate the buffer to `N` elements.

			// SAFETY: `ManuallyDrop<T>` is transparent to `T`.
			let data = unsafe { &*(&raw const data as *const [T; M]) };

			let mut buf = [const { MaybeUninit::uninit() }; N];

			let src = data.as_ptr();
			let dst = buf.as_mut_ptr() as *mut T;

			unsafe { copy_nonoverlapping(src, dst, len) };

			buf
		};

		// SAFETY: We have checked that the length is with-
		// in bounds.
		unsafe { Self::from_raw_parts(buf, len) }
	}

	/// Copies elements from a slice into a new vector.
	///
	/// This constructor copies the raw representation of `data` into a new allocation of `N` elements.
	///
	/// # Errors
	///
	/// If `self` cannot contain the entirety of `data`, then this method will return an error.
	#[inline]
	#[track_caller]
	pub const fn copy_from_slice(data: &[T]) -> Result<Self, LengthError>
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

		// SAFETY: We have checked that the length is with-
		// in bounds.
		let this = unsafe { Self::copy_from_slice_unchecked(data) };
		Ok(this)
	}

	/// Constructs a new slice from existing data without checking bounds.
	///
	/// For a safe version of this constructor, see [`copy_from_slice`](Self::copy_from_slice).
	///
	/// # Safety
	///
	/// The entirety of `data` must be able to fit into an array of `N` elements.
	#[inline(always)]
	#[track_caller]
	pub const unsafe fn copy_from_slice_unchecked(data: &[T]) -> Self
	where
		T: Copy,
	{
		let     len = data.len();
		let mut buf = [const { MaybeUninit::<T>::uninit() }; N];

		debug_assert!(len <= N, "cannot construct vector from slice that is longer");

		unsafe {
			let src = data.as_ptr();
			let dst = buf.as_mut_ptr() as *mut T;

			// SAFETY: `T` implements `Copy`.
			copy_nonoverlapping(src, dst, len);
		}

		// SAFETY: The relevant elements have been ini-
		// tialised, and `len` is not greater than `N` - as
		// guaranteed by the caller.
		unsafe { Self::from_raw_parts(buf, len) }
	}

	/// Constructs a size-constrained vector from raw parts.
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
	#[track_caller]
	pub const unsafe fn from_raw_parts(buf: [MaybeUninit<T>; N], len: usize) -> Self {
		debug_assert!(len <= N, "cannot construct vector longer than its capacity");

		Self { len, buf }
	}

	/// Generates a new vector referencing the elements of `self`.
	///
	/// # Safety
	///
	/// `len` may not be greater than `N`.
	#[inline]
	#[must_use]
	const unsafe fn each_ptr(data: *mut [MaybeUninit<T>; N], len: usize) -> [MaybeUninit<*mut T>; N] {
		// SAFETY: Note that this function does not take
		// any `&_` reference, so the caller can safely
		// reinterpret the returned array as `&mut _` muta-
		// ble  references.

		debug_assert!(len <= N);

		let mut buf = [const { MaybeUninit::uninit() }; N];

		let src_base = data as *mut T;
		let dst_base = buf.as_mut_ptr();

		let mut i = 0x0;
		while i < len {
			// SAFETY: `i` will always be less than `self.len`
			// and thereby within bounds as that variable is
			// also always less than `N`.
			let slot = unsafe { &mut *dst_base.add(i) };

			let value = unsafe { src_base.add(i) };

			slot.write(value);

			i += 0x1;
		}

		buf
	}

	/// Generates a new vector referencing the elements of `self`.
	#[inline(always)]
	#[must_use]
	pub const fn each_ref(&self) -> Vec<&T, N> {
		let buf = (&raw const self.buf).cast_mut();

		let len = self.len;
		let buf = unsafe { Self::each_ptr(buf, len) };

		// SAFETY: every relavent pointer (i.e. the first
		// `len`) have been initialised as valid refer-
		// ences. The destructor of the original `buf` does
		// also not show any substantial effects.
		let buf = unsafe { (&raw const buf as *const [MaybeUninit<&T>; N]).read() };

		unsafe { Vec::from_raw_parts(buf, len) }
	}

	/// Generates a new vector mutably-referencing the elements of `self`.
	#[inline(always)]
	#[must_use]
	pub const fn each_mut(&mut self) -> Vec<&mut T, N> {
		let len = self.len;
		let buf = unsafe { Self::each_ptr(&raw mut self.buf, len) };

		// SAFETY: every relavent pointer (i.e. the first
		// `len`) have been initialised as valid refer-
		// ences. The destructor of the original `buf` does
		// also not show any substantial effects.
		let buf = unsafe { (&raw const buf as *const [MaybeUninit<&mut T>; N]).read() };

		unsafe { Vec::from_raw_parts(buf, len) }
	}

	/// Unsafely sets the length of the vector.
	///
	/// The provided length is **not** tested in any way.
	///
	/// # Safety
	///
	/// The new length `len` may not be larger than `N`.
	///
	/// It is only valid to enlarge vectors if `T` supports being in a purely uninitialised state
	/// Such is permitted by e.g. [`MaybeUninit`].
	#[inline(always)]
	#[track_caller]
	pub const unsafe fn set_len(&mut self, len: usize) {
		debug_assert!(len <= N, "cannot set length past bounds");

		// SAFETY: The caller guarantees that `len` is not
		// be freaky.
		self.len = len
	}

	/// Returns the length of the vector.
	///
	/// This value denotes the current amount of elements contained in the vector, which may be any value between zero and `N` (inclusive).
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

	/// Gets a pointer to the first element.
	///
	/// The pointed-to element may not necessarily be initialised.
	/// See [`len`](Self::len) for more information.
	#[inline(always)]
	#[must_use]
	pub const fn as_ptr(&self) -> *const T {
		// SAFETY: `MaybeUninit<T>` is transparent to `T`.
		self.buf.as_ptr() as *const T
	}

	/// Gets a mutable pointer to the first element.
	///
	/// The pointed-to element may not necessarily be initialised.
	/// See [`len`](Self::len) for more information.
	#[inline(always)]
	#[must_use]
	pub const fn as_mut_ptr(&mut self) -> *mut T {
		// SAFETY: `MaybeUninit<T>` is transparent to `T`.
		self.buf.as_mut_ptr() as *mut T
	}

	/// Borrows the vector as a slice.
	///
	/// The range of the returned slice only includes the elements specified by [`len`](Self::len).
	#[inline(always)]
	#[must_use]
	pub const fn as_slice(&self) -> &[T] {
		let len = self.len();
		let ptr = self.as_ptr();

		unsafe { slice::from_raw_parts(ptr, len) }
	}

	/// Borrows the vector as a mutable slice.
	///
	/// The range of the returned slice only includes the elements specified by [`len`](Self::len).
	#[inline(always)]
	#[must_use]
	pub const fn as_mut_slice(&mut self) -> &mut [T] {
		let len = self.len();
		let ptr = self.as_mut_ptr();

		unsafe { slice::from_raw_parts_mut(ptr, len) }
	}

	/// Destructs the vector into its raw parts.
	///
	/// The returned parts are valid to pass back to [`from_raw_parts`](Self::from_raw_parts).
	#[inline(always)]
	#[must_use]
	pub const fn into_raw_parts(self) -> ([MaybeUninit<T>; N], usize) {
		let len = self.len;

		let this = ManuallyDrop::new(self);

		// FIXME(const-hack): We can't just use drop glue.
		let buf = {
			// SAFETY: `ManuallyDrop<T>` is transparent to `T`.
			let base = &raw const this as *const Self;

			let off = offset_of!(Self, buf);
			let ptr = unsafe { base.byte_add(off) as *const [MaybeUninit<T>; N] };

			// SAFETY: `this` will not be dropped with its own
			// destructor, so we can safely move out of it.
			unsafe { ptr.read() }
		};

		(buf, len)
	}

	/// Converts the vector into a boxed slice.
	///
	/// The vector is reallocated using the global allocator.
	#[cfg(feature = "alloc")]
	#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
	#[inline]
	#[must_use]
	#[track_caller]
	pub fn into_boxed_slice(self) -> Box<[T]> {
		let (buf, len) = self.into_raw_parts();

		let mut boxed = alloc::vec::Vec::with_capacity(len).into_boxed_slice();

		// SAFETY: `MaybeUninit<T>` is transparent to `T`.
		let src = buf.as_ptr() as *const T;

		let dst = boxed.as_mut_ptr();

		// SAFETY: `boxed` has been allocated with at least
		// `len` elements.
		unsafe { copy_nonoverlapping(src, dst, len) };

		boxed
	}

	/// Converts the vector into a dynamically-allocated vector.
	///
	/// The vector is reallocated using the global allocator.
	#[cfg(feature = "alloc")]
	#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
	#[inline(always)]
	#[must_use]
	#[track_caller]
	pub fn into_vec(self) -> alloc::vec::Vec<T> {
		self.into_boxed_slice().into_vec()
	}
}

impl<T, const N: usize> AsMut<[T]> for Vec<T, N> {
	#[inline(always)]
	fn as_mut(&mut self) -> &mut [T] {
		self
	}
}

impl<T, const N: usize> AsRef<[T]> for Vec<T, N> {
	#[inline(always)]
	fn as_ref(&self) -> &[T] {
		self
	}
}

impl<T, const N: usize> Borrow<[T]> for Vec<T, N> {
	#[inline(always)]
	fn borrow(&self) -> &[T] {
		self
	}
}

impl<T, const N: usize> BorrowMut<[T]> for Vec<T, N> {
	#[inline(always)]
	fn borrow_mut(&mut self) -> &mut [T] {
		self
	}
}

impl<T: Clone, const N: usize> Clone for Vec<T, N> {
	#[inline]
	fn clone(&self) -> Self {
		let     len = self.len;
		let mut buf = [const { MaybeUninit::uninit() }; N];

		// SAFETY: `MaybeUninit<T>` is transparent to `T`.
		let src = self.buf.as_ptr() as *const T;

		let dst = buf.as_mut_ptr() as *mut T;

		// SAFETY: The range
		//
		// 0x0..len
		//
		// defines in and of itself the bounds of valid
		// elements.
		unsafe { clone_to_uninit_in_range(src, dst, 0x0..len) };

		// SAFETY: The buffer has been initialised in the
		// provided range - which does not extend beyond
		// bounds.
		unsafe { Self::from_raw_parts(buf, len) }
	}
}

impl<T: Debug, const N: usize> Debug for Vec<T, N> {
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Debug::fmt(self.as_slice(), f)
	}
}

impl<T: Decode, const N: usize> Decode for Vec<T, N> {
	type Error = CollectionDecodeError<LengthError, ItemDecodeError<usize, T::Error>>;

	#[inline]
	fn decode(input: &mut decode::Input) -> Result<Self, Self::Error> {
		let Ok(len) = Decode::decode(input);

		if len > N {
			return Err(CollectionDecodeError::BadLength(LengthError {
				remaining: N,
				count:     len,
			}));
		}

		let mut buf = [const { MaybeUninit::<T>::uninit() }; N];

		for (i, slot) in buf[..len].iter_mut().enumerate() {
			let v = Decode::decode(input)
				.map_err(|e| CollectionDecodeError::BadItem(ItemDecodeError { index: i, error: e }))?;

			slot.write(v);
		}

		let this = unsafe { Self::from_raw_parts(buf, len) };
		Ok(this)
	}
}

impl<T: Decode, const N: usize> DecodeBorrowed<[T]> for Vec<T, N> { }

impl<T, const N: usize> Default for Vec<T, N> {
	#[inline(always)]
	fn default() -> Self {
		let buf = [const { MaybeUninit::uninit() }; N];

		// SAFETY: The resulting vector is zero lengthed
		// and does therefore not expose any uninitialised
		// objects.
		unsafe { Self::from_raw_parts(buf, Default::default()) }
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

impl<T, const N: usize> Drop for Vec<T, N> {
	#[inline(always)]
	fn drop(&mut self) {
		// Drop every element that is currently alive.

		let remaining = self.as_mut_slice() as *mut [T];

		// SAFETY: Mutable references always point to alive
		// and initialised objects.
		unsafe { drop_in_place(remaining) };

		// We do not need to ensure that `self` is in a
		// valid state after this call to `drop`.
	}
}

impl<T: Encode, const N: usize> Encode for Vec<T, N> {
	type Error = <[T] as Encode>::Error;

	#[inline(always)]
	fn encode(&self, output: &mut encode::Output) -> Result<(), Self::Error> {
		(**self).encode(output)
	}
}

impl<T: Eq, const N: usize> Eq for Vec<T, N> { }

impl<T, const N: usize> From<[T; N]> for Vec<T, N> {
	#[inline(always)]
	fn from(value: [T; N]) -> Self {
		Self::new(value)
	}
}

impl<T, const N: usize> FromIterator<T> for Vec<T, N> {
	#[inline]
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		let mut len = 0x0;
		let mut buf = [const { MaybeUninit::<T>::uninit() }; N];

		for (slot, value) in buf.iter_mut().zip(iter) {
			slot.write(value);

			len += 0x1;
		}

		// Drop the remaining elements.

		Self { len, buf }
	}
}

impl<T: Hash, const N: usize> Hash for Vec<T, N> {
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H) {
		(**self).hash(state)
	}
}

impl<T, I: SliceIndex<[T]>, const N: usize> Index<I> for Vec<T, N> {
	type Output = I::Output;

	#[inline(always)]
	#[track_caller]
	fn index(&self, index: I) -> &Self::Output {
		Index::index(&**self, index)
	}
}

impl<T, I: SliceIndex<[T]>, const N: usize> IndexMut<I> for Vec<T, N> {
	#[inline(always)]
	#[track_caller]
	fn index_mut(&mut self, index: I) -> &mut Self::Output {
		IndexMut::index_mut(&mut **self, index)
	}
}

impl<T, const N: usize> IntoIterator for Vec<T, N> {
	type Item = T;

	type IntoIter = IntoIter<T, N>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		IntoIter::new(self)
	}
}

impl<'a, T, const N: usize> IntoIterator for &'a Vec<T, N> {
	type Item = &'a T;

	type IntoIter = slice::Iter<'a, T>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<'a, T, const N: usize> IntoIterator for &'a mut Vec<T, N> {
	type Item = &'a mut T;

	type IntoIter = slice::IterMut<'a, T>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		self.iter_mut()
	}
}

impl<T: Ord, const N: usize> Ord for Vec<T, N> {
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering {
		(**self).cmp(&**other)
	}
}

impl<T: PartialEq<U>, U, const N: usize, const M: usize> PartialEq<Vec<U, M>> for Vec<T, N> {
	#[inline(always)]
	fn eq(&self, other: &Vec<U, M>) -> bool {
		**self == **other
	}

	#[expect(clippy::partialeq_ne_impl)]
	#[inline(always)]
	fn ne(&self, other: &Vec<U, M>) -> bool {
		**self != **other
	}
}

impl<T: PartialEq<U>, U, const N: usize, const M: usize> PartialEq<[U; M]> for Vec<T, N> {
	#[inline(always)]
	fn eq(&self, other: &[U; M]) -> bool {
		**self == *other
	}

	#[expect(clippy::partialeq_ne_impl)]
	#[inline(always)]
	fn ne(&self, other: &[U; M]) -> bool {
		**self != *other
	}
}

impl<T: PartialEq<U>, U, const N: usize> PartialEq<[U]> for Vec<T, N> {
	#[inline(always)]
	fn eq(&self, other: &[U]) -> bool {
		**self == *other
	}

	#[expect(clippy::partialeq_ne_impl)]
	#[inline(always)]
	fn ne(&self, other: &[U]) -> bool {
		**self != *other
	}
}

impl<T: PartialEq<U>, U, const N: usize> PartialEq<&[U]> for Vec<T, N> {
	#[inline(always)]
	fn eq(&self, other: &&[U]) -> bool {
		**self == **other
	}

	#[expect(clippy::partialeq_ne_impl)]
	#[inline(always)]
	fn ne(&self, other: &&[U]) -> bool {
		**self != **other
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: PartialEq<U>, U, const N: usize> PartialEq<alloc::vec::Vec<U>> for Vec<T, N> {
	#[inline(always)]
	fn eq(&self, other: &alloc::vec::Vec<U>) -> bool {
		**self == **other
	}

	#[expect(clippy::partialeq_ne_impl)]
	#[inline(always)]
	fn ne(&self, other: &alloc::vec::Vec<U>) -> bool {
		**self != **other
	}
}

impl<T: PartialOrd, const N: usize, const M: usize> PartialOrd<Vec<T, M>> for Vec<T, N> {
	#[inline(always)]
	fn partial_cmp(&self, other: &Vec<T, M>) -> Option<Ordering> {
		(**self).partial_cmp(&**other)
	}

	#[inline(always)]
	fn lt(&self, other: &Vec<T, M>) -> bool {
		**self < **other
	}

	#[inline(always)]
	fn le(&self, other: &Vec<T, M>) -> bool {
		**self <= **other
	}

	#[inline(always)]
	fn gt(&self, other: &Vec<T, M>) -> bool {
		**self > **other
	}

	#[inline(always)]
	fn ge(&self, other: &Vec<T, M>) -> bool {
		**self >= **other
	}
}

impl<T: SizedEncode, const N: usize> SizedEncode for Vec<T, N> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE * N;
}

impl<T: Copy, const N: usize> TryFrom<&[T]> for Vec<T, N> {
	type Error = LengthError;

	#[inline(always)]
	fn try_from(value: &[T]) -> Result<Self, Self::Error> {
		Self::copy_from_slice(value)
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
		value.into_vec()
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: PartialEq<U>, U, const N: usize> PartialEq<Vec<U, N>> for alloc::vec::Vec<T> {
	#[inline(always)]
	fn eq(&self, other: &Vec<U, N>) -> bool {
		**self == **other
	}
}
