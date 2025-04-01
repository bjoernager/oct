// MIT License
//
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

use core::array;
use core::num::NonZero;
use rand::random;
use rand::distributions::{Distribution, Standard};
use std::time::Instant;
use zerocopy::{Immutable, IntoBytes};

const TEST_COUNT: u32 = 0x4;

const VALUE_COUNT: usize = 0xFFFFFFF;

macro_rules! benchmark {
	{
		$($name:ident: {
			bincode: $bincode_op:block$(,)?

			borsh: $borsh_op:block$(,)?

			oct: $oct_op:block$(,)?

			postcard: $postcard_op:block$(,)?
		}$(,)?)+
	 } => {{
		use ::core::{concat, stringify};
		use ::std::{eprint, eprintln};

		macro_rules! time {
			{ $op: block } => {{
				let start = Instant::now();
				$op
				let stop = Instant::now();

				let duration = stop - start;
				(duration.as_nanos() as f64) / 1_000_000_000.0
			}};
		}

		fn format_score(duration: f64, ref_duration: f64) -> String {
			let vps = (VALUE_COUNT as f64 / duration).round();

			let difference = (duration / ref_duration - 1.0) * 100.0;

			let colour: u8 = if difference >= 0.0 {
				0x20
			} else if difference < 0.0 {
				0x1F
			} else {
				0x00
			};

			format!("{duration:.3}s ({vps:.0} vps) => \u{001B}[{colour}m{difference:+.2}%\u{001B}[000m")
		}

		let mut total_bincode_duration  = 0.0;
		let mut total_borsh_duration    = 0.0;
		let mut total_oct_duration      = 0.0;
		let mut total_postcard_duration = 0.0;

		$({
			eprintln!();
			eprint!(concat!("\u{001B}[001mrunning benchmark `", stringify!($name), "`...\u{001B}[022m"));

			let mut bincode_duration  = 0.0;
			let mut borsh_duration    = 0.0;
			let mut oct_duration      = 0.0;
			let mut postcard_duration = 0.0;

			for i in 0x0..TEST_COUNT {
				eprint!(" {i}...");

				eprint!("\u{001B}[093m");

				bincode_duration  += time! { $bincode_op };
				borsh_duration    += time! { $borsh_op };
				oct_duration      += time! { $oct_op };
				postcard_duration += time! { $postcard_op };

				eprint!("\u{001B}[000m");
			}

			eprintln!();

			bincode_duration  /= f64::from(TEST_COUNT);
			borsh_duration    /= f64::from(TEST_COUNT);
			oct_duration      /= f64::from(TEST_COUNT);
			postcard_duration /= f64::from(TEST_COUNT);

			eprint!("\u{001B}[000m");
			eprintln!("bincode:  {}", format_score(bincode_duration,  oct_duration));
			eprintln!("borsh:    {}", format_score(borsh_duration,    oct_duration));
			eprintln!("oct:      {}", format_score(oct_duration,      oct_duration));
			eprintln!("postcard: {}", format_score(postcard_duration, oct_duration));

			total_bincode_duration  += bincode_duration;
			total_borsh_duration    += borsh_duration;
			total_oct_duration   += oct_duration;
			total_postcard_duration += postcard_duration;
		})*

		eprintln!();
		eprintln!("\u{001B}[001mtotal score:\u{001B}[022m");
		eprintln!("bincode:  {}", format_score(total_bincode_duration,  total_oct_duration));
		eprintln!("borsh:    {}", format_score(total_borsh_duration,    total_oct_duration));
		eprintln!("oct:      {}", format_score(total_oct_duration,      total_oct_duration));
		eprintln!("postcard: {}", format_score(total_postcard_duration, total_oct_duration));
	}};
}

#[derive(bincode::Decode, bincode::Encode)]
#[derive(borsh::BorshSerialize)]
#[derive(oct::decode::Decode, oct::encode::Encode, oct::encode::SizedEncode)]
#[derive(serde::Deserialize, serde::Serialize)]
#[repr(transparent)]
struct Unit;

#[derive(bincode::Decode, bincode::Encode)]
#[derive(borsh::BorshSerialize)]
#[derive(oct::decode::Decode, oct::encode::Encode, oct::encode::SizedEncode)]
#[derive(serde::Deserialize, serde::Serialize)]
#[repr(transparent)]
struct Unnamed(u32);

impl Unnamed {
	#[inline(always)]
	#[must_use]
	pub const fn from_char(value: char) -> Self {
		Self(value as u32)
	}
}

#[derive(bincode::Decode, bincode::Encode)]
#[derive(borsh::BorshSerialize)]
#[derive(oct::decode::Decode, oct::encode::Encode, oct::encode::SizedEncode)]
#[derive(serde::Deserialize, serde::Serialize)]
#[repr(transparent)]
struct Named { buf: [u8; 0x8] }

impl Named {
	#[inline(always)]
	#[must_use]
	pub const fn from_u64(value: u64) -> Self {
		let buf = [
			 (value & 0x00_00_00_00_00_00_00_FF)         as u8,
			((value & 0x00_00_00_00_00_00_FF_00) >> 0x2) as u8,
			((value & 0x00_00_00_00_00_FF_00_00) >> 0x4) as u8,
			((value & 0x00_00_00_00_FF_00_00_00) >> 0x6) as u8,
			((value & 0x00_00_00_FF_00_00_00_00) >> 0x8) as u8,
			((value & 0x00_00_FF_00_00_00_00_00) >> 0xA) as u8,
			((value & 0x00_FF_00_00_00_00_00_00) >> 0xC) as u8,
			((value & 0xFF_00_00_00_00_00_00_00) >> 0xE) as u8,
		];

		Self { buf }
	}
}

#[derive(bincode::Decode, bincode::Encode)]
#[derive(borsh::BorshSerialize)]
#[derive(oct::decode::Decode, oct::encode::Encode, oct::encode::SizedEncode)]
#[derive(serde::Deserialize, serde::Serialize)]
enum Enum {
	Unit(Unit),
	Unnamed(Unnamed),
	Named(Named),
}

fn generate_random_data<T>(item_size: usize, value_count: usize) -> impl Iterator<Item = u8>
where
	T:        Immutable + IntoBytes + Sized,
	Standard: Distribution<T>,
{
	let count = item_size * value_count;

	let mut data = Vec::new();

	for _ in 0x0..count {
		let value = random::<T>();

		data.extend(value.as_bytes());
	}

	data.into_iter()
}

fn main() {
	println!("##################");
	println!("# OCT BENCHMARKS #");
	println!("##################");
	println!();
	println!("Each benchmark has a version written for the following crates:");
	println!();
	println!("- Bincode:  <https://crates.io/crates/bincode/>");
	println!("- Borsh:    <https://crates.io/crates/borsh/>");
	println!("- Oct:      <https://crates.io/crates/oct/>");
	println!("- Postcard: <https://crates.io/crates/postcard/>");
	println!();
	println!("The total time the benchmark took (including memory allocations and dealloca-");
	println!("tions) is printed when the benchmark has completed. The `vps` counter signifies");
	println!("the amount of values per second that the benchmark has handled (during the en-");
	println!("tirety of the benchmark).");
	println!();
	println!("When every benchmark has concluded, the total run time and vps is listed for");
	println!("each crate. A percantage additionally compares the run time between the listed");
	println!("crate and Oct (which should always be c. `0%` for oct itself). DO NOTE THAT");
	println!("THESE FINAL RESULTS INDICATE A NON-WEIGHTED AVERAGE ACROSS BENCHMARKS. It can");
	println!("therefore be skewed relative to real-world performance by the similarity of some");
	println!("benchmarks.");
	println!();

	eprintln!("test_count:  {TEST_COUNT}");
	eprintln!("value_count: {VALUE_COUNT}");

	benchmark! {
		encode_u8: {
			bincode: {
				// Requires `std`.

				use bincode::encode_into_std_write;
				use bincode::config;

				const ITEM_SIZE: usize = size_of::<u8>();

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT];

				for _ in 0x0..VALUE_COUNT {
					encode_into_std_write(random::<u8>(), &mut buf, config::standard()).unwrap();
				}
			}

			borsh: {
				const ITEM_SIZE: usize = size_of::<u8>();

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT];

				for _ in 0x0..VALUE_COUNT {
					borsh::to_writer(&mut buf, &random::<u8>()).unwrap();
				}
			}

			oct: {
				use oct::encode::{Encode, Output, SizedEncode};

				const ITEM_SIZE: usize = u8::MAX_ENCODED_SIZE;

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT].into_boxed_slice();
				let mut stream = Output::new(&mut buf);

				for _ in 0x0..VALUE_COUNT {
					random::<u8>().encode(&mut stream).unwrap();
				}
			}

			postcard: {
				const ITEM_SIZE: usize = size_of::<u8>();

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT];

				for _ in 0x0..VALUE_COUNT {
					postcard::to_io(&random::<u8>(), &mut buf).unwrap();
				}
			}
		}

		encode_u32: {
			bincode: {
				use bincode::encode_into_std_write;
				use bincode::config;

				const ITEM_SIZE: usize = size_of::<u32>();

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT];

				for _ in 0x0..VALUE_COUNT {
					encode_into_std_write(random::<u32>(), &mut buf, config::standard()).unwrap();
				}
			}

			borsh: {
				const ITEM_SIZE: usize = size_of::<u32>();

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT];

				for _ in 0x0..VALUE_COUNT {
					borsh::to_writer(&mut buf, &random::<u32>()).unwrap();
				}
			}

			oct: {
				use oct::encode::{Encode, Output, SizedEncode};

				const ITEM_SIZE: usize = u32::MAX_ENCODED_SIZE;

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT].into_boxed_slice();
				let mut stream = Output::new(&mut buf);

				for _ in 0x0..VALUE_COUNT {
					random::<u32>().encode(&mut stream).unwrap();
				}
			}

			postcard: {
				const ITEM_SIZE: usize = size_of::<u32>();

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT];

				for _ in 0x0..VALUE_COUNT {
					postcard::to_io(&random::<u32>(), &mut buf).unwrap();
				}
			}
		}

		encode_u128: {
			bincode: {
				use bincode::encode_into_std_write;
				use bincode::config;

				const ITEM_SIZE: usize = size_of::<u128>();

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT];

				for _ in 0x0..VALUE_COUNT {
					encode_into_std_write(random::<u128>(), &mut buf, config::standard()).unwrap();
				}
			}

			borsh: {
				const ITEM_SIZE: usize = size_of::<u128>();

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT];

				for _ in 0x0..VALUE_COUNT {
					borsh::to_writer(&mut buf, &random::<u128>()).unwrap();
				}
			}

			oct: {
				use oct::encode::{Encode, Output, SizedEncode};

				const ITEM_SIZE: usize = u128::MAX_ENCODED_SIZE;

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT].into_boxed_slice();
				let mut stream = Output::new(&mut buf);

				for _ in 0x0..VALUE_COUNT {
					random::<u128>().encode(&mut stream).unwrap();
				}
			}

			postcard: {
				const ITEM_SIZE: usize = size_of::<u128>();

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT];

				for _ in 0x0..VALUE_COUNT {
					postcard::to_io(&random::<u128>(), &mut buf).unwrap();
				}
			}
		}

		encode_char: {
			bincode: {
				use bincode::encode_into_std_write;
				use bincode::config;

				const ITEM_SIZE: usize = size_of::<char>();

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT];

				for _ in 0x0..VALUE_COUNT {
					encode_into_std_write(random::<char>(), &mut buf, config::standard()).unwrap();
				}
			}

			borsh: {
				const ITEM_SIZE: usize = size_of::<char>();

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT];

				for _ in 0x0..VALUE_COUNT {
					borsh::to_writer::<u32, _>(&mut buf, &random::<char>().into()).unwrap();
				}
			}

			oct: {
				use oct::encode::{Encode, Output, SizedEncode};

				const ITEM_SIZE: usize = char::MAX_ENCODED_SIZE;

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT].into_boxed_slice();
				let mut stream = Output::new(&mut buf);

				for _ in 0x0..VALUE_COUNT {
					random::<char>().encode(&mut stream).unwrap();
				}
			}

			postcard: {
				const ITEM_SIZE: usize = size_of::<char>();

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT];

				for _ in 0x0..VALUE_COUNT {
					postcard::to_io(&random::<char>(), &mut buf).unwrap();
				}
			}
		}

		encode_struct_unit: {
			bincode: {
				use bincode::encode_into_std_write;
				use bincode::config;

				const ITEM_SIZE: usize = size_of::<Unit>();

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT];

				for _ in 0x0..VALUE_COUNT {
					encode_into_std_write(Unit, &mut buf, config::standard()).unwrap();
				}
			}

			borsh: {
				const ITEM_SIZE: usize = size_of::<Unit>();

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT];

				for _ in 0x0..VALUE_COUNT {
					borsh::to_writer(&mut buf, &Unit).unwrap();
				}
			}

			oct: {
				use oct::encode::{Encode, Output, SizedEncode};

				const ITEM_SIZE: usize = Unit::MAX_ENCODED_SIZE;

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT].into_boxed_slice();
				let mut stream = Output::new(&mut buf);

				for _ in 0x0..VALUE_COUNT {
					Unit.encode(&mut stream).unwrap();
				}
			}

			postcard: {
				const ITEM_SIZE: usize = size_of::<Unit>();

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT];

				for _ in 0x0..VALUE_COUNT {
					postcard::to_io(&Unit, &mut buf).unwrap();
				}
			}
		}

		encode_struct_unnamed: {
			bincode: {
				use bincode::encode_into_std_write;
				use bincode::config;

				const ITEM_SIZE: usize = size_of::<Unnamed>();

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT];

				for _ in 0x0..VALUE_COUNT {
					encode_into_std_write(Unnamed::from_char(random()), &mut buf, config::standard()).unwrap();
				}
			}

			borsh: {
				const ITEM_SIZE: usize = size_of::<Unnamed>();

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT];

				for _ in 0x0..VALUE_COUNT {
					borsh::to_writer(&mut buf, &Unnamed::from_char(random())).unwrap();
				}
			}

			oct: {
				use oct::encode::{Encode, Output, SizedEncode};

				const ITEM_SIZE: usize = Unnamed::MAX_ENCODED_SIZE;

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT].into_boxed_slice();
				let mut stream = Output::new(&mut buf);

				for _ in 0x0..VALUE_COUNT {
					Unnamed::from_char(random()).encode(&mut stream).unwrap();
				}
			}

			postcard: {
				const ITEM_SIZE: usize = size_of::<Unnamed>();

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT];

				for _ in 0x0..VALUE_COUNT {
					postcard::to_io(&Unnamed::from_char(random()), &mut buf).unwrap();
				}
			}
		}

		encode_struct_named: {
			bincode: {
				use bincode::encode_into_std_write;
				use bincode::config;

				const ITEM_SIZE: usize = size_of::<Named>();

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT];

				for _ in 0x0..VALUE_COUNT {
					encode_into_std_write(Named::from_u64(random()), &mut buf, config::standard()).unwrap();
				}
			}

			borsh: {
				const ITEM_SIZE: usize = size_of::<Named>();

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT];

				for _ in 0x0..VALUE_COUNT {
					borsh::to_writer(&mut buf, &Named::from_u64(random())).unwrap();
				}
			}

			oct: {
				use oct::encode::{Encode, Output, SizedEncode};

				const ITEM_SIZE: usize = Named::MAX_ENCODED_SIZE;

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT].into_boxed_slice();
				let mut stream = Output::new(&mut buf);

				for _ in 0x0..VALUE_COUNT {
					Named::from_u64(random()).encode(&mut stream).unwrap();
				}
			}

			postcard: {
				const ITEM_SIZE: usize = size_of::<Named>();

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT];

				for _ in 0x0..VALUE_COUNT {
					postcard::to_io(&Named::from_u64(random()), &mut buf).unwrap();
				}
			}
		}

		encode_enum_unit: {
			bincode: {
				use bincode::encode_into_std_write;
				use bincode::config;

				const ITEM_SIZE: usize =
					size_of::<u32>() // discriminant
					+ size_of::<Unit>();

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT];

				for _ in 0x0..VALUE_COUNT {
					encode_into_std_write(Enum::Unit(Unit), &mut buf, config::standard()).unwrap();
				}
			}

			borsh: {
				const ITEM_SIZE: usize =
					size_of::<u8>() // discriminant
					+ size_of::<Unit>();

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT];

				for _ in 0x0..VALUE_COUNT {
					borsh::to_writer(&mut buf, &Enum::Unit(Unit)).unwrap();
				}
			}

			oct: {
				use oct::encode::{Encode, Output, SizedEncode};

				const ITEM_SIZE: usize =
					isize::MAX_ENCODED_SIZE // discriminant
					+ Unit::MAX_ENCODED_SIZE;

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT].into_boxed_slice();
				let mut stream = Output::new(&mut buf);

				for _ in 0x0..VALUE_COUNT {
					Enum::Unit(Unit).encode(&mut stream).unwrap();
				}
			}

			postcard: {
				const ITEM_SIZE: usize =
					size_of::<u32>() // discriminant
					+ size_of::<Unit>();

				let mut buf = vec![0x00; ITEM_SIZE * VALUE_COUNT];

				for _ in 0x0..VALUE_COUNT {
					postcard::to_io(&Enum::Unit(Unit), &mut buf).unwrap();
				}
			}
		}

		decode_u8: {
			bincode: {
				use bincode::decode_from_slice;
				use bincode::config;

				const ITEM_SIZE: usize = size_of::<u8>();

				let buf: Box<[_]> = generate_random_data::<u8>(ITEM_SIZE, VALUE_COUNT).collect();

				for i in 0x0..VALUE_COUNT {
					let data = array::from_ref(&buf[i]).as_slice();

					let _: (u8, usize) = decode_from_slice(data, config::standard()).unwrap();
				}
			}

			borsh: {
				const ITEM_SIZE: usize = size_of::<u8>();

				let buf: Box<[_]> = generate_random_data::<u8>(ITEM_SIZE, VALUE_COUNT).collect();

				for i in 0x0..VALUE_COUNT {
					let data = array::from_ref(&buf[i]).as_slice();

					let _: u8 = borsh::from_slice(data).unwrap();
				}
			}

			oct: {
				use oct::decode::{Decode, Input};
				use oct::encode::SizedEncode;

				const ITEM_SIZE: usize = u8::MAX_ENCODED_SIZE;

				let buf: Box<[_]> = generate_random_data::<u8>(ITEM_SIZE, VALUE_COUNT).collect();
				let mut stream = Input::new(&buf);

				for _ in 0x0..VALUE_COUNT {
					let _ = u8::decode(&mut stream).unwrap();
				}
			}

			postcard: {
				const ITEM_SIZE: usize = size_of::<u8>();

				let buf: Box<[_]> = generate_random_data::<u8>(ITEM_SIZE, VALUE_COUNT).collect();

				for i in 0x0..VALUE_COUNT {
					let data = array::from_ref(&buf[i]).as_slice();

					let _: u8 = postcard::from_bytes(data).unwrap();
				}
			}
		}

		decode_non_zero_u8: {
			bincode: {
				use bincode::decode_from_slice;
				use bincode::config;

				const ITEM_SIZE: usize = size_of::<NonZero<u8>>();

				let buf: Box<[_]> = generate_random_data::<NonZero<u8>>(ITEM_SIZE, VALUE_COUNT).collect();

				for i in 0x0..VALUE_COUNT {
					let data = array::from_ref(&buf[i]).as_slice();

					let _: (NonZero<u8>, usize) = decode_from_slice(data, config::standard()).unwrap();
				}
			}

			borsh: {
				const ITEM_SIZE: usize = size_of::<NonZero<u8>>();

				let buf: Box<[_]> = generate_random_data::<NonZero<u8>>(ITEM_SIZE, VALUE_COUNT).collect();

				for i in 0x0..VALUE_COUNT {
					let data = array::from_ref(&buf[i]).as_slice();

					let _: NonZero<u8> = borsh::from_slice(data).unwrap();
				}
			}

			oct: {
				use oct::decode::{Decode, Input};
				use oct::encode::SizedEncode;

				const ITEM_SIZE: usize = NonZero::<u8>::MAX_ENCODED_SIZE;

				let buf: Box<[_]> = generate_random_data::<NonZero<u8>>(ITEM_SIZE, VALUE_COUNT).collect();
				let mut stream = Input::new(&buf);

				for _ in 0x0..VALUE_COUNT {
					let _ = NonZero::<u8>::decode(&mut stream).unwrap();
				}
			}

			postcard: {
				const ITEM_SIZE: usize = size_of::<NonZero<u8>>();

				let buf: Box<[_]> = generate_random_data::<NonZero<u8>>(ITEM_SIZE, VALUE_COUNT).collect();

				for i in 0x0..VALUE_COUNT {
					let data = array::from_ref(&buf[i]).as_slice();

					let _: NonZero<u8> = postcard::from_bytes(data).unwrap();
				}
			}
		}

		decode_bool: {
			bincode: {
				use bincode::decode_from_slice;
				use bincode::config;

				const ITEM_SIZE: usize = size_of::<bool>();

				let buf: Box<[_]> = generate_random_data::<bool>(ITEM_SIZE, VALUE_COUNT).collect();

				for i in 0x0..VALUE_COUNT {
					let data = array::from_ref(&buf[i]).as_slice();

					let _: (bool, usize) = decode_from_slice(data, config::standard()).unwrap();
				}
			}

			borsh: {
				const ITEM_SIZE: usize = size_of::<bool>();

				let buf: Box<[_]> = generate_random_data::<bool>(ITEM_SIZE, VALUE_COUNT).collect();

				for i in 0x0..VALUE_COUNT {
					let data = array::from_ref(&buf[i]).as_slice();

					let _: bool = borsh::from_slice(data).unwrap();
				}
			}

			oct: {
				use oct::decode::{Decode, Input};
				use oct::encode::SizedEncode;

				const ITEM_SIZE: usize = bool::MAX_ENCODED_SIZE;

				let buf: Box<[_]> = generate_random_data::<bool>(ITEM_SIZE, VALUE_COUNT).collect();
				let mut stream = Input::new(&buf);

				for _ in 0x0..VALUE_COUNT {
					let Ok(_) = bool::decode(&mut stream);
				}
			}

			postcard: {
				const ITEM_SIZE: usize = size_of::<bool>();

				let buf: Box<[_]> = generate_random_data::<bool>(ITEM_SIZE, VALUE_COUNT).collect();

				for i in 0x0..VALUE_COUNT {
					let data = array::from_ref(&buf[i]).as_slice();

					let _: bool = postcard::from_bytes(data).unwrap();
				}
			}
		}
	};
}
