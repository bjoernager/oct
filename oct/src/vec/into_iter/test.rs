// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

#![cfg(test)]

use oct::string;
use oct::string::String;
use oct::vec::Vec;

#[test]
fn test_vec_iter_clone() {
	let data: String::<0xF> = string!("fran\u{00E7}aise");

	assert_eq!(data.len(),      0xA);
	assert_eq!(data.as_bytes(), b"fran\xC3\xA7aise");

	let mut iter0 = data.into_bytes().into_iter();

	assert_eq!(iter0.len(),      0xA);
	assert_eq!(iter0.as_slice(), b"fran\xC3\xA7aise");

	assert_eq!(iter0.nth(0x3), Some(b'n'));

	assert_eq!(iter0.len(),      0x6);
	assert_eq!(iter0.as_slice(), b"\xC3\xA7aise");

	let mut iter1 = iter0.clone();

	assert_eq!(iter1.len(),      0x6);
	assert_eq!(iter1.as_slice(), b"\xC3\xA7aise");

	assert_eq!(iter0.next(), Some(0xC3));
	assert_eq!(iter1.next(), Some(0xC3));
	assert_eq!(iter0.next(), Some(0xA7));
	assert_eq!(iter1.next(), Some(0xA7));
	assert_eq!(iter0.next(), Some(b'a'));
	assert_eq!(iter1.next(), Some(b'a'));
	assert_eq!(iter0.next(), Some(b'i'));
	assert_eq!(iter1.next(), Some(b'i'));
	assert_eq!(iter0.next(), Some(b's'));
	assert_eq!(iter1.next(), Some(b's'));
	assert_eq!(iter0.next(), Some(b'e'));
	assert_eq!(iter1.next(), Some(b'e'));
	assert_eq!(iter0.next(), None);
	assert_eq!(iter1.next(), None);
}

#[test]
fn test_vec_iter_double_ended() {
	let data = Vec::from([
		'H', 'E', 'L', 'L', 'O', ' ', 'W', 'O',
		'R', 'L', 'D',
	]);

	let mut data = data.into_iter();

	assert_eq!(data.next(),      Some('H'));
	assert_eq!(data.next_back(), Some('D'));
	assert_eq!(data.next(),      Some('E'));
	assert_eq!(data.next_back(), Some('L'));
	assert_eq!(data.next(),      Some('L'));
	assert_eq!(data.next_back(), Some('R'));
	assert_eq!(data.next(),      Some('L'));
	assert_eq!(data.next_back(), Some('O'));
	assert_eq!(data.next(),      Some('O'));
	assert_eq!(data.next_back(), Some('W'));
	assert_eq!(data.next(),      Some(' '));
}

#[test]
fn test_vec_new() {
	assert_eq!(
		Vec::<u32, 0x6>::new([0x6]),
		[0x6].as_slice(),
	);

	assert_eq!(
		Vec::<u32, 0x6>::new([0x6; 0x6]),
		[0x6, 0x6, 0x6, 0x6, 0x6, 0x6].as_slice(),
	);
}
