// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use alloc::format;
use oct::PrimDiscriminant;

#[test]
fn test_primitive_discriminant_fmt() {
	let value = PrimDiscriminant::I128(0x45);

	assert_eq!(format!("{value}"),    "69i128");
	assert_eq!(format!("{value:+}"),  "+69i128");
	assert_eq!(format!("{value:b}"),  "1000101i128");
	assert_eq!(format!("{value:#b}"), "0b1000101i128");
	assert_eq!(format!("{value:o}"),  "105i128");
	assert_eq!(format!("{value:#o}"), "0o105i128");
	assert_eq!(format!("{value:x}"),  "45i128");
	assert_eq!(format!("{value:#x}"), "0x45i128");
	assert_eq!(format!("{value:X}"),  "45i128");
	assert_eq!(format!("{value:#X}"), "0x45i128");

	let value = PrimDiscriminant::I8(-0x45);

	assert_eq!(format!("{value}"),    "-69i8");
	assert_eq!(format!("{value:+}"),  "-69i8");
	assert_eq!(format!("{value:b}"),  "10111011i8");
	assert_eq!(format!("{value:#b}"), "0b10111011i8");
	assert_eq!(format!("{value:o}"),  "273i8");
	assert_eq!(format!("{value:#o}"), "0o273i8");
	assert_eq!(format!("{value:x}"),  "bbi8");
	assert_eq!(format!("{value:#x}"), "0xbbi8");
	assert_eq!(format!("{value:X}"),  "BBi8");
	assert_eq!(format!("{value:#X}"), "0xBBi8");
}
