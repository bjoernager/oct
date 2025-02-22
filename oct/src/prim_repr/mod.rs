// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::PrimDiscriminant;

mod sealed {
	/// Denotes a primitive enumeration representation.
	///
	/// See the public [`PrimRepr`](crate::PrimRepr) trait for more information.
	pub trait PrimRepr { }
}

pub(crate) use sealed::PrimRepr as SealedPrimRepr;

/// Denotes a primitive enumeration representation.
pub trait PrimRepr: Copy + SealedPrimRepr + Sized {
	/// Converts `self` into a [`PrimDiscriminant`] object.
	#[must_use]
	fn into_prim_discriminant(self) -> PrimDiscriminant;
}

macro_rules! impl_prim_repr {
	($ty:ty => $variant:ident) => {
		impl ::oct::SealedPrimRepr for $ty { }

		impl ::oct::PrimRepr for $ty {
			#[inline(always)]
			fn into_prim_discriminant(self) -> ::oct::PrimDiscriminant {
				::oct::PrimDiscriminant::$variant(self)
			}
		}
	};
}

impl_prim_repr!(u8    => U8);
impl_prim_repr!(u16   => U16);
impl_prim_repr!(u32   => U32);
impl_prim_repr!(u64   => U64);
impl_prim_repr!(u128  => U128);
impl_prim_repr!(usize => Usize);

impl_prim_repr!(i8    => I8);
impl_prim_repr!(i16   => I16);
impl_prim_repr!(i32   => I32);
impl_prim_repr!(i64   => I64);
impl_prim_repr!(i128  => I128);
impl_prim_repr!(isize => Isize);
