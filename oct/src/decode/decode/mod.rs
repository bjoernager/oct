// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

mod test;

use crate::__cold_path;
use crate::decode::{DecodeBorrowed, Input};
use crate::error::{
	BoolDecodeError,
	CharDecodeError,
	CollectionDecodeError,
	EnumDecodeError,
	ItemDecodeError,
};

use core::cell::{Cell, RefCell, UnsafeCell};
use core::convert::Infallible;
use core::ffi::c_void;
use core::marker::{PhantomData, PhantomPinned};
use core::mem::MaybeUninit;
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
	crate::error::Utf8Error,

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

#[cfg(any(feature = "f128", feature = "f16"))]
use crate::oct::encode::SizedEncode;

#[cfg(feature = "std")]
use {
	crate::error::SystemTimeDecodeError,

	core::hash::{BuildHasher, Hash},

	std::collections::{HashMap, HashSet},
	std::ffi::OsString,
	std::sync::{Mutex, RwLock},
	std::time::{SystemTime, UNIX_EPOCH},
};

// Should we require `Encode` for `Decode`?

/// Denotes a type capable of being decoded.
///
/// This trait can be derived for custom types using the [`Decode`](derive@crate::decode::Decode) derive macro.
///
/// Do remember that this macro assumes that the [`Encode`](crate::encode::Encode) trait has **not** been manually implemented (i.e. it must either be not implemented or derived).
/// Breaking this promise is a logic error and can lead to failed decodings.
///
/// *See also the [`decode`](crate::decode) module's documentation on how to use decodings.*
#[doc(alias("Deserialise", "Deserialize"))]
pub trait Decode: Sized {
	/// The type returned in case of error.
	type Error;

	/// Decodes an object from the provided input.
	///
	/// # Errors
	///
	/// If decoding fails due to e.g. an invalid byte sequence in the input, then an error should be returned.
	///
	/// # Panics
	///
	/// If `input` unexpectedly terminates before a full encoding was read, then this method should panic.
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error>;
}

/// Implemented for tuples with up to twelve members.
#[cfg_attr(feature = "unstable-docs", doc(fake_variadic))]
impl<T: Decode> Decode for (T,) {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let this = (Decode::decode(input)?,);
		Ok(this)
	}
}

impl<T: Decode, const N: usize> Decode for [T; N] {
	type Error = CollectionDecodeError<Infallible, ItemDecodeError<usize, T::Error>>;

	#[inline]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		// Initialise the array incrementally.

 		let mut buf = [const { MaybeUninit::<T>::uninit() }; N];

		for (i, item) in buf.iter_mut().enumerate() {
			let value = match Decode::decode(input) {
				Ok(value) => value,

				Err(e) => {
					__cold_path();
					return Err(CollectionDecodeError::BadItem(ItemDecodeError { index: i, error: e }));
				}
			};

			item.write(value);
		}

		// SAFETY: This should be safe as `MaybeUninit<T>`
		// is transparent to `T` and we have initialised
		// every element. The original buffer is NOT
		// dropped automatically, so we can just forget
		// about it from this point on. `transmute` cannot
		// be used here, and `transmute_unchecked` is re-
		// served for the greedy rustc devs. >:(
		let this = unsafe { (buf.as_ptr() as *const [T; N]).read() };
		Ok(this)
	}
}

#[cfg(all(feature = "alloc", target_has_atomic = "ptr"))]
#[cfg_attr(feature = "unstable-docs", doc(cfg(all(feature = "alloc", target_has_atomic = "ptr"))))]
impl<T: Decode> Decode for Arc<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let value = Decode::decode(input)?;

		let this = Self::new(value);
		Ok(this)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "alloc")))]
impl<T: Decode + Ord> Decode for BinaryHeap<T> {
	type Error = <Vec<T> as Decode>::Error;

	#[inline(always)]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let v = Vec::decode(input)?;

		let this = v.into();
		Ok(this)
	}
}

impl Decode for bool {
	type Error = BoolDecodeError;

	/// Lossily reinterprets a byte value as a boolean.
	///
	/// Whilst <code>[Encode](crate::encode::Encode)::[encode](crate::encode::Encode::encode)</code> will only yield the values `0` and `1`, this method clamps all values above `1`.
	#[inline]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(value) = u8::decode(input);

		if value > 0x1 {
			__cold_path();
			return Err(BoolDecodeError { value });
		}

		let this = value != 0x0;
		Ok(this)
	}
}

impl<T: Decode> Decode for Bound<T> {
	type Error = EnumDecodeError<u8, <u8 as Decode>::Error, T::Error>;

	#[inline(always)]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(discriminant) = u8::decode(input);

		let this = match discriminant {
			0x0 => {
				let bound = Decode::decode(input)
					.map_err(EnumDecodeError::BadField)?;

				Self::Included(bound)
			}

			0x1 => {
				let bound = Decode::decode(input)
					.map_err(EnumDecodeError::BadField)?;

				Self::Excluded(bound)
			}

			0x2 => Self::Unbounded,

			value => return Err(EnumDecodeError::UnassignedDiscriminant(value)),
		};

		Ok(this)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "alloc")))]
impl<T: Decode> Decode for Box<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let value = Decode::decode(input)?;

		let this = Self::new(value);
		Ok(this)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "alloc")))]
impl Decode for CString {
	type Error = <Vec<u8> as Decode>::Error;

	/// Decodes a byte slice from the input.
	///
	/// This implementation will always allocate one more byte than specified by the slice for the null terminator.
	/// Note that any null value already in the data will truncate the final string.
	#[inline]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(len) = usize::decode(input);

		let mut v = alloc::vec![0x00; len + 0x1];
		input.read_into(&mut v[..len]);

		// SAFETY: We have guaranteed that there is at
		// least one null value. We also don't care if the
		// string gets truncated due to invalid input.
		let this = unsafe { Self::from_vec_with_nul_unchecked(v) };
		Ok(this)
	}
}

impl<T: Decode> Decode for Cell<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let value = Decode::decode(input)?;

		let this = Self::new(value);
		Ok(this)
	}
}

impl Decode for c_void {
	type Error = Infallible;

	#[inline(always)]
	#[track_caller]
	fn decode(_input: &mut Input) -> Result<Self, Self::Error> {
		panic!("cannot deserialise `c_void` as it cannot be constructed to begin with")
	}
}

impl Decode for char {
	type Error = CharDecodeError;

	#[inline]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(code_point) = u32::decode(input);

		code_point.try_into().map_err(|_| {
			__cold_path();
			CharDecodeError { code_point }
		})
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "alloc")))]
impl<T, B> Decode for Cow<'_, B>
where
	T: DecodeBorrowed<B>,
	B: ToOwned<Owned = T> + ?Sized,
{
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let value = Decode::decode(input)?;

		let this = Self::Owned(value);
		Ok(this)
	}
}

impl Decode for Duration {
	type Error = Infallible;

	#[inline(always)]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(secs)  = Decode::decode(input);
		let Ok(nanos) = Decode::decode(input);

		let this = Self::new(secs, nanos);
		Ok(this)
	}
}

#[cfg(feature = "f128")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "f128")))]
impl Decode for f128 {
	type Error = Infallible;

	#[inline]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let mut data = [Default::default(); <Self as SizedEncode>::MAX_ENCODED_SIZE];
		input.read_into(&mut data);

		let this = Self::from_le_bytes(data);
		Ok(this)
	}
}

#[cfg(feature = "f16")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "f16")))]
impl Decode for f16 {
	type Error = Infallible;

	#[inline]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let mut data = [Default::default(); <Self as SizedEncode>::MAX_ENCODED_SIZE];
		input.read_into(&mut data);

		let this = Self::from_le_bytes(data);
		Ok(this)
	}
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "std")))]
impl<K, V, S, E> Decode for HashMap<K, V, S>
where
	K: Decode<Error = E> + Eq + Hash,
	V: Decode<Error: Into<E>>,
	S: BuildHasher + Default,
{
	type Error = CollectionDecodeError<Infallible, ItemDecodeError<usize, E>>;

	#[inline]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(len) = Decode::decode(input);

		let mut this = Self::with_capacity_and_hasher(len, Default::default());

		for i in 0x0..len {
			let key = K::decode(input)
				.map_err(|e| CollectionDecodeError::BadItem(ItemDecodeError { index: i, error: e }))?;

			let value = V::decode(input)
				.map_err(|e| CollectionDecodeError::BadItem(ItemDecodeError { index: i, error: e.into() }))?;

			this.insert(key, value);
		}

		Result::Ok(this)
	}
}


#[cfg(feature = "std")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "std")))]
impl<K, S> Decode for HashSet<K, S>
where
	K: Decode + Eq + Hash,
	S: BuildHasher + Default,
{
	type Error = CollectionDecodeError<Infallible, ItemDecodeError<usize, K::Error>>;

	#[inline]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(len) = Decode::decode(input);

		let mut this = Self::with_capacity_and_hasher(len, Default::default());

		for i in 0x0..len {
			let key = Decode::decode(input)
				.map_err(|e| CollectionDecodeError::BadItem(ItemDecodeError { index: i, error: e }) )?;

			this.insert(key);
		}

		Result::Ok(this)
	}
}

impl Decode for Infallible {
	type Error = Self;

	#[inline(always)]
	#[track_caller]
	fn decode(_input: &mut Input) -> Result<Self, Self::Error> {
		panic!("cannot deserialise `Infallible` as it cannot be constructed to begin with")
	}
}

impl Decode for IpAddr {
	type Error = EnumDecodeError<u8, <u8 as Decode>::Error, Infallible>;

	#[inline]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(discriminant) = u8::decode(input);

		match discriminant {
			0x4 => {
				let Ok(addr) = Decode::decode(input);

				let this = Self::V4(addr);
				Ok(this)
			}

			0x6 => {
				let Ok(addr) = Decode::decode(input);

				let this = Self::V6(addr);
				Ok(this)
			}

			value => {
				__cold_path();
				Err(EnumDecodeError::UnassignedDiscriminant(value))
			}
		}
	}
}

impl Decode for Ipv4Addr {
	type Error = Infallible;

	#[inline(always)]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(value) = Decode::decode(input);

		let this = Self::from_bits(value);
		Ok(this)
	}
}

impl Decode for Ipv6Addr {
	type Error = Infallible;

	#[inline(always)]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(value) = Decode::decode(input);

		let this = Self::from_bits(value);
		Ok(this)
	}
}

impl Decode for isize {
	type Error = Infallible;

	#[inline(always)]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(value) = i16::decode(input);

		Ok(value as Self)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "alloc")))]
impl<T: Decode> Decode for LinkedList<T> {
	type Error = CollectionDecodeError<Infallible, ItemDecodeError<usize, T::Error>>;

	#[inline]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(len) = usize::decode(input);

		let mut this = Self::new();

		for i in 0x0..len {
			let value = T::decode(input)
				.map_err(|e| CollectionDecodeError::BadItem(ItemDecodeError { index: i, error: e }))?;

			this.push_back(value);
		}

		Ok(this)
	}
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "std")))]
impl<T: Decode> Decode for Mutex<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let value = Decode::decode(input)?;

		let this = Self::new(value);
		Ok(this)
	}
}

impl<T: Decode> Decode for Option<T> {
	type Error = EnumDecodeError<bool, <bool as Decode>::Error, T::Error>;

	#[inline]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let sign = bool::decode(input)
			.map_err(EnumDecodeError::InvalidDiscriminant)?;

		let this = if sign {
			let value = Decode::decode(input)
				.map_err(EnumDecodeError::BadField)?;

			Some(value)
		} else {
			None
		};

		Ok(this)
	}
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "std")))]
impl Decode for OsString {
	type Error = <String as Decode>::Error;

	#[inline(always)]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let s: String = Decode::decode(input)?;

		let this = s.into();
		Ok(this)
	}
}

impl<T> Decode for PhantomData<T> {
	type Error = Infallible;

	#[inline(always)]
	#[track_caller]
	fn decode(_input: &mut Input) -> Result<Self, Self::Error> {
		Ok(Self)
	}
}

impl Decode for PhantomPinned {
	type Error = Infallible;

	#[inline(always)]
	#[track_caller]
	fn decode(_input: &mut Input) -> Result<Self, Self::Error> {
		Ok(Self)
	}
}

impl<T: Decode> Decode for Range<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let start = Decode::decode(input)?;
		let end   = Decode::decode(input)?;

		Ok(start..end)
	}
}

impl<T: Decode> Decode for RangeFrom<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let start = Decode::decode(input)?;

		Ok(start..)
	}
}

impl Decode for RangeFull {
	type Error = Infallible;

	#[inline(always)]
	#[track_caller]
	fn decode(_input: &mut Input) -> Result<Self, Self::Error> {
		Ok(..)
	}
}

impl<T: Decode> Decode for RangeInclusive<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let start = Decode::decode(input)?;
		let end   = Decode::decode(input)?;

		Ok(start..=end)
	}
}

impl<T: Decode> Decode for RangeTo<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let end = Decode::decode(input)?;

		Ok(..end)
	}
}

impl<T: Decode> Decode for RangeToInclusive<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let end = Decode::decode(input)?;

		Ok(..=end)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "alloc")))]
impl<T: Decode> Decode for Rc<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		Ok(Self::new(Decode::decode(input)?))
	}
}

impl<T: Decode> Decode for RefCell<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let value = Decode::decode(input)?;

		let this = Self::new(value);
		Ok(this)
	}
}

impl<T, E, Err> Decode for Result<T, E>
where
	T: Decode<Error = Err>,
	E: Decode<Error: Into<Err>>,
{
	type Error = EnumDecodeError<bool, <bool as Decode>::Error, Err>;

	#[inline]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let sign = bool::decode(input)
			.map_err(EnumDecodeError::InvalidDiscriminant)?;

		let this = if sign {
			let value = Decode::decode(input)
				.map_err(Into::<Err>::into)
				.map_err(EnumDecodeError::BadField)?;

			Err(value)
		} else {
			let value = Decode::decode(input)
				.map_err(EnumDecodeError::BadField)?;

			Ok(value)
		};

		Ok(this)
	}
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "std")))]
impl<T: Decode> Decode for RwLock<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let value = Decode::decode(input)?;

		let this = Self::new(value);
		Ok(this)
	}
}

impl<T: Decode> Decode for Saturating<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let value = Decode::decode(input)?;

		let this = Self(value);
		Ok(this)
	}
}

impl Decode for SocketAddr {
	type Error = EnumDecodeError<u8, <u8 as Decode>::Error, Infallible>;

	#[inline]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(discriminant) = u8::decode(input);

		match discriminant {
			0x4 => {
				let Ok(addr) = Decode::decode(input);

				let this = Self::V4(addr);
				Ok(this)
			}

			0x6 => {
				let Ok(addr) = Decode::decode(input);

				let this = Self::V6(addr);
				Ok(this)
			}

			value => {
				__cold_path();
				Err(EnumDecodeError::UnassignedDiscriminant(value))
			}
		}
	}
}

impl Decode for SocketAddrV4 {
	type Error = Infallible;

	#[inline(always)]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let ip   = Decode::decode(input)?;
		let port = Decode::decode(input)?;

		let this = Self::new(ip, port);
		Ok(this)
	}
}

impl Decode for SocketAddrV6 {
	type Error = Infallible;

	#[inline(always)]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let ip        = Decode::decode(input)?;
		let port      = Decode::decode(input)?;
		let flow_info = Decode::decode(input)?;
		let scope_id  = Decode::decode(input)?;

		let this = Self::new(ip, port, flow_info, scope_id);
		Ok(this)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "alloc")))]
impl Decode for String {
	type Error = CollectionDecodeError<Infallible, Utf8Error>;

	#[inline]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(len) = Decode::decode(input);

		let mut v = alloc::vec![0x00; len];
		input.read_into(&mut v);

		match Self::from_utf8(v) {
			Ok(s) => Ok(s),

			Err(e) => {
				__cold_path();

				let i = e.utf8_error().valid_up_to();
				let c = e.as_bytes()[i];

				Err(CollectionDecodeError::BadItem(
					Utf8Error { value: c, index: i },
				))
			}
		}
	}
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "std")))]
impl Decode for SystemTime {
	type Error = SystemTimeDecodeError;

	#[inline]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(time) = i64::decode(input);

		let this = if time.is_positive() {
			let time = time as u64;

			UNIX_EPOCH.checked_add(Duration::from_secs(time))
		} else {
			let time = time.unsigned_abs();

			UNIX_EPOCH.checked_sub(Duration::from_secs(time))
		};

		this.ok_or(SystemTimeDecodeError { timestamp: time })
	}
}

impl Decode for () {
	type Error = Infallible;

	#[inline(always)]
	#[track_caller]
	fn decode(_input: &mut Input) -> Result<Self, Self::Error> {
		Ok(())
	}
}

impl<T: Decode> Decode for UnsafeCell<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let value = Decode::decode(input)?;

		let this = Self::new(value);
		Ok(this)
	}
}

impl Decode for usize {
	type Error = Infallible;

	#[inline(always)]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(value) = u16::decode(input);
		Ok(value as Self)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "alloc")))]
impl<T: Decode> Decode for Vec<T> {
	type Error = CollectionDecodeError<Infallible, ItemDecodeError<usize, T::Error>>;

	#[inline]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let Ok(len) = Decode::decode(input);

		let mut this = Self::with_capacity(len);

		let buf = this.as_mut_ptr();
		for i in 0x0..len {
			let value = match Decode::decode(input) {
				Ok(value) => value,

				Err(e) => {
					__cold_path();
					return Err(CollectionDecodeError::BadItem(ItemDecodeError { index: i, error: e }));
				}
			};

			// SAFETY: Each index is within bounds (i.e. less
			// than `len`).
			unsafe { buf.add(i).write(value) };
		}

		// SAFETY: We have initialised the buffer.
		unsafe { this.set_len(len); }

		Ok(this)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "alloc")))]
impl<T: Decode> Decode for rc::Weak<T> {
	type Error = <Option<Rc<T>> as Decode>::Error;

	#[inline]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		// No matter the presence of a value, the resulting
		// pointer will still be dropped when the caller
		// gets it. So we might as well not care.

		// NOTE: The cursor still has to be moved forward.
		let _ = Option::<Rc<T>>::decode(input)?;

		let this = Self::new();
		Ok(this)
	}
}

#[cfg(all(feature = "alloc", target_has_atomic = "ptr"))]
#[cfg_attr(feature = "unstable-docs", doc(cfg(all(feature = "alloc", target_has_atomic = "ptr"))))]
impl<T: Decode> Decode for sync::Weak<T> {
	type Error = <Option<Arc<T>> as Decode>::Error;

	#[inline]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		// No matter the presence of a value, the resulting
		// pointer will still be dropped when the caller
		// gets it. So we might as well not care.

		// NOTE: The cursor still has to be moved forward.
		let _ = Option::<Arc<T>>::decode(input)?;

		let this = Self::new();
		Ok(this)
	}
}

impl<T: Decode> Decode for Wrapping<T> {
	type Error = T::Error;

	#[inline(always)]
	#[track_caller]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let value = Decode::decode(input)?;

		let this = Self(value);
		Ok(this)
	}
}

macro_rules! impl_numeric {
	($ty:ty$(,)?) => {
		impl ::oct::decode::Decode for $ty {
			type Error = ::core::convert::Infallible;

			#[inline]
			#[track_caller]
			fn decode(input: &mut ::oct::decode::Input) -> ::core::result::Result<Self, Self::Error> {
				let mut data = [::core::default::Default::default(); <Self as ::oct::encode::SizedEncode>::MAX_ENCODED_SIZE];
				input.read_into(&mut data);

				let this = Self::from_le_bytes(data);
				::core::result::Result::Ok(this)
			}
		}
	};
}

macro_rules! impl_tuple {
	{
		$ty:ident,
		$($extra_tys:ident),*$(,)?
	} => {
		#[doc(hidden)]
		impl<$ty, $($extra_tys,)* E> ::oct::decode::Decode for ($ty, $($extra_tys,)*)
		where
			$ty: ::oct::decode::Decode<Error = E>,
			$($extra_tys: ::oct::decode::Decode<Error: ::core::convert::Into<E>>,)*
		{
			type Error = E;

			#[inline(always)]
			#[track_caller]
			fn decode(input: &mut ::oct::decode::Input) -> ::core::result::Result<Self, Self::Error> {
				let this = (
					<$ty as ::oct::decode::Decode>::decode(input)?,

					$(
						<$extra_tys as ::oct::decode::Decode>::decode(input)
							.map_err(::core::convert::Into::<E>::into)?,
					)*
				);

				::core::result::Result::Ok(this)
			}
		}
	};
}

macro_rules! impl_non_zero {
	($ty:ty$(,)?) => {
		impl ::oct::decode::Decode for ::core::num::NonZero<$ty> {
			type Error = ::oct::error::NonZeroDecodeError;

			#[inline]
			#[track_caller]
			fn decode(input: &mut ::oct::decode::Input) -> ::core::result::Result<Self, Self::Error> {
 				let Ok(value) = <$ty as ::oct::decode::Decode>::decode(input);

				::core::num::NonZero::new(value).ok_or_else(|| {
					::oct::__cold_path();
					::oct::error::NonZeroDecodeError
				})
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
		impl ::oct::decode::Decode for $atomic_ty {
			type Error = <$ty as ::oct::decode::Decode>::Error;

			#[inline(always)]
			#[track_caller]
			fn decode(input: &mut ::oct::decode::Input) -> ::core::result::Result<Self, Self::Error> {
				let value = ::oct::decode::Decode::decode(input)?;

				let this = Self::new(value);
				::core::result::Result::Ok(this)
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
	T0,
	T1,
}

impl_tuple! {
	T0,
	T1,
	T2,
}

impl_tuple! {
	T0,
	T1,
	T2,
	T3,
}

impl_tuple! {
	T0,
	T1,
	T2,
	T3,
	T4,
}

impl_tuple! {
	T0,
	T1,
	T2,
	T3,
	T4,
	T5,
}

impl_tuple! {
	T0,
	T1,
	T2,
	T3,
	T4,
	T5,
	T6,
}

impl_tuple! {
	T0,
	T1,
	T2,
	T3,
	T4,
	T5,
	T6,
	T7,
}

impl_tuple! {
	T0,
	T1,
	T2,
	T3,
	T4,
	T5,
	T6,
	T7,
	T8,
}

impl_tuple! {
	T0,
	T1,
	T2,
	T3,
	T4,
	T5,
	T6,
	T7,
	T8,
	T9,
}

impl_tuple! {
	T0,
	T1,
	T2,
	T3,
	T4,
	T5,
	T6,
	T7,
	T8,
	T9,
	T10,
}

impl_tuple! {
	T0,
	T1,
	T2,
	T3,
	T4,
	T5,
	T6,
	T7,
	T8,
	T9,
	T10,
	T11,
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
