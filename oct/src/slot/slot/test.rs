// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use oct::error::CharDecodeError;
use oct::slot::Slot;

#[test]
fn test_buf_write_read() {
	let mut buf = Slot::<char>::new();

	macro_rules! test_read {
		($pattern:pat$(,)?) => {{
			match buf.read() {
				$pattern => { }

				value => panic!("value `{value:?}` does not match pattern `{}`", stringify!($pattern)),
			}
		}};
	}

	buf.write('\u{1F44D}').unwrap();
	assert_eq!(buf, [0x4D, 0xF4, 0x01, 0x00].as_slice());

	buf.copy_from_slice(&[0x00, 0xD8, 0x00, 0x00]);
	test_read!(Err(CharDecodeError { code_point: 0xD800 }));

	buf.copy_from_slice(&[0x3A, 0xFF, 0x00, 0x00]);
	test_read!(Ok('\u{FF3A}'));
}
