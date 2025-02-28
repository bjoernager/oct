// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

//! Vector container and iterators.

mod into_iter;
mod vec;

pub use into_iter::IntoIter;
pub use vec::Vec;

use core::ops::Range;

#[inline]
unsafe fn clone_to_uninit_in_range<T: Clone>(src: *const T, dst: *mut T, range: Range<usize>) {
	// SAFETY: The caller guarantees a valid range.
	for i in range.start..range.end {
		// SAFETY: We guarantee that all items in the range
		//
		// 0x0..self.len
		//
		// are alive (and initialised).
		let src_item = unsafe { &*src.add(i) };

		let dst_item = unsafe { dst.add(i) };

		// Clone the item value.

		let value = src_item.clone();

		// Write the item value.

		unsafe { dst_item.write(value) };
	}
}
