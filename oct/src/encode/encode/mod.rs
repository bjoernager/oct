// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

mod test;

use crate::__cold_path;
use crate::encode::Output;
use crate::error::{
	CollectionEncodeError,
	EnumEncodeError,
	IsizeEncodeError,
	ItemEncodeError,
	RefCellEncodeError,
	UsizeEncodeError,
	Utf8Error,
};

use core::cell::{Cell, LazyCell, RefCell, UnsafeCell};
use core::convert::Infallible;
use core::ffi::{CStr, c_void};
use core::hash::BuildHasher;
use core::marker::{PhantomData, PhantomPinned};
use core::net::{
	IpAddr,
	Ipv4Addr,
	Ipv6Addr,
	SocketAddr,
	SocketAddrV4,
	SocketAddrV6,
};
use core::num::{Saturating, Wrapping};
use core::ops::{
	Bound,
	Range,
	RangeFrom,
	RangeFull,
	RangeInclusive,
	RangeTo,
	RangeToInclusive,
};
use core::time::Duration;

#[cfg(feature = "alloc")]
use {
	alloc::borrow::{Cow, ToOwned},
	alloc::boxed::Box,
	alloc::collections::{BinaryHeap, LinkedList},
	alloc::ffi::CString,
	alloc::rc::{self, Rc},
	alloc::string::String,
	alloc::vec::Vec,
};

#[cfg(all(feature = "alloc", target_has_atomic = "ptr"))]
use alloc::sync::{self, Arc};

#[cfg(feature = "std")]
use {
	core::str,

	std::collections::{HashMap, HashSet},
	std::ffi::{OsStr, OsString},
	std::sync::{LazyLock, Mutex, RwLock},
	std::time::{SystemTime, UNIX_EPOCH},
};

/// Denotes a type capable of being encoded.
///
/// It is recommended to simply derive this trait for custom types (see the [`Encode`](derive@crate::encode::Encode) macro).
/// The trait can, of course, also just be manually implemented.
///
/// If all possible encodings have a known, maximum size, then the [`SizedEncode`](crate::encode::SizedEncode) trait should be considered as well.
///
/// *See also the [`encode`](crate::encode) module's documentation on how to use encodings.*
///
/// # Examples
///
/// A manual implementation of `Encode`:
///
/// ```rust
/// // Manual implementation of custom type. This im-
/// // plementation is equivalent to what would have
/// // been derived.
///
/// use oct::encode::{Encode, Output};
/// use core::convert::Infallible;
///
/// struct Foo {
///     bar: u16,
///     baz: f32,
/// }
///
/// impl Encode for Foo {
///     // Both `u16` and `f32` encode infallibly.
///
///     type Error = Infallible;
///
///     #[track_caller] // (1)
///     fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
///         // Encode fields using chaining.
///
///         self.bar.encode(output)?;
///         self.baz.encode(output)?;
///
///         Ok(())
///     }
/// }
/// ```
///
/// <sub>(1): In cases where `output` is empty, `encode` is expected to panic. Adding `track_caller` makes this "easier" to debug without analysing the stack trace.</sub>
#[doc(alias("Serialise", "Serialize"))]
pub trait Encode {
	/// The type returned in case of error.
	type Error;

	/// Encodes `self` into the provided output.
	///
	/// # Errors
	///
	/// If encoding fails, such as if `self` is unencodable, then an error should be returned.
	///
	/// <sub>Note that types should usually only define encodable variants, unless its variants are platform-dependent, in which case the largest, portable subset of variants should then be encodable.</sub>
	///
	/// # Panics
	///
	/// If `output` cannot contain the entirety of the resulting encoding, then this method should panic.
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error>;
}

impl<T: Encode + ?Sized> Encode for &T {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		T::encode(self, output)
	}
}

impl<T: Encode + ?Sized> Encode for &mut T {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		T::encode(self, output)
	}
}

/// Implemented for tuples with up to twelve members.
#[cfg_attr(feature = "unstable-docs", doc(fake_variadic))]
impl<T: Encode> Encode for (T,) {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		self.0.encode(output)
	}
}

impl<T: Encode, const N: usize> Encode for [T; N] {
	type Error = CollectionEncodeError<Infallible, ItemEncodeError<usize, T::Error>>;

	/// Encodes each element sequentially.
	///
	/// The length `N ` is hard-coded into the type and is therefore not encoded.
	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		for (i, v) in self.iter().enumerate() {
			v
				.encode(output)
				.map_err(|e| CollectionEncodeError::BadItem(ItemEncodeError { index: i, error: e }))?;
		}

		Ok(())
	}
}

impl<T: Encode> Encode for [T] {
	type Error = CollectionEncodeError<UsizeEncodeError, ItemEncodeError<usize, T::Error>>;

	/// Encodes each element sequentially with an extra length specifier (of type [`usize`]) prepended first.
	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		self
			.len()
			.encode(output)
			.map_err(CollectionEncodeError::BadLength)?;

		for (i,v) in self.iter().enumerate() {
			v
				.encode(output)
				.map_err(|e| CollectionEncodeError::BadItem(ItemEncodeError { index: i, error: e }))?;
		}

		Ok(())
	}
}

#[cfg(all(feature = "alloc", target_has_atomic = "ptr"))]
#[cfg_attr(feature = "unstable-docs", doc(cfg(all(feature = "alloc", target_has_atomic = "ptr"))))]
impl<T: Encode + ?Sized> Encode for Arc<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		T::encode(self, output)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "alloc")))]
impl<T: Encode> Encode for BinaryHeap<T> {
	type Error = <[T] as Encode>::Error;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		self.as_slice().encode(output)
	}
}

impl Encode for bool {
	type Error = <u8 as Encode>::Error;

	/// Encodes the raw representationf of the boolean.
	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		u8::from(*self).encode(output)
	}
}

impl<T: Encode> Encode for Bound<T> {
	type Error = EnumEncodeError<u8, T::Error>;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		match *self {
			Self::Included(ref bound) => {
				let Ok(()) = 0x0u8.encode(output);
				bound.encode(output).map_err(EnumEncodeError::BadField)?;
			}

			Self::Excluded(ref bound) => {
				let Ok(()) = 0x1u8.encode(output);
				bound.encode(output).map_err(EnumEncodeError::BadField)?;
			}

			Self::Unbounded => {
				let Ok(()) = 0x2u8.encode(output);
			}
		}

		Ok(())
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "alloc")))]
impl<T: Encode + ?Sized> Encode for Box<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		T::encode(self, output)
	}
}

impl Encode for CStr {
	type Error = <[u8] as Encode>::Error;

	/// Encodes the string identically to [a byte slice](slice) containing the string's byte values **excluding** the null terminator.
	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		self.to_bytes().encode(output)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "alloc")))]
impl Encode for CString {
	type Error = <CStr as Encode>::Error;

	/// See the the implementation of [`CStr`].
	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		self.as_c_str().encode(output)
	}
}

impl Encode for c_void {
	type Error = Infallible;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, _output: &mut Output) -> Result<(), Self::Error> {
		// NOTE: Contrary to `Infallible` and/or `!`,
		// `c_void` *can* actually be constructed (although
		// only by the the standard library).
		unreachable!();
	}
}

impl<T: Copy + Encode> Encode for Cell<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		self.get().encode(output)
	}
}

impl Encode for char {
	type Error = <u32 as Encode>::Error;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		u32::from(*self).encode(output)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "alloc")))]
impl<T: Encode + ToOwned + ?Sized> Encode for Cow<'_, T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		T::encode(self, output)
	}
}

impl Encode for Duration {
	type Error = Infallible;

	/// Encodes the duration's seconds and nanoseconds counters sequentially.
	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		self.as_secs().encode(output)?;
		self.subsec_nanos().encode(output)?;

		Ok(())
	}
}

#[cfg(feature = "f128")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "f128")))]
impl Encode for f128 {
	type Error = Infallible;

	#[inline]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		output.write(&self.to_le_bytes());

		Ok(())
	}
}

#[cfg(feature = "f16")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "f16")))]
impl Encode for f16 {
	type Error = Infallible;

	#[inline]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		output.write(&self.to_le_bytes());

		Ok(())
	}
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "std")))]
impl<K, V, S, E> Encode for HashMap<K, V, S>
where
	K: Encode<Error = E>,
	V: Encode<Error: Into<E>>,
	S: BuildHasher,
{
	type Error = E;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		for (key, value) in self {
			key.encode(output)?;

			value
				.encode(output)
				.map_err(Into::<E>::into)?;
		}

		Ok(())
	}
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "std")))]
impl<K, S> Encode for HashSet<K, S>
where
	K: Encode,
	S: BuildHasher,
{
	type Error = K::Error;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		for key in self {
			key.encode(output)?;
		}

		Ok(())
	}
}

// Especially useful for `Result<T, Infallible>`.
// **If** that is even needed, of course.
impl Encode for Infallible {
	type Error = Self;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, _output: &mut Output) -> Result<(), Self::Error> {
		unreachable!()
	}
}

impl Encode for IpAddr {
	type Error = EnumEncodeError<<u8 as Encode>::Error, Infallible>;

	/// Encodes a the address with a preceding discriminant denoting the IP version of the address (i.e. `4` for IPv4 and `6` for IPv6).
	///
	/// See also the implementations of [`Ipv4Addr`] and [`Ipv6Addr`].
	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		// The discriminant here is the IP version.

		match *self {
			Self::V4(ref addr) => {
				0x4u8.encode(output).map_err(EnumEncodeError::BadDiscriminant)?;
				let Ok(()) = addr.encode(output);
			}

			Self::V6(ref addr) => {
				0x6u8.encode(output).map_err(EnumEncodeError::BadDiscriminant)?;
				let Ok(()) = addr.encode(output);
			}
		}

		Ok(())
	}
}

impl Encode for Ipv4Addr {
	type Error = Infallible;

	/// Encodes the address's bits in big-endian.
	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		let value = self.to_bits();
		value.encode(output)
	}
}

impl Encode for Ipv6Addr {
	type Error = Infallible;

	/// Encodes the address's bits in big-endian.
	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		let value = self.to_bits();
		value.encode(output)
	}
}

impl Encode for isize {
	type Error = IsizeEncodeError;

	/// Casts `self` to [`i16`] and encodes the result.
	#[inline]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		let value = *self;

		if value <= i16::MAX as Self {
			let value = value as i16;
			let Ok(()) = value.encode(output);

			Ok(())
		} else {
			__cold_path();
			Err(IsizeEncodeError(value))
		}
	}
}

impl<T: Encode> Encode for LazyCell<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		T::encode(self, output)
	}
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "std")))]
impl<T: Encode> Encode for LazyLock<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		T::encode(self, output)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "alloc")))]
impl<T: Encode> Encode for LinkedList<T> {
	type Error = CollectionEncodeError<UsizeEncodeError, ItemEncodeError<usize, T::Error>>;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		self
			.len()
			.encode(output)
			.map_err(CollectionEncodeError::BadLength)?;

		for (i, v) in self.iter().enumerate() {
			v
				.encode(output)
				.map_err(|e| CollectionEncodeError::BadItem(ItemEncodeError { index: i, error: e }))?;
		}

		Ok(())
	}
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "std")))]
impl<T: Encode + ?Sized> Encode for Mutex<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		self
			.lock()
			.unwrap_or_else(std::sync::PoisonError::into_inner)
			.encode(output)
	}
}

impl<T: Encode> Encode for Option<T> {
	type Error = EnumEncodeError<<bool as Encode>::Error, T::Error>;

	/// Encodes a sign denoting the optional's variant.
	/// This is `false` for `None` instances and `true` for `Some` instances.
	///
	/// If `Some`, then the contained value is encoded after this sign..
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		let Ok(()) = self.is_some().encode(output);

		if let Some(ref value) = *self {
			value.encode(output).map_err(EnumEncodeError::BadField)?;
		};

		Ok(())
	}
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "std")))]
impl Encode for OsStr {
	type Error = CollectionEncodeError<UsizeEncodeError, Utf8Error>;

	/// Encodes the OS-specific string as a normal string.
	///
	/// `OsStr` is losely defined by Rust as being superset of the standard, UTF-8 `str`.
	/// In other words, all `str` object can directly be translated to `OsStr`, but the inverse operation is not necessarily possible.
	///
	/// # Errors
	///
	/// This implementation will yield an error if the string `self` contains any non-UTF-8 octets.
	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		let data = self.as_encoded_bytes();

		let s = match str::from_utf8(data) {
			Ok(s) => s,

			Err(e) => {
				let i = e.valid_up_to();
				let c = data[i];

				return Err(
					CollectionEncodeError::BadItem(
						Utf8Error { value: c, index: i },
					),
				);
			}
		};

		if let Err(CollectionEncodeError::BadLength(e)) = s.encode(output) {
			Err(CollectionEncodeError::BadLength(e))
		} else {
			Ok(())
		}
	}
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "std")))]
impl Encode for OsString {
	type Error = <OsStr as Encode>::Error;

	/// Encodes the OS-specific string as a normal string.
	///
	/// See [`OsStr`]'s implementation of `Encode` for more information.
	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		self.as_os_str().encode(output)
	}
}

impl<T: ?Sized> Encode for PhantomData<T> {
	type Error = Infallible;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, _output: &mut Output) -> Result<(), Self::Error> {
		Ok(())
	}
}

impl Encode for PhantomPinned {
	type Error = Infallible;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, _output: &mut Output) -> Result<(), Self::Error> {
		Ok(())
	}
}

impl<T: Encode> Encode for Range<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		self.start.encode(output)?;
		self.end.encode(output)?;

		Ok(())
	}
}

impl<T: Encode> Encode for RangeFrom<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		self.start.encode(output)
	}
}

impl Encode for RangeFull {
	type Error = Infallible;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, _output: &mut Output) -> Result<(), Self::Error> {
		Ok(())
	}
}

impl<T: Encode> Encode for RangeInclusive<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		self.start().encode(output)?;
		self.end().encode(output)?;

		Ok(())
	}
}

impl<T: Encode> Encode for RangeTo<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		self.end.encode(output)
	}
}

impl<T: Encode> Encode for RangeToInclusive<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		self.end.encode(output)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "alloc")))]
impl<T: Encode + ?Sized> Encode for Rc<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		T::encode(self, output)
	}
}

impl<T: Encode + ?Sized> Encode for RefCell<T> {
	type Error = RefCellEncodeError<T::Error>;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		let value = self.try_borrow().map_err(RefCellEncodeError::BadBorrow)?;
		T::encode(&value, output).map_err(RefCellEncodeError::BadValue)?;

		Ok(())
	}
}

impl<T, E, Err> Encode for Result<T, E>
where
	T: Encode<Error = Err>,
	E: Encode<Error: Into<Err>>,
{
	type Error = EnumEncodeError<<bool as Encode>::Error, Err>;

	/// Encodes a sign denoting the result's variant.
	/// This is `false` for `Ok` instances and `true` for `Err` instances.
	///
	/// If `Ok`, then the contained value is encoded after this sign.
	#[inline]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		let Ok(()) = self.is_err().encode(output);

		match *self {
			Ok(ref v) => {
				v.encode(output).map_err(EnumEncodeError::BadField)?;
			}

			Err(ref e) => {
				e.encode(output)
					.map_err(Into::into)
					.map_err(EnumEncodeError::BadField)?;
			}
		};

		Ok(())
	}
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "std")))]
impl<T: Encode + ?Sized> Encode for RwLock<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		self
			.read()
			.or_else(|e| Ok(e.into_inner()))?
			.encode(output)
	}
}

impl<T: Encode> Encode for Saturating<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		self.0.encode(output)
	}
}

impl Encode for SocketAddr {
	type Error = EnumEncodeError<<u8 as Encode>::Error, Infallible>;

	/// This implementation encoded as discriminant denoting the IP version of the address (i.e. `4` for IPv4 and `6` for IPv6).
	/// This is then followed by the respective address' own encoding (either [`SocketAddrV4`] or [`SocketAddrV6`]).
	#[inline]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		// The discriminant here is the IP version.

		match *self {
			Self::V4(ref addr) => {
				0x4u8.encode(output).map_err(EnumEncodeError::BadDiscriminant)?;
				let Ok(()) = addr.encode(output);
			}

			Self::V6(ref addr) => {
				0x6u8.encode(output).map_err(EnumEncodeError::BadDiscriminant)?;
				let Ok(()) = addr.encode(output);
			}
		}

		Ok(())
	}
}

impl Encode for SocketAddrV4 {
	type Error = Infallible;

	/// Encodes the address's bits followed by the port number, both of which in big-endian.
	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		self.ip().encode(output)?;
		self.port().encode(output)?;

		Ok(())
	}
}

impl Encode for SocketAddrV6 {
	type Error = Infallible;

	/// Encodes the address's bits followed by the port number, flow information, and scope identifier -- all of which in big-endian.
	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		self.ip().encode(output)?;
		self.port().encode(output)?;
		self.flowinfo().encode(output)?;
		self.scope_id().encode(output)?;

		Ok(())
	}
}

impl Encode for str {
	type Error = <[u8] as Encode>::Error;

	/// Encodes the string identically to [a byte slice](slice) containing the string's byte values.
	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		self.as_bytes().encode(output)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "alloc")))]
impl Encode for String {
	type Error = <str as Encode>::Error;

	/// See [`prim@str`].
	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		self.as_str().encode(output)
	}
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "std")))]
impl Encode for SystemTime {
	type Error = <i64 as Encode>::Error;

	/// Encodes the time point as the nearest, signed UNIX timestamp.
	///
	/// Examples of some timestamps and their encodings include:
	///
	/// | ISO 8601                    | UNIX / oct |
	/// | :-------------------------- | -------------: |
	/// | `2024-11-03T12:02:01+01:00` |    +1730631721 |
	/// | `1989-06-03T20:00:00+09:00` |      +13258800 |
	/// | `1970-01-01T00:00:00Z`      |             +0 |
	/// | `1945-05-04T18:30:00+02:00` |     -778231800 |
	#[expect(clippy::cast_possible_wrap)]
	#[inline]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		let time = if *self >= UNIX_EPOCH {
			let duration = self
				.duration_since(UNIX_EPOCH)
				.expect("cannot compute duration since the epoch");

				duration.as_secs() as i64
		} else {
			let duration = UNIX_EPOCH
				.duration_since(*self)
				.expect("cannot compute duration until the epoch");

			0x0 - duration.as_secs() as i64
		};

		time.encode(output)?;
		Ok(())
	}
}

impl Encode for () {
	type Error = Infallible;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, _output: &mut Output) -> Result<(), Self::Error> {
		Ok(())
	}
}

impl<T: Encode> Encode for UnsafeCell<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		// SAFETY: The pointer returned by `Self::get` is
		// valid for reading for the lifetime of `self`.
		let value = unsafe { &*self.get() };

		value.encode(output)
	}
}

impl Encode for usize {
	type Error = UsizeEncodeError;

	/// Casts `self` to [`u16`] and encodes the result.
	#[inline]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		let value = *self;

		if value <= u16::MAX as Self {
			let value = value as u16;
			let Ok(()) = value.encode(output);

			Ok(())
		} else {
			__cold_path();
			Err(UsizeEncodeError(value))
		}
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "alloc")))]
impl<T: Encode> Encode for Vec<T> {
	type Error = <[T] as Encode>::Error;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		self.as_slice().encode(output)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "alloc")))]
impl<T: Encode + ?Sized> Encode for rc::Weak<T> {
	type Error = <Option<Rc::<T>> as Encode>::Error;

	/// Encodes the value referenced by `Self`.
	///
	/// If the reference counter is broken (i.e. the underlying item has been dropped), then no value will be encoded.
	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		self.upgrade().encode(output)
	}
}

#[cfg(all(feature = "alloc", target_has_atomic = "ptr"))]
#[cfg_attr(feature = "unstable-docs", doc(cfg(all(feature = "alloc", target_has_atomic = "ptr"))))]
impl<T: Encode + ?Sized> Encode for sync::Weak<T> {
	type Error = <Option<Arc::<T>> as Encode>::Error;

	/// Encodes the value referenced by `Self`.
	///
	/// If the reference counter is broken (i.e. the underlying item has been dropped), then no value will be encoded.
	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		self.upgrade().encode(output)
	}
}

impl<T: Encode> Encode for Wrapping<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		self.0.encode(output)
	}
}

macro_rules! impl_numeric {
	($ty:ty$(,)?) => {
		impl ::oct::encode::Encode for $ty {
			type Error = ::core::convert::Infallible;

			#[inline]
			#[track_caller]
			fn encode(&self, output: &mut ::oct::encode::Output) -> ::core::result::Result<(), Self::Error> {
				output.write(&self.to_le_bytes());

				Ok(())
			}
		}
	};
}

macro_rules! impl_tuple {
	{
		$capture:ident: $ty:ident,
		$($extra_captures:ident: $extra_tys:ident),*$(,)?
	} => {
		#[doc(hidden)]
		impl<$ty, $($extra_tys,)* E> ::oct::encode::Encode for ($ty, $($extra_tys,)*)
		where
			$ty: ::oct::encode::Encode<Error = E>,
			$($extra_tys: ::oct::encode::Encode<Error: ::core::convert::Into<E>>,)*
		{
			type Error = E;

			#[inline(always)]
			#[track_caller]
			fn encode(&self, output: &mut ::oct::encode::Output) -> ::core::result::Result<(), Self::Error> {
				let (ref $capture, $(ref $extra_captures,)*) = *self;

				<$ty as ::oct::encode::Encode>::encode($capture, output)?;

				$(
					$extra_captures.encode(output)
						.map_err(::core::convert::Into::<E>::into)?;
				)*

				::core::result::Result::Ok(())
			}
		}
	};
}

macro_rules! impl_non_zero {
	($ty:ty$(,)?) => {
		impl ::oct::encode::Encode for ::core::num::NonZero<$ty> {
			type Error = <$ty as ::oct::encode::Encode>::Error;

			#[inline(always)]
			#[track_caller]
			fn encode(&self, output: &mut ::oct::encode::Output) -> ::core::result::Result<(), Self::Error> {
				self.get().encode(output)
			}
		}
	};
}

macro_rules! impl_atomic {
	{
		width: $width:literal,
		ty: $ty:ty,
		atomic_ty: $atomic_ty:ty$(,)?
	} => {
		#[cfg(target_has_atomic = $width)]
		#[cfg_attr(feature = "unstable-docs", doc(cfg(target_has_atomic = $width)))]
		impl ::oct::encode::Encode for $atomic_ty {
			type Error = <$ty as ::oct::encode::Encode>::Error;

			/// Encodes the atomic with the same scheme as that of the atomic type's primitive counterpart.
			///
			/// The atomic object itself is read with the [`Relaxed`](core::sync::atomic::Ordering) ordering scheme.
			#[inline(always)]
			#[track_caller]
			fn encode(&self, output: &mut ::oct::encode::Output) -> ::core::result::Result<(), Self::Error> {
				use ::core::sync::atomic::Ordering;

				self.load(Ordering::Relaxed).encode(output)
			}
		}
	};
}

//impl_numeric!(f128);
//impl_numeric!(f16);
impl_numeric!(f32);
impl_numeric!(f64);
impl_numeric!(i128);
impl_numeric!(i16);
impl_numeric!(i32);
impl_numeric!(i64);
impl_numeric!(i8);
impl_numeric!(u128);
impl_numeric!(u16);
impl_numeric!(u32);
impl_numeric!(u64);
impl_numeric!(u8);

impl_tuple! {
	value0: T0,
	value1: T1,
}

impl_tuple! {
	value0: T0,
	value1: T1,
	value2: T2,
}

impl_tuple! {
	value0: T0,
	value1: T1,
	value2: T2,
	value3: T3,
}

impl_tuple! {
	value0: T0,
	value1: T1,
	value2: T2,
	value3: T3,
	value4: T4,
}

impl_tuple! {
	value0: T0,
	value1: T1,
	value2: T2,
	value3: T3,
	value4: T4,
	value5: T5,
}

impl_tuple! {
	value0: T0,
	value1: T1,
	value2: T2,
	value3: T3,
	value4: T4,
	value5: T5,
	value6: T6,
}

impl_tuple! {
	value0: T0,
	value1: T1,
	value2: T2,
	value3: T3,
	value4: T4,
	value5: T5,
	value6: T6,
	value7: T7,
}

impl_tuple! {
	value0: T0,
	value1: T1,
	value2: T2,
	value3: T3,
	value4: T4,
	value5: T5,
	value6: T6,
	value7: T7,
	value8: T8,
}

impl_tuple! {
	value0: T0,
	value1: T1,
	value2: T2,
	value3: T3,
	value4: T4,
	value5: T5,
	value6: T6,
	value7: T7,
	value8: T8,
	value9: T9,
}

impl_tuple! {
	value0:  T0,
	value1:  T1,
	value2:  T2,
	value3:  T3,
	value4:  T4,
	value5:  T5,
	value6:  T6,
	value7:  T7,
	value8:  T8,
	value9:  T9,
	value10: T10,
}

impl_tuple! {
	value0:  T0,
	value1:  T1,
	value2:  T2,
	value3:  T3,
	value4:  T4,
	value5:  T5,
	value6:  T6,
	value7:  T7,
	value8:  T8,
	value9:  T9,
	value10: T10,
	value11: T11,
}

impl_non_zero!(i128);
impl_non_zero!(i16);
impl_non_zero!(i32);
impl_non_zero!(i64);
impl_non_zero!(i8);
impl_non_zero!(isize);
impl_non_zero!(u128);
impl_non_zero!(u16);
impl_non_zero!(u32);
impl_non_zero!(u64);
impl_non_zero!(u8);
impl_non_zero!(usize);

impl_atomic! {
	width: "8",
	ty: bool,
	atomic_ty: core::sync::atomic::AtomicBool,
}

impl_atomic! {
	width: "16",
	ty: i16,
	atomic_ty: core::sync::atomic::AtomicI16,
}

impl_atomic! {
	width: "32",
	ty: i32,
	atomic_ty: core::sync::atomic::AtomicI32,
}

impl_atomic! {
	width: "64",
	ty: i64,
	atomic_ty: core::sync::atomic::AtomicI64,
}

impl_atomic! {
	width: "8",
	ty: i8,
	atomic_ty: core::sync::atomic::AtomicI8,
}

impl_atomic! {
	width: "ptr",
	ty: isize,
	atomic_ty: core::sync::atomic::AtomicIsize,
}

impl_atomic! {
	width: "16",
	ty: u16,
	atomic_ty: core::sync::atomic::AtomicU16,
}

impl_atomic! {
	width: "32",
	ty: u32,
	atomic_ty: core::sync::atomic::AtomicU32,
}

impl_atomic! {
	width: "64",
	ty: u64,
	atomic_ty: core::sync::atomic::AtomicU64,
}

impl_atomic! {
	width: "8",
	ty: u8,
	atomic_ty: core::sync::atomic::AtomicU8,
}

impl_atomic! {
	width: "ptr",
	ty: usize,
	atomic_ty: core::sync::atomic::AtomicUsize,
}
