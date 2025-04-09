// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::decode::Decode;

use core::borrow::Borrow;

#[cfg(feature = "alloc")]
use {
	alloc::boxed::Box,
	alloc::ffi::CString,
	alloc::rc::Rc,
	alloc::sync::Arc,
	core::ffi::CStr,
};

#[cfg(feature = "std")]
use std::ffi::{OsStr, OsString};

/// Indicates a scheme relationship between borrowed and owned types.
///
/// Implementing this trait is specifically a promise that <code>&lt;Self as [Decode]&gt;::[decode](Decode::decode)</code> can handle any encoding of `B`.
///
/// This trait is mainly useful for types that implement [`Encode`](crate::encode::Encode::encode) but do not implement `Decode` for whatever reason (mostly due to being unsized).
/// The primary user of this trait is the `Decode` implementation of [`Cow`](alloc::borrow::Cow).
///
/// # Arrays
///
/// This trait in the form <code>DecodeBorrowed&lt;[\[T\]]&gt;</code> is not implemented for [`[T; N]`](array) due to the fact that arrays do not encode their length, instead having it hard-coded into the type, thus rendering their scheme incompatible with that of slices.
///
/// [\[T\]]: slice
#[doc(alias("DeserialiseBorrowed", "DeserializeBorrowed"))]
pub trait DecodeBorrowed<B>
where
	Self: Borrow<B> + Decode,
	B:    ?Sized,
{ }

impl<T: Decode> DecodeBorrowed<T> for T { }

#[cfg(all(feature = "alloc", target_has_atomic = "ptr"))]
#[cfg_attr(feature = "unstable-docs", doc(cfg(all(feature = "alloc", target_has_atomic = "ptr"))))]
impl<T: Decode> DecodeBorrowed<T> for Arc<T> { }

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "alloc")))]
impl<T: Decode> DecodeBorrowed<T> for Box<T> { }

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "alloc")))]
impl DecodeBorrowed<CStr> for CString { }

#[cfg(feature = "std")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "std")))]
impl DecodeBorrowed<OsStr> for OsString { }

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "alloc")))]
impl<T: Decode> DecodeBorrowed<T> for Rc<T> { }

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "alloc")))]
impl DecodeBorrowed<str> for alloc::string::String { }

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "unstable-docs", doc(cfg(feature = "alloc")))]
impl<T: Decode> DecodeBorrowed<[T]> for alloc::vec::Vec<T> { }
