// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

#[cfg(test)]
mod test;

use crate::decode::{Decode, Input};
use crate::encode::{Encode, Output, SizedEncode};

use alloc::boxed::Box;
use alloc::vec;
use core::borrow::{Borrow, BorrowMut};
use core::fmt::{self, Debug, Formatter};
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut, Index, IndexMut};
use core::ptr::{self, copy_nonoverlapping};
use core::slice::{self, SliceIndex};

/// Typed encode/decode slot.
///
/// This structure is intended as a lightweight byte buffer suitable for encoding a single, predefined type.
///
/// The methods [`write`](Self::write) and [`read`](Self::read) can be used to handle the buffer's contents.
/// Other methods also exist for accessing the contents directly.
///
/// # Examples
///
/// Create a slot for holding a `Request` enumeration:
///
/// ```rust
/// use oct::str;
/// use oct::encode::{Encode, Output, SizedEncode};
/// use oct::slot::Slot;
/// use oct::string::String;
///
/// #[derive(Debug, Encode)]
/// enum Request {
///     Join { username: String<0x40> },
///
///     Quit { username: String<0x40> },
///
///     SendMessage { message: String<0x80> },
/// }
///
/// let mut buf = Slot::with_capacity(0x100);
///
/// buf.write(&Request::Join { username: str!["epsiloneridani"] }).unwrap();
/// assert_eq!(buf.as_slice(), b"\0\0\x0E\0epsiloneridani");
///
/// // Do something with the slot...
/// ```
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
pub struct Slot<T> {
	buf: Box<[u8]>,
	len: usize,

	_ty: PhantomData<fn() -> T>,
}

#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T> Slot<T> {
	/// Allocates a new slot suitable for encodings.
	///
	/// The given capacity should be large enough to hold any expected encoding of `T`.
	///
	/// If `T` implements [`SizedEncode`], it is usually preferred to instead use the [`new`](Self::new) constructor as it reserves enough space for *any* arbitrary encoding (according to [`MAX_ENCODED_SIZE`](SizedEncode::MAX_ENCODED_SIZE)):
	#[inline]
	#[must_use]
	#[track_caller]
	pub fn with_capacity(cap: usize) -> Self {
		let buf = vec![0x00; cap].into();

		Self {
			buf,
			len: 0x0,

			_ty: PhantomData,
		}
	}

	/// Constructs a new slot from raw parts.
	///
	/// # Safety
	///
	/// The provided pointer `ptr` must be a valid reference to a mutable array of exactly `capacity` elements.
	/// This array must additionally be allocated with the global allocator using the default layout (i.e. with a specified alignement of `1`), and `len` must also be within the bounds of this array.
	#[inline]
	#[must_use]
	#[track_caller]
	pub unsafe fn from_raw_parts(ptr: *mut u8, cap: usize, len: usize) -> Self {
		let buf = {
			let buf = ptr::slice_from_raw_parts_mut(ptr, cap);

			unsafe { Box::from_raw(buf) }
		};

		Self {
			buf,
			len,

			_ty: PhantomData,
		}
	}

	/// Gets a pointer to the first byte of the slot's buffer.
	///
	/// Note that the all reads to bytes up to the amount specified by [`capacity`](Self::capacity) are valid (i.e. the bytes are always initialised).
	#[inline(always)]
	#[must_use]
	pub fn as_ptr(&self) -> *const u8 {
		self.buf.as_ptr()
	}

	/// Gets a mutable pointer to the first byte of the slot's buffer.
	///
	/// Note that the all reads to bytes up to the amount specified by [`capacity`](Self::capacity) are valid (i.e. the bytes are always initialised).
	#[inline(always)]
	#[must_use]
	pub fn as_mut_ptr(&mut self) -> *mut u8 {
		self.buf.as_mut_ptr()
	}

	/// Gets a slice of the slot's buffer.
	///
	/// The returned slice will only include the used part of the slot (as specified by [`len`](Self::len)).
	/// This is in contrast to [`as_mut_slice`](Self::as_mut_slice), which references the entire slot buffer.
	#[inline(always)]
	#[must_use]
	pub fn as_slice(&self) -> &[u8] {
		// SAFETY: References always contain valid values.
		unsafe { slice::from_raw_parts(self.as_ptr(), self.len()) }
	}

	/// Gets a mutable slice of the slot's buffer.
	///
	/// Contrary to [`as_slice`](Self::as_slice), this method returns a slice of the slot's **entire** buffer (as specified by [`capacity`](Self::capacity)).
	///
	/// Users should call [`set_len`](Self::set_len) if writing has modified the portion of used bytes.
	#[inline(always)]
	#[must_use]
	pub fn as_mut_slice(&mut self) -> &mut [u8] {
		// SAFETY: Our pointer is a valid reference.
		unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.capacity()) }
	}

	/// Copies data from a slice.
	///
	/// The length of `self` is updated to reflect the new data.
	///
	/// # Panics
	///
	/// If `self` cannot contain the entirety of `data` then this method will panic.
	#[inline]
	#[track_caller]
	pub fn copy_from_slice(&mut self, data: &[u8]) {
		let len = data.len();

		assert!(len <= self.capacity(), "slot cannot contain source slice");

		unsafe {
			let src = data.as_ptr();
			let dst = self.as_mut_ptr();

			// SAFETY: The pointers are guaranteed to be valid
			// and the length has been tested. `dst` also guaran-
			// tees exclusivity due to be a mutable reference.
			copy_nonoverlapping(src, dst, len);

			// SAFETY: We have asserted bounds.
			self.set_len_unchecked(len);
		}
	}

	/// Sets the length of the slot.
	///
	/// The provided size is checked before being written (i.e. `len` may not be greater than [`len`](Self::len)).
	/// For the same operation *without* these checks, see [`set_len_unchecked`](Self::set_len_unchecked).
	///
	/// # Panics
	///
	/// The provided size must not be greater than the slot's capacity.
	/// If this is the case, however, this method will panic.
	#[inline(always)]
	#[track_caller]
	pub fn set_len(&mut self, len: usize) {
		assert!(len <= self.capacity(), "cannot extend slot beyond capacity");

		// SAFETY: The length has been tested.
		unsafe { self.set_len_unchecked(len) }
	}

	/// Sets the length of the slot without checks.
	///
	/// The provided size is **not** tested before being written.
	/// For the same operation *with* checks, see [`set_len`](Self::set_len).
	///
	/// # Safety
	///
	/// The value of `len` may never be greater than the capacity of the slot.
	/// Exceeding this will yield undefined behaviour.
	#[inline(always)]
	#[track_caller]
	pub unsafe fn set_len_unchecked(&mut self, len: usize) {
		debug_assert!(len <= self.capacity(), "cannot extend slot beyond capacity");

		// SAFETY: The length has been guaranteed by the
		// caller.
		self.len = len;
	}

	/// Retrieves the capacity of the slot.
	///
	/// If the slot was constructed using [`new`](Self::new), this value is exactly equal to [`MAX_ENCODED_SIZE`](SizedEncode::MAX_ENCODED_SIZE).
	/// In other cases, however, this may either be greater or less than this value.
	#[inline(always)]
	#[must_use]
	pub fn capacity(&self) -> usize {
		self.buf.len()
	}

	/// Retrieves the length of the slot.
	///
	/// This value specifically denotes the length of the previous encoding (if any).
	///
	/// For retrieving the capacity of the slot, see [`capacity`](Self::capacity).
	#[inline(always)]
	#[must_use]
	pub fn len(&self) -> usize {
		self.len
	}

	/// Tests if the slot is empty.
	///
	/// This is strictly equivalent to testing if [`len`](Self::len) is null.
	#[inline(always)]
	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.len() == 0x0
	}
}

#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: Encode> Slot<T> {
	/// Encodes an object into the slot.
	///
	/// The object is encoded as by being passed to <code>&lt;T as [Encode]&gt;::[encode](Encode::encode)</code>.
	///
	/// # Errors
	///
	/// Any error that occurs during encoding is passed on and returned from this method.
	#[inline]
	#[track_caller]
	pub fn write(&mut self, value: &T) -> Result<(), T::Error> {
		let mut stream = Output::new(&mut self.buf);

		value.encode(&mut stream)?;

		let len = stream.position();
		self.set_len(len);

		Ok(())
	}
}

#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: Decode> Slot<T> {
	/// Decodes an object from the slot.
	///
	/// This is done as by passing the contained bytes to <code>&lt;T as [Decode]&gt;::[decode](Decode::decode)</code>.
	///
	/// Note that only the bytes specified by [`len`](Self::len) are passed in this call.
	/// See [`as_slice`](Self::as_slice) for more information.
	///
	/// # Errors
	///
	/// Any error that occurs during decoding is passed on and returned from this method.
	#[inline]
	#[track_caller]
	pub fn read(&self) -> Result<T, T::Error> {
		// We should only pass the used part of the slot to
		// `decode`.

		let mut stream = Input::new(&self.buf);

		let value = Decode::decode(&mut stream)?;
		Ok(value)
	}
}

#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: SizedEncode> Slot<T> {
	/// Allocates a new slot suitable for encoding.
	///
	/// The capacity of the slot is set so that any encoding of `T` may be stored (as specified by [`MAX_ENCODED_SIZE`](SizedEncode::MAX_ENCODED_SIZE)).
	/// See also the [`with_capacity`](Self::with_capacity) constructor.
	#[inline(always)]
	#[must_use]
	pub fn new() -> Self {
		Self::with_capacity(T::MAX_ENCODED_SIZE)
	}
}

/// See also [`as_mut_slice`](Self::as_mut_slice).
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T> AsMut<[u8]> for Slot<T> {
	#[inline(always)]
	fn as_mut(&mut self) -> &mut [u8] {
		self.as_mut_slice()
	}
}

/// See also [`as_slice`](Self::as_slice).
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T> AsRef<[u8]> for Slot<T> {
	#[inline(always)]
	fn as_ref(&self) -> &[u8] {
		self.as_slice()
	}
}

/// See also [`as_slice`](Self::as_slice).
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T> Borrow<[u8]> for Slot<T> {
	#[inline(always)]
	fn borrow(&self) -> &[u8] {
		self.as_slice()
	}
}

/// See also [`as_mut_slice`](Self::as_mut_slice).
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T> BorrowMut<[u8]> for Slot<T> {
	#[inline(always)]
	fn borrow_mut(&mut self) -> &mut [u8] {
		self.as_mut_slice()
	}
}

#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T> Debug for Slot<T> {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result { write!(f, "{:?}", self.as_slice()) }
}

#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: SizedEncode> Default for Slot<T> {
	#[inline(always)]
	fn default() -> Self {
		Self::new()
	}
}

#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T> Deref for Slot<T> {
	type Target = [u8];

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		self.as_slice()
	}
}

#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T> DerefMut for Slot<T> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.as_mut_slice()
	}
}

#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T, I: SliceIndex<[u8]>> Index<I> for Slot<T> {
	type Output = I::Output;

	#[inline(always)]
	fn index(&self, index: I) -> &Self::Output {
		self.get(index).unwrap()
	}
}

#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T, I: SliceIndex<[u8]>> IndexMut<I> for Slot<T> {
	#[inline(always)]
	fn index_mut(&mut self, index: I) -> &mut Self::Output {
		self.get_mut(index).unwrap()
	}
}

#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T> PartialEq<[u8]> for Slot<T> {
	#[inline(always)]
	fn eq(&self, other: &[u8]) -> bool {
		self.as_slice() == other
	}
}

#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T> PartialEq<&[u8]> for Slot<T> {
	#[inline(always)]
	fn eq(&self, other: &&[u8]) -> bool {
		self.as_slice() == *other
	}
}

#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T> PartialEq<&mut [u8]> for Slot<T> {
	#[inline(always)]
	fn eq(&self, other: &&mut [u8]) -> bool {
		self.as_slice() == *other
	}
}
