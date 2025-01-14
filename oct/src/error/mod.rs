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

use crate::use_mod;

use_mod!(pub char_decode_error);
use_mod!(pub collection_decode_error);
use_mod!(pub collection_encode_error);
use_mod!(pub enum_decode_error);
use_mod!(pub enum_encode_error);
use_mod!(pub generic_decode_error);
use_mod!(pub generic_encode_error);
use_mod!(pub input_error);
use_mod!(pub isize_encode_error);
use_mod!(pub item_decode_error);
use_mod!(pub item_encode_error);
use_mod!(pub length_error);
use_mod!(pub non_zero_decode_error);
use_mod!(pub output_error);
use_mod!(pub ref_cell_encode_error);
use_mod!(pub usize_encode_error);
use_mod!(pub utf8_error);

#[cfg(feature = "std")]
use_mod!(pub system_time_decode_error);
