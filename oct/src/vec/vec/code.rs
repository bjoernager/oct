// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::vec::Vec;
use crate::decode::{Decode, DecodeBorrowed, Input};
use crate::encode::{Encode, Output, SizedEncode};
use crate::error::{CollectionDecodeError, ItemDecodeError, LengthError};

use core::mem::MaybeUninit;

impl<T: Decode, const N: usize> Decode for Vec<T, N> {
	type Error = CollectionDecodeError<LengthError, ItemDecodeError<usize, T::Error>>;

	#[inline]
	fn decode(input: &mut Input) -> Result<Self, Self::Error> {
		let len = Decode::decode(input).unwrap();
		if len > N {
			return Err(CollectionDecodeError::BadLength(LengthError {
				remaining: N,
				count:     len,
			}));
		}

		let mut buf = [const { MaybeUninit::<T>::uninit() };N];

		for (i, slot) in buf.iter_mut().enumerate() {
			let v = Decode::decode(input)
				.map_err(|e| CollectionDecodeError::BadItem(ItemDecodeError { index: i, error: e }))?;

			slot.write(v);
		}

		Ok(Self { buf, len })
	}
}

impl<T: Decode, const N: usize> DecodeBorrowed<[T]> for Vec<T, N> { }

impl<T: Encode, const N: usize> Encode for Vec<T, N> {
	type Error = <[T] as Encode>::Error;

	#[inline(always)]
	fn encode(&self, output: &mut Output) -> Result<(), Self::Error> {
		self.as_slice().encode(output)
	}
}

impl<T: SizedEncode, const N: usize> SizedEncode for Vec<T, N> {
	const MAX_ENCODED_SIZE: usize = T::MAX_ENCODED_SIZE * N;
}
