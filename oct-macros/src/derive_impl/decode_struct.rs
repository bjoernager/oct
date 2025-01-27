// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use core::iter;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataStruct, Fields};

#[must_use]
pub fn decode_struct(data: DataStruct) -> TokenStream {
	let commands = iter::repeat_n(
		quote! {
			::oct::decode::Decode::decode(stream)
				.map_err(::core::convert::Into::<::oct::error::GenericDecodeError>::into)?
		},
		data.fields.len(),
	);

	let value = match data.fields {
		Fields::Unit => quote! { Self },

		Fields::Unnamed(_fields) => quote! { Self (#(#commands, )*) },

		Fields::Named(fields) => {
			let field_names = fields
				.named
				.into_iter()
				.map(|field| field.ident.unwrap());

			quote! { Self { #(#field_names: #commands, )* } }
		},
	};

	quote! {
		type Error = ::oct::error::GenericDecodeError;

		#[inline]
		fn decode(stream: &mut ::oct::decode::Input) -> ::core::result::Result<Self, Self::Error> {
			let this = #value;
			::core::result::Result::Ok(this)
		}
	}
}
