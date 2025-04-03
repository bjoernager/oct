// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

//! Encoding-related facilities.
//!
//! To encode an object directly using the [`Encode`] trait, simply allocate a buffer for the encoding and wrap it in an [`Output`] object:
//!
//! ```rust
//! use oct::encode::{Encode, Output, SizedEncode};
//!
//! let mut buf = [0x00; char::MAX_ENCODED_SIZE];
//! let mut stream = Output::new(&mut buf);
//!
//! 'Ж'.encode(&mut stream).unwrap();
//!
//! assert_eq!(buf, [0x16, 0x04, 0x00, 0x00].as_slice());
//! ```
//!
//! The `Output` type can also be used to chain multiple objects together:
//!
//! ```rust
//! use oct::encode::{Encode, Output, SizedEncode};
//!
//! let mut buf = [0x0; char::MAX_ENCODED_SIZE * 0x5];
//! let mut stream = Output::new(&mut buf);
//!
//! // Note: For serialising multiple characters, the
//! // `String` and `oct::str::String` types are usual-
//! // ly preferred.
//!
//! 'ل'.encode(&mut stream).unwrap();
//! 'ا'.encode(&mut stream).unwrap();
//! 'م'.encode(&mut stream).unwrap();
//! 'د'.encode(&mut stream).unwrap();
//! 'ا'.encode(&mut stream).unwrap();
//!
//! assert_eq!(buf, [
//!     0x44, 0x06, 0x00, 0x00, 0x27, 0x06, 0x00, 0x00,
//!     0x45, 0x06, 0x00, 0x00, 0x2F, 0x06, 0x00, 0x00,
//!     0x27, 0x06, 0x00, 0x00,
//! ]);
//! ```
//!
//! If the encoded type additionally implements [`SizedEncode`], then the maximum size of any encoding is guaranteed with the [`MAX_ENCODED_SIZE`](SizedEncode::MAX_ENCODED_SIZE) constant.

mod encode;
mod output;
mod sized_encode;

pub use encode::Encode;
pub use output::Output;
pub use sized_encode::SizedEncode;

/// Implements [`Encode`] for the provided type.
///
/// This derive macro, by default, assumes that all fields implement <code>Encode&lt;[Error]: [Into]&lt;[GenericEncodeError]&gt;&gt;</code>.
/// If this is **not** the case, then the `#[oct(encode_error = T)` attribute should be used for the desired error `T` on the deriving type.
///
/// [Error]: Encode::Error
/// [GenericEncodeError]: crate::error::GenericEncodeError
///
/// Do also consider deriving [`SizedEncode`](derive@SizedEncode) -- if appropriate.
///
/// # Structs
///
/// For structures, each element is chained in **order of declaration.**
/// If the structure is a unit structure (i.e. it has *no* fields) then it is encoded equivalently to the [unit] type.
///
/// For example, the following struct will encode its field `foo` followed by `bar`:
///
/// ```rust
/// use oct::encode::Encode;
///
/// #[derive(Encode)]
/// struct FooBar {
///     pub foo: char,
///     pub bar: char,
/// }
/// ```
///
/// This should be kept in mind when changing the structure's declarationm as doing so may invalidate previous encodings.
///
/// The [`Error`](Encode::Error) type will in all cases just be `GenericEncodeError`.
///
/// ## Example
///
/// ```rust
/// use oct::decode::Decode;
/// use oct::encode::Encode;
/// use oct::slot::Slot;
///
/// #[derive(Debug, Decode, Encode, PartialEq)]
/// struct Ints {
///     value0: u8,
///     value1: u16,
///     value2: u32,
///     value3: u64,
///     value4: u128,
/// }
///
/// const VALUE: Ints = Ints {
///     value0: 0x00,
///     value1: 0x02_01,
///     value2: 0x06_05_04_03,
///     value3: 0x0E_0D_0C_0B_0A_09_08_07,
///     value4: 0x1E_1D_1C_1B_1A_19_18_17_16_15_14_13_12_11_10_0F,
/// };
///
/// let mut buf = Slot::with_capacity(0x100);
///
/// buf.write(&VALUE).unwrap();
///
/// assert_eq!(buf.len(), 0x1F);
///
/// assert_eq!(
///     buf,
///     [
///         0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
///         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
///         0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
///         0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E
///     ].as_slice(),
/// );
///
/// assert_eq!(buf.read().unwrap(), VALUE);
/// ```
///
/// # Enums
///
/// Enumerations encode like structures except that each variant additionally encodes a unique discriminant.
///
/// By default, each discriminant is assigned from the range 0 to infinite, to the extend allowed by the [`isize`] type and its encoding (as which **all** discriminants are encoded).
/// A custom discriminant may be set instead by assigning the variant an integer constant.
/// Unspecified discriminants then increment the previous variant's discriminant:
///
/// ## Example
///
/// ```rust
/// use oct::encode::Encode;
/// use oct::slot::Slot;
///
/// #[derive(Encode)]
/// enum Num {
///     Two = 0x2,
///
///     Three,
///
///     Zero = 0x0,
///
///     One,
/// }
///
/// let mut buf = Slot::with_capacity(size_of::<i16>());
///
/// buf.write(&Num::Zero).unwrap();
/// assert_eq!(buf, [0x00, 0x00].as_slice());
///
/// buf.write(&Num::One).unwrap();
/// assert_eq!(buf, [0x01, 0x00].as_slice());
///
/// buf.write(&Num::Two).unwrap();
/// assert_eq!(buf, [0x02, 0x00].as_slice());
///
/// buf.write(&Num::Three).unwrap();
/// assert_eq!(buf, [0x03, 0x00].as_slice());
/// ```
///
/// Variants with fields are encoded exactly like structures.
/// That is, each field is chained in order of declaration.
///
/// For error handling, the `Error` type is defined as:
///
/// <code>[EnumEncodeError]&lt;&lt;Repr as Encode&gt;::Error, GenericEncodeError&gt;</code>,
///
/// [EnumEncodeError]: crate::error::GenericEncodeError
///
/// wherein `Repr` is the enumeration's representation.
///
/// # Unions
///
/// Unions cannot derive `Encode` due to the uncertainty of their contents.
/// The trait should therefore be implemented manually for such types.
///
/// ## Example
///
/// ```rust compile_fail
/// use oct::encode::Encode;
///
/// #[derive(Encode)]
/// union MyUnion {
///     my_field: u32,
/// }
/// ```
#[cfg(feature = "proc-macro")]
#[cfg_attr(doc, doc(cfg(feature = "proc-macro")))]
#[doc(inline)]
pub use oct_macros::Encode;

/// Implements [`Encode`](trait@Encode) using the default implementation.
///
/// For simple structures, the value of [`MAX_ENCODED_SIZE`](SizedEncode::MAX_ENCODED_SIZE) is set as the combined value of <code>T*n*::MAX_ENCODED_SIZE</code> wherein <code>T*n*</code> is the type of each field.
///
/// For enumerations, the value is set such that each variant is treated like a structure (with the discriminant as an extra field) and where the variant that produces the largest `MAX_ENCODED_SIZE` is chosen.
///
/// As untagged unions cannot derive `Encode`, `SizedEncode` also cannot be derived for them.
///
/// Do remember that deriving this trait is only recommended
#[cfg(feature = "proc-macro")]
#[cfg_attr(doc, doc(cfg(feature = "proc-macro")))]
#[doc(inline)]
pub use oct_macros::SizedEncode;
