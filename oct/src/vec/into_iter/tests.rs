// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use oct::string::String;
use oct::vec::Vec;
use std::str::FromStr;

#[test]
fn test_vec_iter_clone() {
	let data = String::<0x9>::from_str("fran\u{00E7}ais").unwrap();

	let mut data0 = data.into_bytes().into_iter();

	let _ = data0.nth(0x4);

	let mut data1 = data0.clone();

	assert_eq!(data0.next(), Some(0xC3));
	assert_eq!(data1.next(), Some(0xC3));
	assert_eq!(data0.next(), Some(0xA7));
	assert_eq!(data1.next(), Some(0xA7));
	assert_eq!(data0.next(), Some(b'a'));
	assert_eq!(data1.next(), Some(b'a'));
	assert_eq!(data0.next(), Some(b'i'));
	assert_eq!(data1.next(), Some(b'i'));
	assert_eq!(data0.next(), Some(b's'));
	assert_eq!(data1.next(), Some(b's'));
	assert_eq!(data0.next(), None);
	assert_eq!(data1.next(), None);
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
