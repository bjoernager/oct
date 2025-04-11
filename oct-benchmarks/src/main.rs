// Copyright (c) 2024-2025 Gabriel Bjørnager Jensen.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

const _: () = assert!(usize::BITS >= u32::BITS);

mod benchmark;
mod stats;
mod suite;

pub use benchmark::{Benchmark, BenchmarkInit, BenchmarkMain};
pub use stats::Stats;
pub use suite::Suite;

use rand::distr::{Distribution, StandardUniform};
use rand::Rng;

#[derive(bincode::Decode, bincode::Encode)]
#[derive(borsh::BorshDeserialize, borsh::BorshSerialize)]
#[derive(oct::decode::Decode, oct::encode::Encode, oct::encode::SizedEncode)]
#[derive(serde::Deserialize, serde::Serialize)]
#[repr(transparent)]
struct Unit;

impl Distribution<Unit> for StandardUniform {
	fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> Unit {
		Unit
	}
}

#[derive(bincode::Decode, bincode::Encode)]
#[derive(borsh::BorshDeserialize, borsh::BorshSerialize)]
#[derive(oct::decode::Decode, oct::encode::Encode, oct::encode::SizedEncode)]
#[derive(serde::Deserialize, serde::Serialize)]
#[repr(transparent)]
struct Unnamed(u32);

impl Distribution<Unnamed> for StandardUniform {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Unnamed {
		let c: char = self.sample(rng);
		Unnamed(c.into())
	}
}

#[derive(bincode::Decode, bincode::Encode)]
#[derive(borsh::BorshDeserialize, borsh::BorshSerialize)]
#[derive(oct::decode::Decode, oct::encode::Encode, oct::encode::SizedEncode)]
#[derive(serde::Deserialize, serde::Serialize)]
#[repr(transparent)]
struct Named { buf: [u8; 0x8] }

impl Distribution<Named> for StandardUniform {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Named {
		let i: u64 = self.sample(rng);
		Named { buf: i.to_be_bytes() }
	}
}

#[repr(u8)]
#[derive(bincode::Decode, bincode::Encode)]
#[derive(borsh::BorshDeserialize, borsh::BorshSerialize)]
#[derive(oct::decode::Decode, oct::encode::Encode, oct::encode::SizedEncode)]
#[derive(serde::Deserialize, serde::Serialize)]
enum Enum {
	None,
	Unit(Unit),
	Unnamed(Unnamed),
	Named(Named),
}

impl Distribution<Enum> for StandardUniform {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Enum {
		let d = <Self as Distribution<u8>>::sample(self, rng) & 0b00000011;

		match d {
			0x0 => Enum::None,
			0x1 => Enum::Unit(   self.sample(rng)),
			0x2 => Enum::Unnamed(self.sample(rng)),
			0x3 => Enum::Named(  self.sample(rng)),

			_ => unreachable!(),
		}
	}
}

fn main() {
	eprintln!("##################");
	eprintln!("# OCT BENCHMARKS #");
	eprintln!("##################");
	eprintln!();

	let mut benchmarks = Suite::new(0xFFFFFFF, 0x4);

	benchmarks.push_benchmark(
		benchmark! {
			name: "encode_u8",
			kind: encode,
			ty:   u8,
		},
	);

	benchmarks.push_benchmark(
		benchmark! {
			name: "encode_u16",
			kind: encode,
			ty:   u16,
		},
	);

	benchmarks.push_benchmark(
		benchmark! {
			name: "encode_u32",
			kind: encode,
			ty:   u32,
		},
	);

	benchmarks.push_benchmark(
		benchmark! {
			name: "encode_u64",
			kind: encode,
			ty:   u64,
		},
	);

	benchmarks.push_benchmark(
		benchmark! {
			name: "encode_u128",
			kind: encode,
			ty:   u128,
		},
	);

	benchmarks.push_benchmark(
		benchmark! {
			name: "encode_bool",
			kind: encode,
			ty:   bool,
		},
	);

	benchmarks.push_benchmark(
		benchmark! {
			name: "encode_unit_struct",
			kind: encode,
			ty:   Unit,
		},
	);

	benchmarks.push_benchmark(
		benchmark! {
			name: "encode_newtype",
			kind: encode,
			ty:   Unnamed,
		},
	);

	benchmarks.push_benchmark(
		benchmark! {
			name: "encode_struct",
			kind: encode,
			ty:   Named,
		},
	);

	benchmarks.push_benchmark(
		benchmark! {
			name: "encode_enum",
			kind: encode,
			ty:   Enum,
		},
	);

	benchmarks.push_benchmark(
		benchmark! {
			name: "decode_u8",
			kind: decode,
			ty:   u8,
		},
	);

	benchmarks.push_benchmark(
		benchmark! {
			name: "decode_u16",
			kind: decode,
			ty:   u16,
		},
	);

	benchmarks.push_benchmark(
		benchmark! {
			name: "decode_u32",
			kind: decode,
			ty:   u32,
		},
	);

	benchmarks.push_benchmark(
		benchmark! {
			name: "decode_u64",
			kind: decode,
			ty:   u64,
		},
	);

	benchmarks.push_benchmark(
		benchmark! {
			name: "decode_u128",
			kind: decode,
			ty:   u128,
		},
	);

	benchmarks.push_benchmark(
		benchmark! {
			name: "decode_bool",
			kind: decode,
			ty:   bool,
		},
	);

	benchmarks.push_benchmark(
		benchmark! {
			name: "decode_unit_struct",
			kind: decode,
			ty:   Unit,
		},
	);

	benchmarks.push_benchmark(
		benchmark! {
			name: "decode_newtype",
			kind: decode,
			ty:   Unnamed,
		},
	);

	benchmarks.push_benchmark(
		benchmark! {
			name: "decode_struct",
			kind: decode,
			ty:   Named,
		},
	);

	// Does not work with Borsh.
//	benchmarks.push_benchmark(
//		benchmark! {
//			name: "decode_enum",
//			kind: decode,
//			ty:   Enum,
//		},
//	);

	benchmarks.run();
}
