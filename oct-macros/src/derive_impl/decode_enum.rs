// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::{Discriminants, Repr};

use core::iter;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, Fields, Type};

#[must_use]
pub fn decode_enum(data: DataEnum, repr: Repr, error: Type) -> TokenStream {
	let discriminants: Vec<_> = Discriminants::new(&data.variants).collect();

	let values = data
		.variants
		.into_iter()
		.map(|variant| {
			let variant_name = variant.ident;

			let commands = iter::repeat_n(
				quote! {
					::oct::decode::Decode::decode(stream)
						.map_err(::core::convert::Into::<#error>::into)
						.map_err(::oct::error::EnumDecodeError::BadField)?
				},
				variant.fields.len(),
			);

			match variant.fields {
				Fields::Unit => quote! { Self::#variant_name },

				Fields::Unnamed(_fields) => quote! { Self::#variant_name (#(#commands, )*) },

				Fields::Named(fields) => {
					let field_names = fields
						.named
						.into_iter()
						.map(|field| field.ident.unwrap());

					quote! { Self::#variant_name { #(#field_names: #commands, )* } }
				},
			}
		});

	quote! {
		type Error = ::oct::error::EnumDecodeError<#repr, <#repr as ::oct::decode::Decode>::Error, #error>;

		#[inline]
		fn decode(stream: &mut ::oct::decode::Input) -> ::core::result::Result<Self, Self::Error> {
			use ::core::result::Result;

			let discriminant = <#repr as ::oct::decode::Decode>::decode(stream)
				.map_err(::core::convert::Into::<::core::convert::Infallible>::into)
				.map_err(::oct::error::EnumDecodeError::InvalidDiscriminant)?;

			let this = match discriminant {
				#(#discriminants => #values,)*

				value => {
					::oct::__cold_path();

					return ::core::result::Result::Err(::oct::error::EnumDecodeError::UnassignedDiscriminant(value))
				},
			};

			::core::result::Result::Ok(this)
		}
	}
}
