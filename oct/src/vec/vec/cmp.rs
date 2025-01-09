// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::vec::Vec;

use core::cmp::Ordering;

impl<T: Eq, const N: usize> Eq for Vec<T, N> { }

impl<T: Ord, const N: usize> Ord for Vec<T, N> {
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_slice().cmp(other.as_slice())
	}
}

impl<T: PartialEq<U>, U: PartialEq<T>, const N: usize, const M: usize> PartialEq<Vec<U, M>> for Vec<T, N> {
	#[inline(always)]
	fn eq(&self, other: &Vec<U, M>) -> bool {
		self.as_slice() == other.as_slice()
	}
}

impl<T: PartialEq<U>, U: PartialEq<T>, const N: usize, const M: usize> PartialEq<[U; M]> for Vec<T, N> {
	#[inline(always)]
	fn eq(&self, other: &[U; M]) -> bool {
		self.as_slice() == other.as_slice()
	}
}

impl<T: PartialEq<U>, U: PartialEq<T>, const N: usize> PartialEq<&[U]> for Vec<T, N> {
	#[inline(always)]
	fn eq(&self, other: &&[U]) -> bool {
		self.as_slice() == *other
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: PartialEq<U>, U: PartialEq<T>, const N: usize> PartialEq<alloc::vec::Vec<U>> for Vec<T, N> {
	#[inline(always)]
	fn eq(&self, other: &alloc::vec::Vec<U>) -> bool {
		self.as_slice() == other.as_slice()
	}
}

impl<T: PartialOrd, const N: usize, const M: usize> PartialOrd<Vec<T, M>> for Vec<T, N> {
	#[inline(always)]
	fn partial_cmp(&self, other: &Vec<T, M>) -> Option<Ordering> {
		self.as_slice().partial_cmp(other.as_slice())
	}
}

impl<T: PartialOrd, const N: usize, const M: usize> PartialOrd<[T; M]> for Vec<T, N> {
	#[inline(always)]
	fn partial_cmp(&self, other: &[T; M]) -> Option<Ordering> {
		self.as_slice().partial_cmp(other.as_slice())
	}
}

impl<T: PartialOrd, const N: usize> PartialOrd<&[T]> for Vec<T, N> {
	#[inline(always)]
	fn partial_cmp(&self, other: &&[T]) -> Option<Ordering> {
		self.as_slice().partial_cmp(*other)
	}
}

#[cfg(feature = "alloc")]
#[cfg_attr(doc, doc(cfg(feature = "alloc")))]
impl<T: PartialOrd, const N: usize> PartialOrd<alloc::vec::Vec<T>> for Vec<T, N> {
	#[inline(always)]
	fn partial_cmp(&self, other: &alloc::vec::Vec<T>) -> Option<Ordering> {
		self.as_slice().partial_cmp(other.as_slice())
	}
}
