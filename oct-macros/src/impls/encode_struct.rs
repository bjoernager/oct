// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{DataStruct, Fields, Ident};

#[must_use]
pub fn encode_struct(data: DataStruct) -> TokenStream {
	let captures: Vec<_> = data
		.fields
		.iter()
		.enumerate()
		.map(|(index, _)| Ident::new(&format!("value{index}"), Span::call_site()))
		.collect();

	let pattern = match data.fields {
		Fields::Unit => quote! { Self },

		Fields::Unnamed(_fields) => quote! { Self(#(ref #captures, )*) },

		Fields::Named(fields) => {
			let field_names = fields
				.named
				.into_iter()
				.map(|field| field.ident.unwrap());

			quote! { Self { #(#field_names: ref #captures, )* } }
		},
	};

	quote! {
		type Error = ::oct::error::GenericEncodeError;

		#[inline]
		fn encode(&self, stream: &mut ::oct::encode::Output) -> ::core::result::Result<(), Self::Error> {
			let #pattern = *self;

			#(
				::oct::encode::Encode::encode(#captures, stream)
					.map_err(::core::convert::Into::<::oct::error::GenericEncodeError>::into)?;
			)*

			::core::result::Result::Ok(())
		}
	}
}
