// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::PrimRepr;

#[cfg(test)]
mod test;

/// A generic-but-primitive discriminant.
///
/// This type represents all types supported by the `repr` attribute on enumerations.
#[expect(missing_docs)]
#[non_exhaustive]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum PrimDiscriminant {
	U8(u8),
	U16(u16),
	U32(u32),
	U64(u64),
	U128(u128),
	Usize(usize),

	I8(i8),
	I16(i16),
	I32(i32),
	I64(i64),
	I128(i128),
	Isize(isize),
}

impl<T: PrimRepr> From<T> for PrimDiscriminant {
	#[inline(always)]
	fn from(value: T) -> Self {
		value.into_prim_discriminant()
	}
}

macro_rules! impl_fmt {
	($fmt:path) => {
		impl $fmt for ::oct::PrimDiscriminant {
			#[inline]
			fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
				macro_rules! match_ty {
					($variant:ident => $ty:ty) => {
						if let Self::$variant(ref v) = *self {
							<$ty as $fmt>::fmt(v, f)?;
							::core::write!(f, ::core::stringify!($ty))?;
						}
					};
				}

				match_ty!(U8    => u8);
				match_ty!(U16   => u16);
				match_ty!(U32   => u32);
				match_ty!(U64   => u64);
				match_ty!(U128  => u128);
				match_ty!(Usize => usize);

				match_ty!(I8    => i8);
				match_ty!(I16   => i16);
				match_ty!(I32   => i32);
				match_ty!(I64   => i64);
				match_ty!(I128  => i128);
				match_ty!(Isize => isize);

				::core::result::Result::Ok(())
			}
		}
	};
}

impl_fmt!(core::fmt::Binary);
impl_fmt!(core::fmt::Debug);
impl_fmt!(core::fmt::Display);
impl_fmt!(core::fmt::LowerExp);
impl_fmt!(core::fmt::LowerHex);
impl_fmt!(core::fmt::Octal);
impl_fmt!(core::fmt::UpperExp);
impl_fmt!(core::fmt::UpperHex);
