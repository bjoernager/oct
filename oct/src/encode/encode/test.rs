// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use core::time::Duration;
use oct::encode::{Encode, SizedEncode};
use oct::error::UsizeEncodeError;
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
			let mut buf = ::std::vec![0x00; 0x100];

			let mut output = ::oct::encode::Output::new(&mut buf);

			let lhs: ::core::result::Result<&[u8], <$ty as ::oct::encode::Encode>::Error> = <$ty as ::oct::encode::Encode>::encode(&$value, &mut output).map(|_| ::core::borrow::Borrow::borrow(&output));
			let rhs: ::core::result::Result<&[u8], <$ty as ::oct::encode::Encode>::Error> = $data;

			::std::assert_eq!(lhs, rhs);
		})*)*
	}};
}

#[test]
fn test_encode() {
	test! {
		u8 {
			0x00 => Ok(&[0x00]),
			0xFF => Ok(&[0xFF]),
			0x7F => Ok(&[0x7F]),
		}

		u16 {
			0x0F_7E => Ok(&[0x7E, 0x0F]),
		}

		u32 {
			0x00_2F_87_E7 => Ok(&[0xE7, 0x87, 0x2F, 0x00]),
		}

		u64 {
			0xF3_37_CF_8B_DB_03_2B_39 => Ok(&[0x39, 0x2B, 0x03, 0xDB, 0x8B, 0xCF, 0x37, 0xF3]),
		}

		u128 {
			0x45_A0_15_6A_36_77_17_8A_83_2E_3C_2C_84_10_58_1A => Ok(&[
				0x1A, 0x58, 0x10, 0x84, 0x2C, 0x3C, 0x2E, 0x83,
				0x8A, 0x17, 0x77, 0x36, 0x6A, 0x15, 0xA0, 0x45,
			]),
		}

		usize {
			0x1A4 => Ok(&[0xA4, 0x01]),

			0x10000 => Err(UsizeEncodeError(0x10000)),
		}

		[char; 0x5] {
			['\u{03B4}', '\u{0190}', '\u{03BB}', '\u{03A4}', '\u{03B1}'] => Ok(&[
				0xB4, 0x03, 0x00, 0x00, 0x90, 0x01, 0x00, 0x00,
				0xBB, 0x03, 0x00, 0x00, 0xA4, 0x03, 0x00, 0x00,
				0xB1, 0x03, 0x00, 0x00,
			]),
		}

		str {
			"A" => Ok(&[0x01, 0x00, 0x41]),
		}

		str {
			"l\u{00F8}gma\u{00F0}ur" => Ok(&[
				0x0A, 0x00, 0x6C, 0xC3, 0xB8, 0x67, 0x6D, 0x61,
				0xC3, 0xB0, 0x75, 0x72,
			])
		}

		Result<u16, char> {
			Ok(0x4545) => Ok(&[0x00, 0x45, 0x45]),

			Err(char::REPLACEMENT_CHARACTER) => Ok(&[0x01, 0xFD, 0xFF, 0x00, 0x00]),
		}

		Option<()> {
			None => Ok(&[0x00]),

			Some(()) => Ok(&[0x01]),
		}

		SystemTime {
			UNIX_EPOCH => Ok(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),

			UNIX_EPOCH - Duration::from_secs(0x1) => Ok(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]),

			UNIX_EPOCH + Duration::from_secs(0x1) => Ok(&[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
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
			Foo('\u{FDF2}') => Ok(&[0xF2, 0xFD, 0x00, 0x00]),
		}

		Bar {
			Bar::Unit => Ok(&[0x45]),

			Bar::Pretty(true) => Ok(&[0x7F, 0x01]),

			Bar::Teacher { initials: ['T', 'L', '\0'] } => Ok(&[
				0x80, 0x54, 0x00, 0x00, 0x00, 0x4C, 0x00, 0x00,
				0x00, 0x00, 0x00, 0x00, 0x00,
			]),
		}
	}
}
