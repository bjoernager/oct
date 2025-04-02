// Copyright 2024-2025 Gabriel Bjørnager Jensen.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, you
// can obtain one at:
// <https://mozilla.org/MPL/2.0/>.

#![doc(html_logo_url = "https://gitlab.com/bjoernager/oct/-/raw/master/doc-icon.svg")]

//! This crate implements procedural macros for [`oct`](https://crates.io/crates/oct/).

// For use in macros:
extern crate self as oct_macros;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

mod derive_impl;

mod discriminants;
mod repr;

use discriminants::Discriminants;
use repr::Repr;

#[proc_macro_derive(Decode)]
pub fn derive_decode(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let self_name = input.ident;

	let body = match input.data {
		Data::Struct(data) => derive_impl::decode_struct(data),

		Data::Enum(data) => {
			let repr = Repr::get(&input.attrs).unwrap_or_default();

			derive_impl::decode_enum(data, repr)
		}

		Data::Union(_) => panic!("untagged union `{self_name}` cannot derive `oct::decode::Decode`"),
	};

	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	let output = quote! {
		impl #impl_generics ::oct::decode::Decode for #self_name #ty_generics
		#where_clause
		{
			#body
		}
	};

	output.into()
}

#[proc_macro_derive(Encode)]
pub fn derive_encode(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let self_name = input.ident;

	let body = match input.data {
		Data::Struct(data) => derive_impl::encode_struct(data),

		Data::Enum(data) => {
			let repr = Repr::get(&input.attrs).unwrap_or_default();

			derive_impl::encode_enum(data, repr)
		}

		Data::Union(_) => panic!("untagged union `{self_name}` cannot derive `oct::encode::Encode`"),
	};

	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	let output = quote! {
		impl #impl_generics ::oct::encode::Encode for #self_name #ty_generics
		#where_clause
		{
			#body
		}
	};

	output.into()
}

#[proc_macro_derive(SizedEncode)]
pub fn derive_sized_encode(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let self_name = input.ident;

	let body = match input.data {
		Data::Struct(data) => derive_impl::sized_encode_struct(data),

		Data::Enum(data) => {
			let repr = Repr::get(&input.attrs).unwrap_or_default();

			derive_impl::sized_encode_enum(data, repr)
		}

		Data::Union(_) => panic!("untagged union `{self_name}` cannot derive `oct::encode::SizedEncode`"),
	};

	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	let output = quote! {
		impl #impl_generics ::oct::encode::SizedEncode for #self_name #ty_generics
		#where_clause
		{
			#body
		}
	};

	output.into()
}
