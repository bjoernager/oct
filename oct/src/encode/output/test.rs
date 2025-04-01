// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

#![cfg(test)]

use oct::encode::Output;
use oct::error::OutputError;

#[test]
fn test_encode_output() {
	let mut buf = [0x00; 0x4];

	let mut output = Output::new(&mut buf);

	assert_eq!(output.capacity(),  0x4);
	assert_eq!(output.position(),  0x0);
	assert_eq!(output.remaining(), 0x4);

	assert_eq!(output, [].as_slice());
	assert_eq!(output.write(&[0x04]), Ok(()));

	assert_eq!(output.capacity(),  0x4);
	assert_eq!(output.position(),  0x1);
	assert_eq!(output.remaining(), 0x3);

	assert_eq!(output, [0x04].as_slice());
	assert_eq!(output.write(&[0x03]), Ok(()));

	assert_eq!(output.capacity(),  0x4);
	assert_eq!(output.position(),  0x2);
	assert_eq!(output.remaining(), 0x2);

	assert_eq!(output, [0x04, 0x03].as_slice());
	assert_eq!(output.write(&[0x02]), Ok(()));

	assert_eq!(output.capacity(),  0x4);
	assert_eq!(output.position(),  0x3);
	assert_eq!(output.remaining(), 0x1);

	assert_eq!(output, [0x04, 0x03, 0x02].as_slice());
	assert_eq!(output.write(&[0x01]), Ok(()));

	assert_eq!(output.capacity(),  0x4);
	assert_eq!(output.position(),  0x4);
	assert_eq!(output.remaining(), 0x0);

	assert_eq!(output, [0x04, 0x03, 0x02, 0x01].as_slice());
	assert_eq!(output.write(&[0x00]), Err(OutputError { capacity: 0x4, position: 0x4, count: 0x1 }));

	assert_eq!(output.write(&[]), Ok(()));
}
