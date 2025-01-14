// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use core::convert::Infallible;
use core::marker::PhantomData;
use core::net::{
	IpAddr,
	Ipv4Addr,
	Ipv6Addr,
	SocketAddr,
	SocketAddrV4,
	SocketAddrV6,
};
use core::num::NonZero;
use oct::encode::{Encode, SizedEncode};

macro_rules! assert_encoded_size {
	($ty:ty, $value:expr$(,)?) => {{
		assert_eq!(<$ty as ::oct::encode::SizedEncode>::MAX_ENCODED_SIZE, $value);
	}};
}

#[test]
fn test_sized_encode() {
	assert_encoded_size!(bool,                      0x1);
	assert_encoded_size!(char,                      0x4);
	assert_encoded_size!(f32,                       0x4);
	assert_encoded_size!(f64,                       0x8);
	assert_encoded_size!(i128,                      0x10);
	assert_encoded_size!(i16,                       0x2);
	assert_encoded_size!(i32,                       0x4);
	assert_encoded_size!(i64,                       0x8);
	assert_encoded_size!(i8,                        0x1);
	assert_encoded_size!(isize,                     0x2);
	assert_encoded_size!(Infallible,                0x0);
	assert_encoded_size!(IpAddr,                    0x11);
	assert_encoded_size!(Ipv4Addr,                  0x4);
	assert_encoded_size!(Ipv6Addr,                  0x10);
	assert_encoded_size!(Option<NonZero<i128>>,     0x11);
	assert_encoded_size!(Option<NonZero<i16>>,      0x3);
	assert_encoded_size!(Option<NonZero<i32>>,      0x5);
	assert_encoded_size!(Option<NonZero<i64>>,      0x9);
	assert_encoded_size!(Option<NonZero<i8>>,       0x2);
	assert_encoded_size!(Option<NonZero<isize>>,    0x3);
	assert_encoded_size!(Option<NonZero<u128>>,     0x11);
	assert_encoded_size!(Option<NonZero<u16>>,      0x3);
	assert_encoded_size!(Option<NonZero<u32>>,      0x5);
	assert_encoded_size!(Option<NonZero<u64>>,      0x9);
	assert_encoded_size!(Option<NonZero<u8>>,       0x2);
	assert_encoded_size!(Option<NonZero<usize>>,    0x3);
	assert_encoded_size!(PhantomData<[u128; 0x10]>, 0x0);
	assert_encoded_size!(SocketAddr,                0x1B);
	assert_encoded_size!(SocketAddrV4,              0x6);
	assert_encoded_size!(SocketAddrV6,              0x1A);
	assert_encoded_size!(u128,                      0x10);
	assert_encoded_size!(u16,                       0x2);
	assert_encoded_size!(u32,                       0x4);
	assert_encoded_size!(u64,                       0x8);
	assert_encoded_size!(u8,                        0x1);
	assert_encoded_size!(usize,                     0x2);
	assert_encoded_size!((),                        0x0);
}

#[test]
fn test_sized_encode_derive() {
	#[derive(Encode, SizedEncode)]
	struct Foo(char);

	#[derive(Encode, SizedEncode)]
	#[expect(dead_code)]
	#[repr(i64)]
	enum Bar {
		Unit = 0x45,

		Pretty(bool) = 127,

		Teacher { initials: [char; 0x3] },
	}

	assert_encoded_size!(Foo, 0x4);
	assert_encoded_size!(Bar, 0x14);
}
