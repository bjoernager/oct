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

use crate::Stats;

use indexmap::IndexMap;
use std::time::Instant;

pub type BenchmarkInit = fn(u32) -> Vec<u8>;
pub type BenchmarkMain = fn(Vec<u8>, u32);

#[macro_export]
macro_rules! benchmark {
	{
		name: $name:expr,
		kind: encode,
		ty:   $ty:ty$(,)?
	} => {{
		use $crate::{BenchmarkInit, BenchmarkMain};

		let bincode_init: BenchmarkInit = |_count| {
			use ::std::vec::Vec;

			Vec::<u8>::new()
		};

		let bincode_main: BenchmarkMain = |mut data: ::std::vec::Vec<u8>, count| {
			use ::bincode::encode_into_std_write;
			use ::bincode::config;
			use ::rand::random;

			for _ in 0x0..count {
				encode_into_std_write(random::<$ty>(), &mut data, config::standard()).unwrap();
			}
		};

		let borsh_init: BenchmarkInit = |_count| {
			use ::std::vec::Vec;

			Vec::<u8>::new()
		};

		let borsh_main: BenchmarkMain = |mut data: ::std::vec::Vec<u8>, count| {
			use ::borsh::to_writer;
			use ::rand::random;

			for _ in 0x0..count {
				to_writer(&mut data, &random::<$ty>()).unwrap();
			}
		};

		let oct_init: BenchmarkInit = |count| {
			use ::oct::encode::SizedEncode;
			use ::std::vec::Vec;

			let len = <$ty>::MAX_ENCODED_SIZE * count as usize;

			let mut data = Vec::with_capacity(len);
			data.resize(len, Default::default());

			data
		};

		let oct_main: BenchmarkMain = |mut data: ::std::vec::Vec<u8>, count| {
			use ::oct::encode::{Encode, Output};
			use ::rand::random;

			let mut output = Output::new(&mut data);

			for _ in 0x0..count {
				Encode::encode(&random::<$ty>(), &mut output).unwrap();
			}
		};

		let postcard_init: BenchmarkInit = |_count| {
			use ::std::vec::Vec;

			Vec::<u8>::new()
		};

		let postcard_main: BenchmarkMain = |mut data: ::std::vec::Vec<u8>, count| {
			use ::postcard::to_io;
			use ::rand::random;

			for _ in 0x0..count {
				to_io(&random::<$ty>(), &mut data).unwrap();
			}
		};

		let code = [
			(
				"oct",
				(oct_init, oct_main),
			),
			(
				"bincode",
				(bincode_init, bincode_main),
			),
			(
				"borsh",
				(borsh_init, borsh_main),
			),
			(
				"postcard",
				(postcard_init, postcard_main),
			),
		];

		$crate::Benchmark {
			name: $name,

			code: code.into(),
		}
	}};

	{
		name: $name:expr,
		kind: decode,
		ty:   $ty:ty$(,)?
	} => {{
		let bincode_init: BenchmarkInit = |count| {
			use ::bincode::encode_into_std_write;
			use ::bincode::config;
			use ::rand::random;
			use ::std::vec::Vec;

			let mut data = Vec::<u8>::new();

			for _ in 0x0..count {
				encode_into_std_write(random::<$ty>(), &mut data, config::standard()).unwrap();
			}

			data
		};

		let bincode_main: BenchmarkMain = |data: ::std::vec::Vec<u8>, count| {
			use ::bincode::decode_from_std_read;
			use ::bincode::config;
			use ::std::io::Cursor;

			let mut data = Cursor::new(data);

			for _ in 0x0..count {
				let _: $ty = decode_from_std_read(&mut data, config::standard()).unwrap();
			}
		};

		let borsh_init: BenchmarkInit = |count| {
			use ::borsh::to_writer;
			use ::rand::random;
			use ::std::vec::Vec;

			let mut data = Vec::<u8>::new();

			for _ in 0x0..count {
				to_writer(&mut data, &random::<$ty>()).unwrap();
			}

			data
		};

		let borsh_main: BenchmarkMain = |data: ::std::vec::Vec<u8>, count| {
			use ::borsh::from_slice;
			use ::std::io::{Cursor, Read};

			let mut data = Cursor::new(data);

			for _ in 0x0..count {
				let mut buf = [0x00; size_of::<$ty>()];
				data.read_exact(&mut buf).unwrap();

				let _: $ty = from_slice(&buf).unwrap();
			}
		};

		let oct_init: BenchmarkInit = |count| {
			use ::oct::encode::{Encode, Output, SizedEncode};
			use ::rand::random;
			use ::std::vec::Vec;

			let len = <$ty>::MAX_ENCODED_SIZE * count as usize;

			let mut data = Vec::with_capacity(len);
			data.resize(len, Default::default());

			let mut output = Output::new(&mut data);

			for _ in 0x0..count {
				Encode::encode(&random::<$ty>(), &mut output).unwrap();
			}

			data
		};

		let oct_main: BenchmarkMain = |data: ::std::vec::Vec<u8>, count| {
			use ::oct::decode::{Decode, Input};

			let mut input = Input::new(&data);

			for _ in 0x0..count {
				let _: $ty = Decode::decode(&mut input).unwrap();
			}
		};

		let postcard_init: BenchmarkInit = |count| {
			use ::postcard::to_io;
			use ::rand::random;
			use ::std::vec::Vec;

			let mut data = Vec::<u8>::new();

			for _ in 0x0..count {
				to_io(&random::<$ty>(), &mut data).unwrap();
			}

			data
		};

		let postcard_main: BenchmarkMain = |data: ::std::vec::Vec<u8>, count| {
			use ::postcard::take_from_bytes;

			let mut data = data.as_slice();

			for _ in 0x0..count {
				let _value: $ty;
				(_value, data) = take_from_bytes(data).unwrap();
			}
		};

		let code = [
			(
				"oct",
				(oct_init, oct_main),
			),
			(
				"bincode",
				(bincode_init, bincode_main),
			),
			(
				"borsh",
				(borsh_init, borsh_main),
			),
			(
				"postcard",
				(postcard_init, postcard_main),
			),
		];

		$crate::Benchmark {
			name: $name,

			code: code.into(),
		}
	}};
}

pub struct Benchmark {
	pub name: &'static str,

	pub code: IndexMap<&'static str, (BenchmarkInit, BenchmarkMain)>,
}

impl Benchmark {
	pub fn run(self, value_count: u32, run_count: u32) {
		assert!(value_count != 0x0);
		assert!(run_count   != 0x0);

		eprintln!("- running benchmark `{}`:", self.name);

		fn print_stats(stats: Stats, ref_stats: Option<Stats>) {
			eprint!(" {:>9.3}s", stats.main_duration.as_secs_f64());

			if let Some(ref_stats) = ref_stats {
				let difference = (stats.main_duration.as_secs_f64() / ref_stats.main_duration.as_secs_f64() - 1.0) * 100.0;

				let colour: u8 = if difference >= 0.0 {
					0x20
				} else if difference < 0.0 {
					0x1F
				} else {
					0x00
				};

				let difference = format!("{difference:+.2}");

				eprint!(" => \u{001B}[{colour:03}m{difference:>8}%\u{001B}[000m");
			}

			eprintln!();
		}

		let mut ref_stats = None;

		for (crate_name, (init, main)) in self.code {
			let crate_name = format!("`{crate_name}`");

			eprint!("--- {crate_name:<16}");

			let mut stats = Stats {
				value_count,
				run_count,

				..Default::default()
			};

			for i in 0x0..run_count {
				eprint!(" {i}...");

				let init_start = Instant::now();
				let data = init(value_count);
				let init_end = Instant::now();

				stats.data_size += data.len();

				let main_start = Instant::now();
				main(data, value_count);
				let main_end = Instant::now();

				stats.init_duration += init_end.duration_since(init_start);
				stats.main_duration += main_end.duration_since(main_start);
			}

			stats.data_size /= run_count as usize;

			stats.init_duration /= run_count;
			stats.main_duration /= run_count;

			print_stats(stats, ref_stats);

			ref_stats.get_or_insert(stats);
		}
	}
}
