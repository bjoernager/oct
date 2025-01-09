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
use crate::vec::Vec;

use core::fmt::{self, Debug, Display, Formatter};
use core::hash::{Hash, Hasher};
use core::ops::{Index, IndexMut};
use core::slice::SliceIndex;

// Comparison facilities:
mod cmp;

// Encode/decode facilities:
mod code;

// Conversion facilities:
mod conv;

/// String container with maximum length.
///
/// This is in contrast to [`str`] and the standard library's [`String`](alloc::string::String) type -- both of which have no size limit in practice.
///
/// The string itself is encoded in UTF-8 for interoperability wtih Rust's standard string facilities, and partly due to memory concerns.
/// Keep in mind that the size limit specified by `N` then denotes *octets* (or "bytes") and **not** *characters* -- i.e. a value of `8` may translate to anywhere between two and eight characters due to variable-length encoding.
///
/// See [`Vec`] for an equivalent alternative to [`alloc::vec::Vec`].
///
/// # Examples
///
/// All instances of this type have the same size if the value of `N` is also the same.
/// Therefore, the following four strings have -- despite their different contents -- the same total size.
///
/// ```rust
/// use oct::string::String;
/// use std::str::FromStr;
///
/// let str0 = String::<0x40>::default(); // Empty string.
/// let str1 = String::<0x40>::from_str("Hello there!").unwrap();
/// let str2 = String::<0x40>::from_str("أنا من أوروپا").unwrap();
/// let str3 = String::<0x40>::from_str("COGITO ERGO SUM").unwrap();
///
/// assert_eq!(size_of_val(&str0), size_of_val(&str1));
/// assert_eq!(size_of_val(&str0), size_of_val(&str2));
/// assert_eq!(size_of_val(&str0), size_of_val(&str3));
/// assert_eq!(size_of_val(&str1), size_of_val(&str2));
/// assert_eq!(size_of_val(&str1), size_of_val(&str3));
/// assert_eq!(size_of_val(&str2), size_of_val(&str3));
/// ```
#[derive(Clone, Default, Eq)]
#[repr(transparent)]
pub struct String<const N: usize>(Vec<u8, N>);

impl<const N: usize> String<N> {
	/// Constructs a new, fixed-size string.
	///
	/// Note that string is not required to completely fill out its size-constraint.
	///
	/// # Errors
	///
	/// If the internal buffer cannot contain the entirety of `s`, then an error is returned.
	#[inline]
	pub const fn new(s: &str) -> Result<Self, LengthError> {
		if s.len() > N {
			return Err(LengthError {
				remaining: N,
				count:     s.len(),
			});
		}

		let this = unsafe { Self::from_utf8_unchecked(s.as_bytes()) };
		Ok(this)
	}

	/// Returns the length of the string.
	///
	/// This does not necessarily equate to the value of `N`, as the internal buffer may be used but partially.
	///
	/// Also remember that the returned value only denotes the octet count and not characters, graphemes, etc.
	#[inline(always)]
	#[must_use]
	pub const fn len(&self) -> usize {
		self.0.len()
	}

	/// Checks if the string is empty, i.e. no characters are contained.
	#[inline(always)]
	#[must_use]
	pub const fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	/// Checks if the string is full, i.e. it cannot hold any more characters.
	#[inline(always)]
	#[must_use]
	pub const fn is_full(&self) -> bool {
		self.0.is_full()
	}
}

impl<const N: usize> Debug for String<N> {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Debug::fmt(self.as_str(), f)
	}
}

impl<const N: usize> Display for String<N> {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(self.as_str(), f)
	}
}

impl<const N: usize> FromIterator<char> for String<N> {
	#[inline]
	fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
		let mut buf = [0x00; N];
		let mut len = 0x0;

		for c in iter {
			let rem = N - len;
			let req = c.len_utf8();

			if rem < req { break }

			let start = len;
			let stop  = start + req;

			c.encode_utf8(&mut buf[start..stop]);

			len += req;
		}

		// SAFETY: All octets are initialised and come from
		// `char::encode_utf8`.
		unsafe { Self::from_raw_parts(buf, len) }
	}
}

impl<const N: usize> Hash for String<N> {
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.as_str().hash(state)
	}
}

impl<I: SliceIndex<str>, const N: usize> Index<I> for String<N> {
	type Output	= I::Output;

	#[inline(always)]
	fn index(&self, index: I) -> &Self::Output {
		self.get(index).unwrap()
	}
}

impl<I: SliceIndex<str>, const N: usize> IndexMut<I> for String<N> {
	#[inline(always)]
	fn index_mut(&mut self, index: I) -> &mut Self::Output {
		self.get_mut(index).unwrap()
	}
}
