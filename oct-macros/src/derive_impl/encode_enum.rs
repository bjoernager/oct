// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

use crate::{Discriminants, Repr};

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
	DataEnum,
	Fields,
	Ident,
	LitInt,
	Type,
};

#[must_use]
pub fn encode_enum(data: DataEnum, repr: Repr, error: Type) -> TokenStream {
	let discriminants: Vec<LitInt> = Discriminants::new(&data.variants).collect();

	let captures: Vec<Vec<Ident>> = data
		.variants
		.iter()
		.map(|variant| {
			variant
				.fields
				.iter()
				.enumerate()
				.map(|(index, _)| Ident::new(&format!("value{index}"), Span::call_site()))
				.collect()
		})
		.collect();

	let patterns = data
		.variants
		.into_iter()
		.zip(&captures)
		.map(|(variant, captures)| {
			let variant_name = variant.ident;

			match variant.fields {
				Fields::Unit => quote! { Self::#variant_name },

				Fields::Unnamed(_fields) => quote! { Self::#variant_name (#(ref #captures,)*) },

				Fields::Named(fields) => {
					let field_names = fields
						.named
						.into_iter()
						.map(|field| field.ident.unwrap());

					quote! { Self::#variant_name { #(#field_names: ref #captures,)* } }
				},
			}
		});

	quote! {
		type Error = ::oct::error::EnumEncodeError<<#repr as ::oct::encode::Encode>::Error, #error>;

		#[expect(unreachable_patterns)]
		#[inline]
		fn encode(&self, output: &mut ::oct::encode::Output) -> ::core::result::Result<(), Self::Error> {
			match *self {
				#(
					#patterns => {
						<#repr as ::oct::encode::Encode>::encode(&#discriminants, output)
							.map_err(::oct::error::EnumEncodeError::BadDiscriminant)?;

						#(
							::oct::encode::Encode::encode(#captures, output)
								.map_err(::core::convert::Into::<#error>::into)
								.map_err(::oct::error::EnumEncodeError::BadField)?;
						)*
					}
				)*

				_ => ::core::unreachable!("no variants defined for this enumeration"),
			}

			::core::result::Result::Ok(())
		}
	}
}
