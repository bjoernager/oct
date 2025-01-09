// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::vec::{IntoIter, Vec};

use core::mem::MaybeUninit;
use core::slice;

impl<T, const N: usize> FromIterator<T> for Vec<T, N> {
	#[inline]
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		let mut iter = iter.into_iter();

		let mut buf = [const { MaybeUninit::<T>::uninit() };N];
		let mut len = 0x0;

		for item in &mut buf {
			let Some(value) = iter.next() else { break };
			item.write(value);

			len += 0x1;
		}

		Self { buf, len }
	}
}

impl<T, const N: usize> IntoIterator for Vec<T, N> {
	type Item = T;

	type IntoIter = IntoIter<T, N>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		let (buf, len) = self.into_raw_parts();

		unsafe { IntoIter::new(buf, len) }
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
