// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use core::borrow::Borrow;
use proc_macro2::Span;
use syn::{Expr, Lit, LitInt, Variant};

pub struct Discriminants<I: IntoIterator<Item: Borrow<Variant>>> {
	variants: I::IntoIter,
	prev: Option<u128>,
}

impl<I: IntoIterator<Item: Borrow<Variant>>> Discriminants<I> {
	#[inline(always)]
	#[must_use]
	pub fn new(variants: I) -> Self {
		Self {
			variants: variants.into_iter(),
			prev: None,
		}
	}
}

impl<I: IntoIterator<Item: Borrow<Variant>>> Iterator for Discriminants<I> {
	type Item = LitInt;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		let variant = self.variants.next()?;

		let discriminant = if let Some((_, ref expr)) = variant.borrow().discriminant {
			let Expr::Lit(ref expr) = *expr else {
				panic!("expected literal expression for discriminant value");
			};

			let Lit::Int(ref expr) = expr.lit else {
				panic!("expected (potentially signed) integer literal for discriminant value`");
			};

			let expr = expr.base10_digits();

			let value: u128 = expr
				.parse()
				.or_else(|_| expr.parse::<i128>().map(|v| v as u128))
				.unwrap();

			value
		} else if let Some(prev) = self.prev {
			prev
				.checked_add(0x1)
				.unwrap_or_else(|| panic!("overflow following discriminant `{prev}`"))
		} else {
			Default::default()
		};

		self.prev = Some(discriminant);

		let discriminant = LitInt::new(&discriminant.to_string(), Span::call_site());

		Some(discriminant)
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.variants.size_hint()
	}
}
