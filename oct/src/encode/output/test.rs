// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

#![cfg(test)]

use oct::encode::Output;

#[test]
fn test_encode_output() {
	let mut buf = [0x00; 0x4];

	let mut output = Output::new(&mut buf);

	assert_eq!(output.capacity(),  0x4);
	assert_eq!(output.position(),  0x0);
	assert_eq!(output.remaining(), 0x4);

	assert_eq!(output, [].as_slice());
	output.write(&[0x04]);

	assert_eq!(output.capacity(),  0x4);
	assert_eq!(output.position(),  0x1);
	assert_eq!(output.remaining(), 0x3);

	assert_eq!(output, [0x04].as_slice());
	output.write(&[0x03]);

	assert_eq!(output.capacity(),  0x4);
	assert_eq!(output.position(),  0x2);
	assert_eq!(output.remaining(), 0x2);

	assert_eq!(output, [0x04, 0x03].as_slice());
	output.write(&[0x02]);

	assert_eq!(output.capacity(),  0x4);
	assert_eq!(output.position(),  0x3);
	assert_eq!(output.remaining(), 0x1);

	assert_eq!(output, [0x04, 0x03, 0x02].as_slice());
	output.write(&[0x01]);

	assert_eq!(output.capacity(),  0x4);
	assert_eq!(output.position(),  0x4);
	assert_eq!(output.remaining(), 0x0);

	assert_eq!(output, [0x04, 0x03, 0x02, 0x01].as_slice());
}

#[should_panic]
#[test]
fn test_encode_output_empty() {
	let mut output = Output::new(&mut []);

	output.write(&[0x00]);
}
