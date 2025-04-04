// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataStruct, Index, Type};

#[must_use]
pub fn encode_struct(data: DataStruct, error: Type) -> TokenStream {
	let commands = data
		.fields
		.iter()
		.enumerate()
		.map(|(index, field)| {
			let name = field.ident.as_ref().map_or_else(
				|| {
					let index = Index::from(index);
					quote! { #index }
				},

				|name| quote! { #name },
			);

			quote! {
				::oct::encode::Encode::encode(&self.#name, output)
					.map_err(::core::convert::Into::<#error>::into)?;
			}
		});

	quote! {
		type Error = #error;

		#[inline]
		fn encode(&self, output: &mut ::oct::encode::Output) -> ::core::result::Result<(), Self::Error> {
			#(#commands)*

			::core::result::Result::Ok(())
		}
	}
}
