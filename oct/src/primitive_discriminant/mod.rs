// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

mod sealed {
	/// Denotes a primitive, integral discriminant type.
	///
	/// See the public [`PrimitiveDiscriminant`](crate::PrimitiveDiscriminant) trait for more information.
	pub trait PrimitiveDiscriminant {
		/// Interprets the discriminant value as `u128`.
		///
		/// The returned value has exactly the same representation as the original value except that it is zero-extended to fit.
		#[must_use]
		fn to_u128(self) -> u128;
	}
}

pub(crate) use sealed::PrimitiveDiscriminant as SealedPrimitiveDiscriminant;

/// Denotes a primitive, integral discriminant type.
///
/// This trait is specifically defined as a type which may be used as a representation in the `repr` attribute, i.e. [`u8`], [`i8`], [`u16`], [`i16`], [`u32`], [`i32`], [`u64`], [`i64`], [`usize`], and [`isize`].
///
/// On nightly, this additionally includes [`u128`] and [`i128`] (see the tracking issue for [`repr128`](https://github.com/rust-lang/rust/issues/56071/)), although this trait is implemented for these two types anyhow.
pub trait PrimitiveDiscriminant: Copy + SealedPrimitiveDiscriminant + Sized { }

macro_rules! impl_primitive_discriminant {
	($ty:ty) => {
		impl ::oct::SealedPrimitiveDiscriminant for $ty {
			#[allow(clippy::cast_lossless)]
			#[inline(always)]
			fn to_u128(self) -> u128 {
				self as u128
			}
		}

		impl ::oct::PrimitiveDiscriminant for $ty { }
	};
}

impl_primitive_discriminant!(u8);
impl_primitive_discriminant!(i8);
impl_primitive_discriminant!(u16);
impl_primitive_discriminant!(i16);
impl_primitive_discriminant!(u32);
impl_primitive_discriminant!(i32);
impl_primitive_discriminant!(u64);
impl_primitive_discriminant!(i64);
impl_primitive_discriminant!(u128);
impl_primitive_discriminant!(i128);
impl_primitive_discriminant!(usize);
impl_primitive_discriminant!(isize);
