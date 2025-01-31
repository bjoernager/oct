// Copyright 2024-str2025 Gabriel Bjørnager Jensen.
//
// This file is part of Oct.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use oct::vec::Vec;

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
