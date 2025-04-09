// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

mod test;

use crate::enum_encoded_size;
use crate::encode::Encode;

use core::cell::{Cell, LazyCell, RefCell, UnsafeCell};
use core::convert::Infallible;
use core::ffi::c_void;
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
	alloc::rc::{self, Rc},
};

#[cfg(all(feature = "alloc", target_has_atomic = "ptr"))]
use alloc::sync::{self, Arc};

#[cfg(feature = "std")]
use {
	std::sync::{LazyLock, Mutex, RwLock},
	std::time::SystemTime,
};

/// Denotes a size-constrained, encodable type.
///
/// When using [`Encode`], the size of the resulting encoding cannot always be known beforehand.
/// This trait defines an upper bound for these sizes.
///
/// Note that whilst *technically* having a size limit, [`alloc::vec::Vec`], [`alloc::string::String`], etc. do not implement this trait.
/// The general rule is that the size limit must be a substantial part of a type's design to constitute implementing this trait.
///
/// Also note that -- in practice -- this trait is **not** strictly enforceable.
/// Users of this trait should assume that it is mostly properly defined, but still with the possibility of it not being such.
#[doc(alias("SizedSerialise", "SizedSerialize"))]
pub trait SizedEncode: Encode {
	/// The maximum, guaranteed amount of bytes that can result from an encoding.
	///
	/// Implementors of this trait should make sure that no encoding (or decoding) consumes more than the amount specified by this constant.
	const MAX_ENCODED_SIZE: usize;
}

impl<T: SizedEncode + ?Sized> SizedEncode for &T {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE;
}

impl<T: SizedEncode + ?Sized> SizedEncode for &mut T {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE;
}

/// Implemented for tuples with up to twelve members.
#[cfg_attr(feature = "unstable-docs", doc(fake_variadic))]
impl<T: SizedEncode> SizedEncode for (T,) {
	#[doc(hidden)]
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE;
}

impl<T: SizedEncode, const N: usize> SizedEncode for [T; N] {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE * N;
}

#[cfg(all(feature = "alloc", target_has_atomic = "ptr"))]
#[cfg_attr(feature = "unstable-docs", doc(cfg(all(feature = "alloc", target_has_atomic = "ptr"))))]
impl<T: SizedEncode + ?Sized> SizedEncode for Arc<T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE;
}

impl SizedEncode for bool {
	const MAX_ENCODED_SIZE: usize = u8::MAX_ENCODED_SIZE;
}

impl<T: SizedEncode> SizedEncode for Bound<T> {
	const MAX_ENCODED_SIZE: usize =
		u8::MAX_ENCODED_SIZE
		+ T::MAX_ENCODED_SIZE;
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "alloc")))]
impl<T: SizedEncode + ?Sized> SizedEncode for Box<T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE;
}

impl SizedEncode for c_void {
	const MAX_ENCODED_SIZE: usize = 0x0;
}

impl<T: Copy + SizedEncode> SizedEncode for Cell<T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE;
}

impl SizedEncode for char {
	const MAX_ENCODED_SIZE: usize = u32::MAX_ENCODED_SIZE;
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "alloc")))]
impl<T: SizedEncode + ?Sized + ToOwned> SizedEncode for Cow<'_, T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE;
}

impl SizedEncode for Duration {
	const MAX_ENCODED_SIZE: usize =
		u64::MAX_ENCODED_SIZE
		+ u32::MAX_ENCODED_SIZE;
}

#[cfg(feature = "f16")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "f16")))]
impl SizedEncode for f16 {
	const MAX_ENCODED_SIZE: usize = size_of::<Self>();
}

#[cfg(feature = "f128")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "f128")))]
impl SizedEncode for f128 {
	const MAX_ENCODED_SIZE: usize = size_of::<Self>();
}

impl SizedEncode for Infallible {
	const MAX_ENCODED_SIZE: usize = 0x0;
}

impl SizedEncode for IpAddr {
	const MAX_ENCODED_SIZE: usize =
		u8::MAX_ENCODED_SIZE
		+ enum_encoded_size!((Ipv4Addr), (Ipv6Addr));
}

impl SizedEncode for Ipv4Addr {
	const MAX_ENCODED_SIZE: usize = u32::MAX_ENCODED_SIZE;
}

impl SizedEncode for Ipv6Addr {
	const MAX_ENCODED_SIZE: usize = u128::MAX_ENCODED_SIZE;
}

impl SizedEncode for isize {
	const MAX_ENCODED_SIZE: usize = i16::MAX_ENCODED_SIZE;
}

impl<T: SizedEncode> SizedEncode for LazyCell<T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE;
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "std")))]
impl<T: SizedEncode> SizedEncode for LazyLock<T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE;
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "std")))]
impl<T: SizedEncode + ?Sized> SizedEncode for Mutex<T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE;
}

impl<T: SizedEncode> SizedEncode for Option<T> {
	const MAX_ENCODED_SIZE: usize =
		bool::MAX_ENCODED_SIZE
		+ T::MAX_ENCODED_SIZE;
}

impl<T: ?Sized> SizedEncode for PhantomData<T> {
	const MAX_ENCODED_SIZE: usize = 0x0;
}

impl SizedEncode for PhantomPinned {
	const MAX_ENCODED_SIZE: usize = 0x0;
}

impl<T: SizedEncode> SizedEncode for Range<T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE * 0x2;
}

impl<T: SizedEncode> SizedEncode for RangeFrom<T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE;
}

impl SizedEncode for RangeFull {
	const MAX_ENCODED_SIZE: usize = 0x0;
}

impl<T: SizedEncode> SizedEncode for RangeInclusive<T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE * 0x2;
}

impl<T: SizedEncode> SizedEncode for RangeTo<T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE;
}

impl<T: SizedEncode> SizedEncode for RangeToInclusive<T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE;
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "alloc")))]
impl<T: SizedEncode + ?Sized> SizedEncode for Rc<T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE;
}

impl<T: SizedEncode + ?Sized> SizedEncode for RefCell<T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE;
}

impl<T, E, Err> SizedEncode for Result<T, E>
where
	T: Encode<Error = Err> + SizedEncode,
	E: Encode<Error: Into<Err>> + SizedEncode,
{
	const MAX_ENCODED_SIZE: usize =
		bool::MAX_ENCODED_SIZE
		+ enum_encoded_size!((T), (E));
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "std")))]
impl<T: SizedEncode + ?Sized> SizedEncode for RwLock<T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE;
}

impl<T: SizedEncode> SizedEncode for Saturating<T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE;
}

impl SizedEncode for SocketAddr {
	const MAX_ENCODED_SIZE: usize =
		u8::MAX_ENCODED_SIZE
		+ enum_encoded_size!((SocketAddrV4), (SocketAddrV6));
}

impl SizedEncode for SocketAddrV4 {
	const MAX_ENCODED_SIZE: usize =
		Ipv4Addr::MAX_ENCODED_SIZE
		+ u16::MAX_ENCODED_SIZE;
}

impl SizedEncode for SocketAddrV6 {
	const MAX_ENCODED_SIZE: usize =
		Ipv6Addr::MAX_ENCODED_SIZE
		+ u16::MAX_ENCODED_SIZE
		+ u32::MAX_ENCODED_SIZE
		+ u32::MAX_ENCODED_SIZE;
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "std")))]
impl SizedEncode for SystemTime {
	const MAX_ENCODED_SIZE: usize = i64::MAX_ENCODED_SIZE;
}

impl SizedEncode for () {
	const MAX_ENCODED_SIZE: usize = 0x0;
}

impl<T: SizedEncode> SizedEncode for UnsafeCell<T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE;
}

impl SizedEncode for usize {
	const MAX_ENCODED_SIZE: Self = u16::MAX_ENCODED_SIZE;
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "alloc")))]
impl<T: SizedEncode> SizedEncode for rc::Weak<T> {
	const MAX_ENCODED_SIZE: usize = Option::<Rc<T>>::MAX_ENCODED_SIZE;
}

#[cfg(all(feature = "alloc", target_has_atomic = "ptr"))]
#[cfg_attr(feature = "unstable-docs", doc(cfg(all(feature = "alloc", target_has_atomic = "ptr"))))]
impl<T: SizedEncode> SizedEncode for sync::Weak<T> {
	const MAX_ENCODED_SIZE: usize = Option::<Arc<T>>::MAX_ENCODED_SIZE;
}

impl<T: SizedEncode> SizedEncode for Wrapping<T> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE;
}

macro_rules! impl_numeric {
	($ty:ty$(,)?) => {
		impl ::oct::encode::SizedEncode for $ty {
			const MAX_ENCODED_SIZE: usize = ::core::mem::size_of::<Self>();
		}
	};
}

macro_rules! impl_tuple {
	{
		$($tys:ident),+$(,)?
	} => {
		#[doc(hidden)]
		impl<$($tys,)* E> ::oct::encode::SizedEncode for ($($tys,)*)
		where
			$($tys: Encode<Error = E> + SizedEncode,)* {
			const MAX_ENCODED_SIZE: usize =
				0x0
				$(+ <$tys as ::oct::encode::SizedEncode>::MAX_ENCODED_SIZE)*;
		}
	};
}

macro_rules! impl_non_zero {
	($ty:ty$(,)?) => {
		impl ::oct::encode::SizedEncode for ::core::num::NonZero<$ty> {
			const MAX_ENCODED_SIZE: usize = <$ty as ::oct::encode::SizedEncode>::MAX_ENCODED_SIZE;
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
		impl ::oct::encode::SizedEncode for $atomic_ty {
			const MAX_ENCODED_SIZE: usize = <$ty as ::oct::encode::SizedEncode>::MAX_ENCODED_SIZE;
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
