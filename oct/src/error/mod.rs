// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

//! Error variants.
//!
//! This module defines the error types used by Oct.
//! All of these types define (at least conditionally) the [`Error`](core::error::Error) trait.

mod char_decode_error;
mod collection_decode_error;
mod collection_encode_error;
mod enum_decode_error;
mod enum_encode_error;
mod generic_decode_error;
mod generic_encode_error;
mod input_error;
mod isize_encode_error;
mod item_decode_error;
mod item_encode_error;
mod length_error;
mod non_zero_decode_error;
mod output_error;
mod ref_cell_encode_error;
mod system_time_decode_error;
mod usize_encode_error;
mod utf8_error;

pub use char_decode_error::CharDecodeError;
pub use collection_decode_error::CollectionDecodeError;
pub use collection_encode_error::CollectionEncodeError;
pub use enum_decode_error::EnumDecodeError;
pub use enum_encode_error::EnumEncodeError;
pub use generic_decode_error::GenericDecodeError;
pub use generic_encode_error::GenericEncodeError;
pub use input_error::InputError;
pub use isize_encode_error::IsizeEncodeError;
pub use item_decode_error::ItemDecodeError;
pub use item_encode_error::ItemEncodeError;
pub use length_error::LengthError;
pub use non_zero_decode_error::NonZeroDecodeError;
pub use output_error::OutputError;
pub use ref_cell_encode_error::RefCellEncodeError;
pub use system_time_decode_error::SystemTimeDecodeError;
pub use usize_encode_error::UsizeEncodeError;
pub use utf8_error::Utf8Error;
