// Copyright 2024-str2025 Gabriel Bjørnager Jensen.
//
// This file is part of Oct.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

#![cfg(test)]

use oct::vec;
use oct::vec::Vec;

#[test]
fn test_vec_copy_from_slice() {
	const DATA: &[i64] = &[
		-0x67,
		-0x51,
		 0x07,
		 0x0D,
		 0x14,
		 0x1A,
		 0x1F,
		 0x24,
		 0x2A,
		 0x2F,
		 0x37,
	];

	assert_eq!(
		Vec::<_, 0xB0>::copy_from_slice(DATA).as_deref(),
		Ok([-0x67, -0x51, 0x7, 0xD, 0x14, 0x1A, 0x1F, 0x24, 0x2A, 0x2F, 0x37].as_slice()),
	);
}

#[test]
fn test_vec_from_array() {
	const DATA: [i64; 0x6] = [
		-0x1E,
		 0x0D,
		 0x0F,
		 0x12,
		 0x18,
		 0x1C,
	];

	assert_eq!(
		Vec::<_, 0x6>::new(DATA),
		[-0x1E, 0xD, 0xF, 0x12, 0x18, 0x1C],
	);

	assert_eq!(
		Vec::<_, 0x60>::new(DATA),
		[-0x1E, 0xD, 0xF, 0x12, 0x18, 0x1C],
	);

	assert_eq!(
		Vec::<_, 0x6>::from(DATA),
		[-0x1E, 0xD, 0xF, 0x12, 0x18, 0x1C],
	);
}

#[test]
fn test_vec_from_iter() {
	let f = |x: u32| -> u32 {
		let x = f64::from(x);

		let y = x.sin().powi(0x2) * 1000.0;

		y as u32
	};

	let mut v = alloc::vec::Vec::new();

	for x in 0x0..0x8 {
		v.push(f(x));
	}

	let v: Vec<_, 0x10> = v.into_iter().collect();

	assert_eq!(
		v,
		[0, 708, 826, 19, 572, 919, 78, 431],
	);
}

#[test]
fn test_vec_macro() {
	let v0: Vec<u8, 0x4> = vec![0xEF; 0x3];
	let v1: Vec<u8, 0x4> = vec![0xEF, 0xEF, 0xEF];

	assert_eq!(v0, v1);
}
