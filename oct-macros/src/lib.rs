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

macro_rules! use_mod {
	($vis:vis $name:ident) => {
		mod $name;
		$vis use $name::*;
	};
}
pub(crate) use use_mod;

use_mod!(discriminants);
use_mod!(generic_name);
use_mod!(impl_derive_macro);
use_mod!(repr);

mod impls;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse2};

#[proc_macro_derive(Decode)]
pub fn derive_decode(input: TokenStream) -> TokenStream {
	let input = syn::parse_macro_input!(input as DeriveInput);

	let output = impl_derive_macro(
		input,
		parse2(quote! { ::oct::decode::Decode }).unwrap(),
		None,
		impls::decode_struct,
		impls::decode_enum,
	);

	//panic!("{output}");

	output.into()
}

#[proc_macro_derive(Encode)]
pub fn derive_encode(input: TokenStream) -> TokenStream {
	let input = syn::parse_macro_input!(input as DeriveInput);

	let output = impl_derive_macro(
		input,
		parse2(quote! { ::oct::encode::Encode }).unwrap(),
		None,
		impls::encode_struct,
		impls::encode_enum,
	);

	//panic!("{output}");

	output.into()
}

#[proc_macro_derive(SizedEncode)]
pub fn derive_sized_encode(input: TokenStream) -> TokenStream {
	let input = syn::parse_macro_input!(input as DeriveInput);

	let output = impl_derive_macro(
		input,
		parse2(quote! { ::oct::encode::SizedEncode }).unwrap(),
		None,
		impls::sized_encode_struct,
		impls::sized_encode_enum,
	);

	//panic!("{output}");

	output.into()
}
