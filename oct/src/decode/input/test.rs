// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

#![cfg(test)]

use oct::decode::Input;
use oct::error::InputError;

#[test]
fn test_decode_input() {
	let buf = [0xFF, 0xFE, 0xFD, 0xFC];

	let mut input = Input::new(&buf);

	assert_eq!(input.capacity(),  0x4);
	assert_eq!(input.position(),  0x0);
	assert_eq!(input.remaining(), 0x4);

	assert_eq!(input, [0xFF, 0xFE, 0xFD, 0xFC].as_slice());
	assert_eq!(input.read(0x1), Ok([0xFF].as_slice()));

	assert_eq!(input.capacity(),  0x4);
	assert_eq!(input.position(),  0x1);
	assert_eq!(input.remaining(), 0x3);

	assert_eq!(input, [0xFE, 0xFD, 0xFC].as_slice());
	assert_eq!(input.read(0x1), Ok([0xFE].as_slice()));

	assert_eq!(input.capacity(),  0x4);
	assert_eq!(input.position(),  0x2);
	assert_eq!(input.remaining(), 0x2);

	assert_eq!(input, [0xFD, 0xFC].as_slice());
	assert_eq!(input.read(0x1), Ok([0xFD].as_slice()));

	assert_eq!(input.capacity(),  0x4);
	assert_eq!(input.position(),  0x3);
	assert_eq!(input.remaining(), 0x1);

	assert_eq!(input, [0xFC].as_slice());
	assert_eq!(input.read(0x1), Ok([0xFC].as_slice()));

	assert_eq!(input.capacity(),  0x4);
	assert_eq!(input.position(),  0x4);
	assert_eq!(input.remaining(), 0x0);

	assert_eq!(input, [].as_slice());
	assert_eq!(input.read(0x1), Err(InputError { capacity: 0x4, position: 0x4, count: 0x1 }));

	assert_eq!(input.read(0x0), Ok([].as_slice()));
}
