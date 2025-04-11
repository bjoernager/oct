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

use crate::Benchmark;

pub struct Suite {
	value_count: u32,
	run_count:   u32,

	benchmarks: Vec<Benchmark>,
}

impl Suite {
	#[inline]
	#[must_use]
	pub fn new(value_count: u32, run_count: u32) -> Self {
		assert!(value_count != 0x0);
		assert!(run_count   != 0x0);

		Self {
			value_count,
			run_count,

			benchmarks: Default::default(),
		}
	}

	#[inline]
	#[track_caller]
	pub fn push_benchmark(&mut self, benchmark: Benchmark) {
		self.benchmarks.push(benchmark);
	}

	pub fn run(self) {
		let benchmark_count = self.benchmarks.len();

		eprintln!("running `{benchmark_count}` benchmark(s)");
		eprintln!();

		for benchmark in self.benchmarks {
			benchmark.run(self.value_count, self.run_count);

			eprintln!();
		}
	}
}
