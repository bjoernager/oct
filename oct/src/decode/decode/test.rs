// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

#![cfg(test)]

use core::char;
use oct::decode::{Decode, Input};
use oct::encode::{Encode, SizedEncode};
use oct::error::EnumDecodeError;

macro_rules! test {
	{
		$(
			$ty:ty {
				$($data:expr => $value:expr),+$(,)?
			}$(,)?
		)*
	} => {
		$($({
			let data: &[u8] = &$data;

			let mut input = ::oct::decode::Input::new(data);

			let left:  ::core::result::Result<$ty, <$ty as ::oct::decode::Decode>::Error> = ::oct::decode::Decode::decode(&mut input);
			let right: ::core::result::Result<$ty, <$ty as ::oct::decode::Decode>::Error> = $value;

			::std::assert_eq!(left, right);
		})*)*
	};
}

#[test]
fn test_decode() {
	test! {
		i8 {
			[0x00] => Ok( 0x00),
			[0x7F] => Ok( 0x7F),
			[0x80] => Ok(-0x80),
			[0xFF] => Ok(-0x01),
		}

		i16 {
			[0x00, 0x00] => Ok( 0x0000),
			[0xFF, 0x7F] => Ok( 0x7FFF),
			[0x00, 0x80] => Ok(-0x8000),
			[0xFF, 0xFF] => Ok(-0x0001),
		}

		i32 {
			[0x00, 0x00, 0x00, 0x00] => Ok( 0x00000000),
			[0xFF, 0xFF, 0xFF, 0x7F] => Ok( 0x7FFFFFFF),
			[0x00, 0x00, 0x00, 0x80] => Ok(-0x80000000),
			[0xFF, 0xFF, 0xFF, 0xFF] => Ok(-0x00000001),
		}

		i64 {
			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] => Ok( 0x0000000000000000),
			[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F] => Ok( 0x7FFFFFFFFFFFFFFF),
			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80] => Ok(-0x8000000000000000),
			[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF] => Ok(-0x0000000000000001),
		}

		u128 {
			[
				0x7F, 0x8F, 0x6F, 0x9F, 0x5F, 0xAF, 0x4F, 0xBF,
				0x3F, 0xCF, 0x2F, 0xDF, 0x1F, 0xEF, 0x0F, 0xFF,
			] => Ok(0xFF_0F_EF_1F_DF_2F_CF_3F_BF_4F_AF_5F_9F_6F_8F_7F),
		}

		char {
			[0xFD, 0xFF, 0x00, 0x00] => Ok(char::REPLACEMENT_CHARACTER),
		}

		[char; 0x5] {
			[
				0xBB, 0x03, 0x00, 0x00, 0x91, 0x03, 0x00, 0x00,
				0xBC, 0x03, 0x00, 0x00, 0x94, 0x03, 0x00, 0x00,
				0xB1, 0x03, 0x00, 0x00,
			] => Ok(['\u{03BB}', '\u{0391}', '\u{03BC}', '\u{0394}', '\u{03B1}']),
		}

		Option<()> {
			[0x00] => Ok(None),
			[0x01] => Ok(Some(())),
		}

		Result<(), i8> {
			[0x00, 0x00] => Ok(Ok(())),
			[0x01, 0x7F] => Ok(Err(i8::MAX)),
		}
	}
}

#[test]
#[should_panic]
fn test_decode_alloc_vec_long_len() {
	let data = [
		0xFF, 0xFF,
	];

	let mut stream = Input::new(&data);

	let _ = <alloc::vec::Vec<u32> as Decode>::decode(&mut stream).unwrap();
}

#[test]
fn test_decode_derive() {
	#[derive(Debug, Decode, Encode, PartialEq, SizedEncode)]
	struct ProcExit {
		exit_code: i32,
		timestmap: u64,
	}

	#[derive(Debug, Decode, Encode, PartialEq, SizedEncode)]
	struct NewByte(u8);

	#[derive(Debug, Decode, Encode, PartialEq, SizedEncode)]
	struct Unit;

	#[derive(Debug, Decode, Encode, PartialEq, SizedEncode)]
	enum UnitOrFields {
		Unit,
		Unnamed(i32),
		Named { timestamp: u64 },
	}

	test! {
		ProcExit {
			[
				0x01, 0x00, 0x00, 0x00, 0x00, 0xE1, 0x0B, 0x5E,
				0x00, 0x00, 0x00, 0x00,
			] => Ok(ProcExit { exit_code: 0x1, timestmap: 1577836800 }),
		}

		NewByte {
			[0x80] => Ok(NewByte(0x80)),
		}

		Unit {
			[] => Ok(Unit),
		}

		UnitOrFields {
			[
				0x00, 0x00,
			] => Ok(UnitOrFields::Unit),

			[
				0x01, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
			] => Ok(UnitOrFields::Unnamed(-0x1)),

			[
				0x02, 0x00, 0x4C, 0xC8, 0xC5, 0x66, 0x00, 0x00,
				0x00, 0x00,
			] => Ok(UnitOrFields::Named { timestamp: 1724237900 }),

			[
				0x03, 0x00,
			] => Err(EnumDecodeError::UnassignedDiscriminant(0x3)),
		}
	}
}

#[test]
fn test_decode_derive_custom_error() {
	#[derive(Debug, Eq, PartialEq)]
	struct FooError;

	#[derive(Debug, Encode, Eq, PartialEq)]
	struct Foo;

	impl Decode for Foo {
		type Error = FooError;

		fn decode(_input: &mut Input) -> Result<Self, Self::Error> {
			Err(FooError)
		}
	}

	#[derive(Debug, Eq, PartialEq)]
	struct BarError;

	#[derive(Debug, Encode, Eq, PartialEq)]
	struct Bar;

	impl Decode for Bar {
		type Error = BarError;

		fn decode(_input: &mut Input) -> Result<Self, Self::Error> {
			Err(BarError)
		}
	}

	#[derive(Debug, Eq, PartialEq)]
	enum FooBarError {
		Foo(FooError),
		Bar(BarError),
	}

	impl From<FooError> for FooBarError {
		fn from(value: FooError) -> Self {
			Self::Foo(value)
		}
	}

	impl From<BarError> for FooBarError {
		fn from(value: BarError) -> Self {
			Self::Bar(value)
		}
	}

	#[repr(u8)]
	#[derive(Debug, Decode, Encode, Eq, PartialEq)]
	#[oct(decode_error = FooBarError)]
	enum FooBar {
		Foo(Foo),
		Bar(Bar),
	}

	test! {
		Foo {
			[] => Err(FooError),
		}

		Bar {
			[] => Err(BarError),
		}

		FooBar {
			[0x00] => Err(EnumDecodeError::BadField(FooBarError::Foo(FooError))),
			[0x01] => Err(EnumDecodeError::BadField(FooBarError::Bar(BarError))),
		}
	}
}
