// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use oct::encode::{Encode, SizedEncode};
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

macro_rules! test {
	{
		$(
			$ty:ty {
				$($value:expr => $data:expr),+$(,)?
			}$(,)?
		)*
	} => {{
		$($({
			let     right: &[u8] = &$data;
			let mut left         = ::std::vec![0x00; right.len()];

			let mut output = ::oct::encode::Output::new(&mut left);
			<$ty as ::oct::encode::Encode>::encode(&$value, &mut output).unwrap();

			::std::assert_eq!(left, right);
		})*)*
	}};
}

#[test]
fn test_encode() {
	test! {
		u8 {
			0x00 => [0x00],
			0xFF => [0xFF],
			0x7F => [0x7F],
		}

		u16 {
			0x0F_7E => [0x7E, 0x0F],
		}

		u32 {
			0x00_2F_87_E7 => [0xE7, 0x87, 0x2F, 0x00],
		}

		u64 {
			0xF3_37_CF_8B_DB_03_2B_39 => [0x39, 0x2B, 0x03, 0xDB, 0x8B, 0xCF, 0x37, 0xF3],
		}

		u128 {
			0x45_A0_15_6A_36_77_17_8A_83_2E_3C_2C_84_10_58_1A => [
				0x1A, 0x58, 0x10, 0x84, 0x2C, 0x3C, 0x2E, 0x83,
				0x8A, 0x17, 0x77, 0x36, 0x6A, 0x15, 0xA0, 0x45,
			],
		}

		[char; 0x5] {
			['\u{03B4}', '\u{0190}', '\u{03BB}', '\u{03A4}', '\u{03B1}'] => [
				0xB4, 0x03, 0x00, 0x00, 0x90, 0x01, 0x00, 0x00,
				0xBB, 0x03, 0x00, 0x00, 0xA4, 0x03, 0x00, 0x00,
				0xB1, 0x03, 0x00, 0x00,
			],
		}

		Result<u16, char> {
			Ok(0x4545) => [0x00, 0x45, 0x45],

			Err(char::REPLACEMENT_CHARACTER) => [0x01, 0xFD, 0xFF, 0x00, 0x00],
		}

		Option<()> {
			None => [0x00],

			Some(()) => [0x01],
		}

		SystemTime {
			UNIX_EPOCH => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],

			UNIX_EPOCH - Duration::from_secs(0x1) => [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],

			UNIX_EPOCH + Duration::from_secs(0x1) => [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
		}
	}
}

#[test]
fn test_encode_derive() {
	#[derive(Encode, SizedEncode)]
	#[repr(transparent)]
	struct Foo(char);

	#[derive(Encode, SizedEncode)]
	#[repr(u8)]
	enum Bar {
		Unit = 0x45,

		Pretty(bool) = 127,

		Teacher { initials: [char; 0x3] },
	}

	test! {
		Foo {
			Foo('\u{FDF2}') => [0xF2, 0xFD, 0x00, 0x00],
		}

		Bar {
			Bar::Unit => [0x45],

			Bar::Pretty(true) => [0x7F, 0x01],

			Bar::Teacher { initials: ['T', 'L', '\0'] } => [
				0x80, 0x54, 0x00, 0x00, 0x00, 0x4C, 0x00, 0x00,
				0x00, 0x00, 0x00, 0x00, 0x00,
			],
		}
	}
}
