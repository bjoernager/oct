// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::string::String;
use crate::decode::{Decode, DecodeBorrowed, Input};
use crate::encode::{Encode, Output, SizedEncode};
use crate::error::{CollectionDecodeError, LengthError, StringError, Utf8Error};

impl<const N: usize> Decode for String<N> {
	type Error = CollectionDecodeError<LengthError, Utf8Error>;

	#[inline]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let len = Decode::decode(input).unwrap();

		let data = input.read(len).unwrap();

		Self::from_utf8(data)
			.map_err(|e| match e {
				StringError::BadUtf8(e) => CollectionDecodeError::BadItem(e),

				StringError::SmallBuffer(e) => CollectionDecodeError::BadLength(e),
			})
	}
}

impl<const N: usize> DecodeBorrowed<str> for String<N> { }

impl<const N: usize> Encode for String<N> {
	type Error = <str as Encode>::Error;

	#[inline(always)]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		self.as_str().encode(output)
	}
}

impl<const N: usize> SizedEncode for String<N> {
	const MAX_ENCODED_SIZE: usize =
		usize::MAX_ENCODED_SIZE
		+ u8::MAX_ENCODED_SIZE * N;
}
