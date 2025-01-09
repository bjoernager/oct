// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::string::String;

use core::cmp::Ordering;

impl<const N: usize> Ord for String<N> {
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_str().cmp(other.as_str())
	}
}

impl<const N: usize, const M: usize> PartialEq<String<M>> for String<N> {
	#[inline(always)]
	fn eq(&self, other: &String<M>) -> bool {
		self.as_str() == other.as_str()
	}
}

impl<const N: usize> PartialEq<&str> for String<N> {
	#[inline(always)]
	fn eq(&self, other: &&str) -> bool {
		self.as_str() == *other
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<const N: usize> PartialEq<alloc::string::String> for String<N> {
	#[inline(always)]
	fn eq(&self, other: &alloc::string::String) -> bool {
		self.as_str() == other.as_str()
	}
}

impl<const N: usize, const M: usize> PartialOrd<String<M>> for String<N> {
	#[inline(always)]
	fn partial_cmp(&self, other: &String<M>) -> Option<Ordering> {
		self.as_str().partial_cmp(other.as_str())
	}
}

impl<const N: usize> PartialOrd<&str> for String<N> {
	#[inline(always)]
	fn partial_cmp(&self, other: &&str) -> Option<Ordering> {
		self.as_str().partial_cmp(*other)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<const N: usize> PartialOrd<alloc::string::String> for String<N> {
	#[inline(always)]
	fn partial_cmp(&self, other: &alloc::string::String) -> Option<Ordering> {
		self.as_str().partial_cmp(other.as_str())
	}
}
