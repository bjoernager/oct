// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use core::cmp::Ordering;
use oct::error::Utf8Error;
use oct::string::String;
use oct::vec::Vec;

#[test]
fn test_string() {
	let s: String::<0x4> = "hello world".chars().collect();
	assert_eq!(s, "hell")
}

#[test]
fn test_string_size() {
	let s0 = String::<0x0C>::new("Hello there!").unwrap();
	let s1 = String::<0x12>::new("MEIN_GRO\u{1E9E}_GOTT").unwrap();
	let s2 = String::<0x05>::new("Hello").unwrap();

	assert_eq!(s0.partial_cmp(&s0), Some(Ordering::Equal));
	assert_eq!(s0.partial_cmp(&s1), Some(Ordering::Less));
	assert_eq!(s0.partial_cmp(&s2), Some(Ordering::Greater));

	assert_eq!(s1.partial_cmp(&s0), Some(Ordering::Greater));
	assert_eq!(s1.partial_cmp(&s1), Some(Ordering::Equal));
	assert_eq!(s1.partial_cmp(&s2), Some(Ordering::Greater));

	assert_eq!(s2.partial_cmp(&s0), Some(Ordering::Less));
	assert_eq!(s2.partial_cmp(&s1), Some(Ordering::Less));
	assert_eq!(s2.partial_cmp(&s2), Some(Ordering::Equal));

	assert_eq!(s0, "Hello there!");
	assert_eq!(s1, "MEIN_GRO\u{1E9E}_GOTT");
	assert_eq!(s2, "Hello");
}

#[test]
fn test_string_from_utf8() {
	macro_rules! test_utf8 {
		{
			len: $len:literal,
			utf8: $utf8:expr,
			result: $result:pat$(,)?
		 } => {{
			let utf8 = core::borrow::Borrow::<[u8]>::borrow($utf8);

			assert!(matches!(
				String::<$len>::from_utf8(Vec::new(utf8).unwrap()),
				$result,
			));
		}};
	}

	test_utf8!(
		len:    0x3,
		utf8:   b"A\xF7c",
		result: Err((Utf8Error { value: 0xF7, index: 0x1 }, ..)),
	);

	test_utf8!(
		len:    0x4,
		utf8:   b"A\xC3\xBCc",
		result: Ok(..),
	);

	test_utf8!(
		len:    0x4,
		utf8:   b"20\x20\xAC",
		result: Err((Utf8Error { value: 0xAC, index: 0x3 }, ..)),
	);

	test_utf8!(
		len:    0x5,
		utf8:   b"20\xE2\x82\xAC",
		result: Ok(..),
	);
}
