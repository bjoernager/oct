// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use oct::error::{StringError, Utf8Error};
use oct::string::String;
use std::cmp::Ordering;

#[test]
fn test_string() {
	let s: String::<0x4> = "hello world".chars().collect();
	assert_eq!(s, "hell")
}

#[test]
fn test_string_size() {
	let string0 = String::<0x0C>::try_from("Hello there!").unwrap();
	let string1 = String::<0x12>::try_from("MEIN_GRO\u{1E9E}_GOTT").unwrap();
	let string2 = String::<0x05>::try_from("Hello").unwrap();

	assert_eq!(string0.partial_cmp(&string0), Some(Ordering::Equal));
	assert_eq!(string0.partial_cmp(&string1), Some(Ordering::Less));
	assert_eq!(string0.partial_cmp(&string2), Some(Ordering::Greater));

	assert_eq!(string1.partial_cmp(&string0), Some(Ordering::Greater));
	assert_eq!(string1.partial_cmp(&string1), Some(Ordering::Equal));
	assert_eq!(string1.partial_cmp(&string2), Some(Ordering::Greater));

	assert_eq!(string2.partial_cmp(&string0), Some(Ordering::Less));
	assert_eq!(string2.partial_cmp(&string1), Some(Ordering::Less));
	assert_eq!(string2.partial_cmp(&string2), Some(Ordering::Equal));

	assert_eq!(string0, "Hello there!");
	assert_eq!(string1, "MEIN_GRO\u{1E9E}_GOTT");
	assert_eq!(string2, "Hello");
}

#[test]
fn test_string_from_utf8() {
	macro_rules! test_utf8 {
		{
			len: $len:literal,
			utf8: $utf8:expr,
			result: $result:pat$(,)?
		 } => {{
			let utf8: &[u8] = $utf8.as_ref();

			assert!(matches!(
				String::<$len>::from_utf8(utf8),
				$result,
			));
		}};
	}

	test_utf8!(
		len:    0x3,
		utf8:   b"A\xF7c",
		result: Err(StringError::BadUtf8(Utf8Error { value: 0xF7, index: 0x1 })),
	);

	test_utf8!(
		len:    0x4,
		utf8:   "A\u{00F7}c",
		result: Ok(..),
	);

	test_utf8!(
		len:    0x4,
		utf8:   b"20\x20\xAC",
		result: Err(StringError::BadUtf8(Utf8Error { value: 0xAC, index: 0x3 })),
	);

	test_utf8!(
		len:    0x5,
		utf8:   "20\u{20AC}",
		result: Ok(..),
	);
}
