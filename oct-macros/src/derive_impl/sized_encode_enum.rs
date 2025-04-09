// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::Repr;

use core::iter;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::DataEnum;

#[must_use]
pub fn sized_encode_enum(data: DataEnum, repr: Repr) -> TokenStream {
	let tys: Vec<Vec<_>> = data
		.variants
		.iter()
		.map(|variant| {
			variant
				.fields
				.iter()
				.map(|field| field.ty.clone())
				.chain(iter::once(repr.to_type(Span::call_site())))
				.collect()
		})
		.collect();

	quote! {
		const MAX_ENCODED_SIZE: usize = ::oct::enum_encoded_size!(#((#(#tys,)*),)*);
	}
}
