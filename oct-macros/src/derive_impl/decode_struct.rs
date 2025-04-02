// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataStruct, Fields};

#[must_use]
pub fn decode_struct(data: DataStruct) -> TokenStream {
	let commands = data
		.fields
		.iter()
		.map(|field| {
			let slot = field.ident.as_ref().map(|name| {
				quote! { #name: }
			});

			quote! {
				#slot {
					::oct::decode::Decode::decode(input)
						.map_err(::core::convert::Into::<::oct::error::GenericDecodeError>::into)?
				},
			}
		});

	let value = match data.fields {
		Fields::Unit       => quote! { Self },
		Fields::Unnamed(_) => quote! { Self(#(#commands)*) },
		Fields::Named(_)   => quote! { Self { #(#commands)* } },
	};

	quote! {
		type Error = ::oct::error::GenericDecodeError;

		#[inline]
		fn decode(input: &mut ::oct::decode::Input) -> ::core::result::Result<Self, Self::Error> {
			let this = #value;
			::core::result::Result::Ok(this)
		}
	}
}
