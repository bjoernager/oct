// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use proc_macro2::TokenStream;
use quote::quote;
use syn::DataStruct;

#[must_use]
pub fn sized_encode_struct(data: DataStruct) -> TokenStream {
	let tys: Vec<_> = data.fields
		.into_iter()
		.map(|field| field.ty)
		.collect();

	quote! {
		const MAX_ENCODED_SIZE: usize = 0x0 #( + <#tys as ::oct::encode::SizedEncode>::MAX_ENCODED_SIZE)*;
	}
}
